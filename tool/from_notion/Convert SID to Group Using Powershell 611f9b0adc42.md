---
title: Convert SID to Group Using Powershell 611f9b0adc424e80a96bbfe2d7ac86e2
updated: 2026-01-14 22:55:24Z
created: 2026-01-15 04:51:09Z
---

# Convert SID to Group Using Powershell

Published On: August 11, 2024
Author: dilawar
Status: Published
Type: Note
Tags: windows

```powershell
        
PS C:\Windows\system32> $SID = New-Object System.Security.Principal.SecurityIdentifier("S-1-5-21-3331414065-2088522717-2563937559-513")
PS C:\Windows\system32> $User = $SID.Translate([System.Security.Principal.NTAccount])
PS C:\Windows\system32> $User.Value
ookie\None
```