---
title: >-
  BackupOnDelete — A windows kernel minifilter and u
  2c2e579bff89802497dfe44c6b77e744
updated: 2026-01-14 22:55:08Z
created: 2026-01-15 04:51:09Z
---

# BackupOnDelete — A windows kernel minifilter and user-space app

Published On: December 7, 2025
Author: dilawar
Status: Published
Type: Note
Tags: lang:cxx

# What is it?

When a file is deleted by any means possible, you want to ensure a backup of the file before its get permanently deleted. This post describes a windows minifilter and a user-space app working in tandem with minifilter to implement a working solution. 

It is *not* a tutorial since I am not great at writing. However, you should browse the code and check if the patterns I’ve used make sense to you. You may also like to have a look at the build system, packaging (nsis based installer) and application code which is much larger than the minifilter. Later I rewrote the application in Rust but that is not part of this repository.

<aside>
💡

Repository https://github.com/dilawar/minifilter-backup-before-delete/tree/main. This minifilter is a personal project. Do not use it in production.

</aside>

### Architectural summary

[](https://kroki.io/plantuml/svg/eNp1kEEKwjAQRfc9xT-ALoq7LqSCdCOuxAOkyRdD0xiSiee3wWKL6GYW_78ZHtMmUVHy6KprYsR2jxOjp2twpKMQnXXTqN5p6c_W25t1wthAqyA5MsHMMPikl2ph1hcjvRoLJA90NZQ3uFszBfXXQlE5hNCgV3rIoQBz9LstBy879FkPlDW6di00DXLYfGzrP6JL39Kb6TkvnxVhrg==)

- The minifilter monitors certain events in filesystem that says ‘delete this file’ (on close!).
- Instead of letting kernel delete the file, the minifilter ask the kernel to rename the file (move it to a temporary space) and hide it. For example, if file `D:\MyDocuments\important.pdf` is being deleted, it will be moved to `C:\ProgramData\Minifilter\D:^^MyDocument^^__??important.pdf??__` and then marked hidden. Minifilter also ensures that new file is not renamed by anyone.
    - Original filepath is converted to a special valid filename (filename!) e.g. `D:\MyDocuments\important.pdf` to `D__^^MyDocument^^??important.pdf??` . As long as you can recover the original filepath from new filename unambigously, we are good to go. You can also use base64 encoded paths as filename. I used  a simpler scheme since decoding base64 encoded path inside a kernel minifilter was quite a lot of work.
- The new path is sent to user-space application which backs it in a bucket and then send a message a kernel minifilter to delete the file.
- When delete this file request come from the user-space application, the minifilter doesn’t modify the delete-file event and let is go down the kernel stack. The kernel delete the file for sure this time.

## Minifilter

This kernel minifilter is a cmake based project that also create a nsis based installer and sign it using your personal key. Note that you need to get the minifilter signed by MS or MS approved vendors before you can publish it. To load the minifilter into your machine, you should follow the official guide https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/development-and-testing-tools

- The minifilter captures required filesystem events.
    
    ```cpp
    CONST FLT_OPERATION_REGISTRATION Callbacks[] = {
    
        { IRP_MJ_CREATE, 0, BackupOnDeleteShield, NULL },
    
        // IRP_MJ_SET_INFORMATION is sent whenever file is marked for deletion.
        { IRP_MJ_SET_INFORMATION,
          FLTFL_OPERATION_REGISTRATION_SKIP_PAGING_IO,
          BackupOnDeleteShield,
          NULL },
    
        { IRP_MJ_OPERATION_END }
    
    };
    ```
    
- See https://github.com/dilawar/minifilter-backup-before-delete/blob/e61e9c5fbfd9df51749d0de98acfbad7b7a6290b/minifilter/src/main.cpp#L396 for the definition of `BackupOnDeleteShield` function.
    - If the delete request has come from kenel mode application, let is pass. And if the user-space app is not connected with minifilter (more on it later), don’t do anything.
        
        ```cpp
          if (Data->RequestorMode == KernelMode) {
                // if the requestor is kernel mode, pass the request on uninterrupted
                ret = FLT_PREOP_SUCCESS_NO_CALLBACK;
                goto CleanUp;
            }
        
            //
            // If no client is connected to create backups then there is no point using
            // the shield.
            //
            if (!IsUserAppConnected()) {
                DFLOG(ERROR, "Shield>" __FUNCTION__ ": No backup client connected.\n");
                ret = FLT_PREOP_SUCCESS_NO_CALLBACK;
                goto CleanUp;
            }
        ```
        
- For an user-space app to talk to minifilter, the minifilter must create a channel to listen to. The port is opened by minifilter https://github.com/dilawar/minifilter-backup-before-delete/blob/e61e9c5fbfd9df51749d0de98acfbad7b7a6290b/minifilter/src/communication.cpp#L185. The channel is created using https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/fltkernel/nf-fltkernel-fltcreatecommunicationport
    
    ```cpp
    
    PORT_STATE
    InitializeServerPort(IN PUNICODE_STRING CommunicationPortName,
                         IN PFLT_FILTER Filter)
    {
        NTSTATUS status = STATUS_SUCCESS;
    
        gServerPortState = PORT_STATE::UnInitialized;
    
        PSECURITY_DESCRIPTOR SecurityDescriptor;
        OBJECT_ATTRIBUTES ObjectAttributes;
    
        PAGED_CODE();
    
        DFLOG(ERROR,
              "Shield>" __FUNCTION__ ": Trying opening port: '%wZ'.\n",
              CommunicationPortName);
    
        //
        // Create communication descriptor.
        //
        status = FltBuildDefaultSecurityDescriptor(&SecurityDescriptor,
                                                   FLT_PORT_ALL_ACCESS);
    
        if (!NT_SUCCESS(status)) {
            DFLOG(
              ERROR,
              "Shield>" __FUNCTION__ " : Port is not initialized. Error "
                                     "FltBuildDefaultSecurityDescriptor - %X.\n",
              status);
            gServerPortState = PORT_STATE::UnInitialized;
            goto CleanUp;
        }
    
        InitializeObjectAttributes(&ObjectAttributes,
                                   CommunicationPortName,
                                   OBJ_KERNEL_HANDLE | OBJ_CASE_INSENSITIVE,
                                   NULL,
                                   SecurityDescriptor);
    
        status = FltCreateCommunicationPort(
          Filter,
          &gServerPort,
          &ObjectAttributes,
          NULL,
          ShieldConnect,    // Connect notify callback
          ShieldDisconnect, // Disconnect notify callback.
          ShieldMessage,    // message notify callback.
          MAX_CLIENTS);
    
        if (!NT_SUCCESS(status)) {
            DFLOG(ERROR,
                  "Shield>" __FUNCTION__ " : Port is not initialized. Error "
                                         "FltCreateCommunicationPort - %X.\n",
                  status);
            gServerPortState = PORT_STATE::UnInitialized;
            goto CleanUp;
        }
    
        DFLOG(ERROR,
              "Shield>" __FUNCTION__ ": opened server port handle 0x%p.\n",
              gServerPort);
    
        gServerPortState = PORT_STATE::Initialized;
    
    CleanUp:
        if (SecurityDescriptor)
            FltFreeSecurityDescriptor(SecurityDescriptor);
    
        return gServerPortState;
    }
    
    ```
    

## User-space app

How to connect a user-space app to communicate with minifilter? We opened a port with a name. Use the same name to connect to the port (minifilter). See the `app` directory in the repository.

```cpp
void
ShieldClient::connect()
{
    port_ = INVALID_HANDLE_VALUE;
    HRESULT hr = S_OK;

#ifdef COMMUNICATION_IN_SYNC_MODE
    //
    // port in sync mode. We don't have to use Overlapped structure here.
    // TODO: Not sure about the performance.
    //
    PLOGI << "Connecting to " << portname_ << " in SYNC mode.";
    hr = FilterConnectCommunicationPort(
      portname_.c_str(), FLT_PORT_FLAG_SYNC_HANDLE, NULL, 0, NULL, &port_);
#else
    // port in async mode. This is the preferred way. Use with completion port.
    PLOGI << "Connecting to " << portname_ << " in ASYNC mode.";
    hr = FilterConnectCommunicationPort(
      portname_.c_str(), 0, NULL, 0, NULL, &port_);
#endif

    if (FAILED(hr)) {
        PLOGW << "Failed to connect to Shield: Error: 0x" << std::hex << hr;
        connected_ = false;
        goto MAIN_EXIT;
    }

    completion_port_ =
      CreateIoCompletionPort(port_, NULL, 0, 2 /* 2 user threads */);

    if (NULL == completion_port_) {
        hr = HRESULT_FROM_WIN32(GetLastError());
        PLOGW << "Error in creating completion port. Code: 0x" << std::hex
              << hr;
        goto MAIN_EXIT;
    }

    PLOGI << " ... Successfully connected.";
    connected_ = true;

MAIN_EXIT:
    if (!connected_ && INVALID_HANDLE_VALUE != port_) {
        CloseHandle(port_);
    }
}
```

## Messaging

To keep it simple,

- Each messages has first two bytes of it designated as message code.
- Next 4 bytes are the size of the message, and rest is the message.