---
title: >-
  binwalk (and friends) is all you need to extract a
  141e579bff898014a3e0e65a51e273d8
updated: 2026-01-14 22:54:12Z
created: 2026-01-15 04:51:09Z
---

# binwalk (and friends) is all you need to extract arbitrary binary files

Published On: November 17, 2024
Author: dilawar
Status: Published
Type: Note
Tags: tools

Today, I wanted to look inside the firmware before uploading it to the router. I downloaded and unzipped it. It's a pretty big file for a firmware.

Inside the zip, there was a `.bin` file. The venerable `file` utility told me it that it is a hex file.

```bash
[dilawar@rasmalai keeda-rs (main)]$ file ax10v3-up-us-ver1-0-6-P1\[20240701-rel63845\]_nosign_2024-07-01_17.49.38.bin
../ax10v3-up-us-ver1-0-6-P1[20240701-rel63845]_nosign_2024-07-01_17.49.38.bin: data
```

The all-mighty https://github.com/google/magika is much-much better but still not very helpful.

```bash
$ magika ax10v3-up-us-ver1-0-6-P1\[20240701-rel63845\]_nosign_2024-07-01_17.49.38.bin
ax10v3-up-us-ver1-0-6-P1[20240701-rel63845]_nosign_2024-07-01_17.49.38.bin: ISO 9660 CD-ROM filesystem data (archive)
```

Then, I played with `hexdump` but not with much success. Too much tribal knowledge and bravery is needed to use `hexdump` or similar tool. What would a coward do?

I searched around a bit and found exactly was I was looking for. This blog post https://thunderysteak.github.io/tl-wa901nd-basic-re. explains exactly what I was looking for.

Great! Just use `binwalk`. And `binwalk` has been rewritten in Rust! 🦀. Double great!

I asked it to extract the file using `binwalk -e`. It extracted two *UBI Image* files.

```bash
[dilawar@rasmalai 034ff8a7811405e50d03c9fd06c29409b25243cd2b5bed35b1d0aafb2f793a26]$ binwalk -e ax10v3-up-us-ver1-0-6-P1\[20240701-rel63845\]_nosign_2024-07-01_17.49.38.bin    
/home/dilawar/.keeda/data/034ff8a7811405e50d03c9fd06c29409b25243cd2b5bed35b1d0aafb2f793a26/extractions/ax10v3-up-us-ver1-0-6-P1[20240701-rel63845]_nosign_2024-07-01_17.49.38.bin
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
DECIMAL                            HEXADECIMAL                        DESCRIPTION
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
4825                               0x12D9                             UBI image, version: 1, image size: 23330816 bytes
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
[+] Extraction of ubi data at offset 0x12D9 completed successfully
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Analyzed 1 file for 85 file signatures (187 magic patterns) in 120.0 milliseconds

[dilawar@rasmalai ubi_12D9.img]$ ls -la
total 21824
drwxr-xr-x 2 dilawar users       84 Nov 17 07:42 .
drwxr-xr-x 3 dilawar users       26 Nov 17 07:42 ..
-rw-r--r-- 1 dilawar users  3936256 Nov 17 07:42 **img-1957174073_vol-kernel.ubifs**
-rw-r--r-- 1 dilawar users 18411520 Nov 17 07:42 **img-1957174073_vol-rootfs.ubifs**

```

Now can I extract what is inside UBI file? Sure I can. ~~I wish there was a recursive extract option in binwalk.~~ Use `binwalk -Mve` to recursively extract files. It's super cool!

```bash
[dilawar@rasmalai ubi_12D9.img]$ binwalk -e img-1957174073_vol-kernel.ubifs

/home/dilawar/.keeda/data/034ff8a7811405e50d03c9fd06c29409b25243cd2b5bed35b1d0aafb2f793a26/extractions/ax10v3-up-us-ver1-0-6-P1[20240701-rel63845]_nosign_2024-07-01_17.49.38.bin.extracted/12D9/ubifs-root/ubi_12D9.img/extractions/img-1957174073_vol-kernel.ubifs
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
DECIMAL                            HEXADECIMAL                        DESCRIPTION
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
0                                  0x0                                uImage firmware image, header size: 64 bytes, data size: 3839011 bytes, compression: gzip, CPU: MIPS32,
                                                                      OS: Linux, image type: OS Kernel Image, load address: 0x80010000, entry point: 0x8063F8E0, creation time:
                                                                      2024-06-14 11:14:05, image name: "Linux-4.4.140"
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
[+] Extraction of uimage data at offset 0x0 completed successfully
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

[dilawar@rasmalai img-1957174073_vol-kernel.ubifs.extracted]$ cd 0/
[dilawar@rasmalai 0]$ ls -ltrha
total 3.7M
drwxr-xr-x 3 dilawar users   15 Nov 17 07:46 ..
-rw-r--r-- 1 dilawar users 3.7M Nov 17 07:46 **Linux-4.4.140.bin**
drwxr-xr-x 2 dilawar users   31 Nov 17 07:46 .

```

## Gotchas

### Missing helper tools

```bash
[2024-11-17T02:19:03Z ERROR binwalk::extractors::common] Failed to execute command vmlinux-to-elf["/home/dilawar/.keeda/data/034ff8a7811405e50d03c9fd06c29409b25243cd2b5bed35b1d0aafb2f793a26/extractions/ax10v3-up-us-ver1-0-6-P1[20240701-rel63845]_nosign_2024-07-01_17.49.38.bin.extracted/12D9/ubifs-root/ubi_12D9.img/extractions/img-1957174073_vol-kernel.ubifs.extracted/0/extractions/Linux-4.4.140.bin.extracted/0/extractions/decompressed.bin.extracted/0/linux_kernel_0.bin", "linux_kernel.elf"]: No such file or directory (os error 2)
[2024-11-17T02:19:03Z ERROR binwalk::extractors::common] Failed to spawn external extractor for 'linux_kernel' signature: No such file or directory (os error 2)
[2024-11-17T02:19:03Z ERROR binwalk::extractors::common] Failed to execute command dtc["-I", "dtb", "-O", "dts", "-o", "system.dtb", "/home/dilawar/.keeda/data/034ff8a7811405e50d03c9fd06c29409b25243cd2b5bed35b1d0aafb2f793a26/extractions/ax10v3-up-us-ver1-0-6-P1[20240701-rel63845]_nosign_2024-07-01_17.49.38.bin.extracted/12D9/ubifs-root/ubi_12D9.img/extractions/img-1957174073_vol-kernel.ubifs.extracted/0/extractions/Linux-4.4.140.bin.extracted/0/extractions/decompressed.bin.extracted/C3BDE0/dtb_C3BDE0.dtb"]: No such file or directory (os error 2)
[2024-11-17T02:19:03Z ERROR binwalk::extractors::common] Failed to spawn external extractor for 'dtb' signature: No such file or directory (os error 2)
[2024-11-17T02:19:03Z ERROR binwalk::extractors::common] Failed to execute command dtc["-I", "dtb", "-O", "dts", "-o", "system.dtb", "/home/dilawar/.keeda/data/034ff8a7811405e50d03c9fd06c29409b25243cd2b5bed35b1d0aafb2f793a26/extractions/ax10v3-up-us-ver1-0-6-P1[20240701-rel63845]_nosign_2024-07-01_17.49.38.bin.extracted/12D9/ubifs-root/ubi_12D9.img/extractions/img-1957174073_vol-kernel.ubifs.extracted/0/extractions/Linux-4.4.140.bin.extracted/0/extractions/decompressed.bin.extracted/C3BDE0/dtb_C3BDE0.dtb"]: No such file or directory (os error 2)
[2024-11-17T02:19:03Z ERROR binwalk::extractors::common] Failed to spawn external extractor for 'dtb' signature: No such file or directory (os error 2)
```

`binwalk` depends on other tools to do its bidding e.g., `dtc`and `vmlinux-to-elf` (https://github.com/marin-m/vmlinux-to-elf) You must ensure these are installed.  

- Run the following
    
    ```bash
    sudo apt install -y p7zip-full pipx
    pipx install git+https://github.com/sviehb/jefferson.git
    pipx install git+https://github.com/jrspruitt/ubi_reader
    pipx install git+https://github.com/marin-m/vmlinux-to-elf
    ```
    
- Also install https://github.com/dgibson/dtc. It's not available on Debian. You have to manually install it. It is available on multiple other OSes though [https://pkgs.org/search/?q=dtc&on=provides](https://pkgs.org/search/?q=dtc&on=provides).