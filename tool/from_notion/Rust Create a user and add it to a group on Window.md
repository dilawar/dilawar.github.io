---
title: >-
  Rust Create a user and add it to a group on Window
  dd8b15f1c5794e339109cddfc70daaef
updated: 2026-01-14 22:58:28Z
created: 2026-01-15 04:51:09Z
---

# Rust: Create a user and add it to a group on Windows

Published On: July 16, 2024
Author: dilawar
Status: Published
Type: Note
Tags: note, rust

<aside>
💡 The following snippet may be incomplete since I copied and pasted it from a large project. I should have created a snippet!

</aside>

I’d recommend this excellent resource https://github.com/secur30nly/netuser-rs. It is a rust-based cli/library to manage Windows users and groups.

My solution is built on top of it but is slightly different: I used `USER_INFO_2` instead of `USER_INFO_1`, which allows for the full name of the user, and `LOCALGROUP_MEMBER_INFO_3`, instead of `LOCALGROUP_MEMBER_INFO_1` which is a simpler API.

```rust
#[cfg(windows)]
fn create_account_win32<S: AsRef<str> + std::fmt::Display>(
    user_name: S,
    full_name: S,
    password: S,
    group: &str
) -> anyhow::Result<()> {
    use windows::Win32::NetworkManagement::NetManagement::{
        NetUserAdd,
        UF_NORMAL_ACCOUNT,
        UF_SCRIPT,
        USER_ACCOUNT_FLAGS,
        USER_INFO_2,
        USER_PRIV_USER,
    };

    // use level 2 i.e. USER_INFO_2 in NetUserAdd
    // https://learn.microsoft.com/en-us/windows/win32/api/lmaccess/ns-lmaccess-user_info_2
    let mut user = USER_INFO_2::default();

    let (user_name_w, _v1) = str_to_pwstr(user_name.as_ref());
    user.usri2_name = user_name_w;

    let (password_w, _v2) = str_to_pwstr(password.as_ref());
    user.usri2_password = password_w;

    let (full_name_w, _v3) = str_to_pwstr(full_name.as_ref());
    user.usri2_full_name = full_name_w;

    user.usri2_priv = USER_PRIV_USER;
    user.usri2_flags = UF_SCRIPT | USER_ACCOUNT_FLAGS(UF_NORMAL_ACCOUNT);
    user.usri2_acct_expires = u32::MAX;

    tracing::debug!("Creating user {user:?}");
    let status: u32 = unsafe { NetUserAdd(None, 2, &user as *const _ as _, None) };
    if status == 0 {
        println!("✓ Successfully created account.");
    } else {
        eprintln!("⛌  Failed to create account!");
    }

    // Now add user to a localgroup. This group must already exist in the system.
    manage_group_users_win32(user_name.as_ref(), group, false)?;

    Ok(())
}

/// Add user to a specified local group.
/// If the `deletion` param is `true` - delete the specified user account from the local
/// group.
#[cfg(windows)]
unsafe fn manage_group_users_win32(
    username: &str,
    groupname: &str,
    deletion: bool
) -> anyhow::Result<()> {
    use windows::Win32::NetworkManagement::NetManagement::{
        NetLocalGroupAddMembers,
        NetLocalGroupDelMembers,
        LOCALGROUP_MEMBERS_INFO_3,
    };

    let (wide_groupname_nul, _buf) = str_to_pwstr(groupname);
    let (wide_username_nul, _buf) = str_to_pwstr(username);
    tracing::debug!("Mangaing groups of user {username}: group={groupname} , delete={deletion}.");

    let group_members = LOCALGROUP_MEMBERS_INFO_3 {
        lgrmi3_domainandname: wide_username_nul,
    };

    let rc = if deletion {
        NetLocalGroupDelMembers(
            None,
            wide_groupname_nul,
            3,
            &group_members as *const _ as *const u8,
            1
        )
    } else {
        NetLocalGroupAddMembers(
            None,
            wide_groupname_nul,
            3,
            &group_members as *const _ as *const u8,
            1
        )
    };

    if rc != 0 {
        anyhow::bail!("Failed to manage user group. Error {rc}.");
    }

    Ok(())
}

/// Convert &str into PWSTR (null terminated).
pub fn str_to_pwstr(s: &str) -> (PWSTR, Vec<u16>) {
    let mut encoded = s.encode_utf16().chain([0u16]).collect::<Vec<u16>>();
    (PWSTR(encoded.as_mut_ptr()), encoded)
}
```