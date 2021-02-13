---
title: "Using mutt - saving password and writing mail in markdown"
date: "2014-01-28"
categories: 
  - "utility"
tags: 
  - "markdown-with-mutt"
  - "mutt"
---

If I were to be forced to replace my mutt with some other general purpose email client I'd prefer thunderbird with plugin `muttator`.

If you prefer easy to setup and click-based tools then you will not find mutt very exciting, however those who loves to split hair and tweak their tools to their whims will find much in mutt to please them. Learning the full power of mutt takes time and some sort of persistence. This post is not intended for first time user. It is meant for those who have tried their hands on mutt. In the end, there are some link which can help who wants to start using mutt. I had the fleeting temptation to combine my scripts in one which can set up [mutt automatically](http://github.com/dilawar/mutt) for arbitrary no of accounts, but then it beats the whole purpose of tweaking the tool.

## Storing password

Mutt stores password in plain text format. To avoid this, I used `gpg` to encrypt a file in which password are stored. For example, a simple text file is created to store the password.

```
export EE_NAME="Dilawar Singh"
export EE_USER=dilawar
export EE_PASS=password1
```

This has export command which set an environment variable. Now I encrypt the file using `gpg`.

```
gpd -c password.txt
```

It will ask for a pass-phrase. When decrypting the file, you have provide the same pass-phrase. Passphrase are easy to remember and hard to crack. If I were Gabbar Singh, I would have chose something like `thakur a per katne chahiye the`. Hard to guess and easy to remember, isn't it? An encrypted file will be created by gpg default name `password.txt.gpg`. Mutt can read environment variables, let's exploit this fact. We write a wrapper around the mutt command. Let's name this script `my_mutt` or after the name of your pet doggie. And `chmod +x my_mutt` so it is executable. Store it somewhere like `/usr/local/bin`.

```
#!/bin/bash
pwds=gpg -d ~/.mutt/password.txt.gpg
eval $pwds
mutt -F ~/.mutt/mutt_server "$@"
```

Second line de-crypt the encrypted file, text of the file is stored in `pwds` variable (valid mutt configuration lines) and this text is executed by `eval` command. We invoke mutt in this shell so that it can access environment variables EE\_USER, EE\_PASS etc. On line 4, `-F ~/.mutt/mutt_server` says that this file is my config file which mutt should read. The default configuration file is `~/.muttrc`.

In the configuration file, write something like this. You need to write some more to setup mutt. This is only fragment which is relevant to one account.

```bash
# ~/Scripts/mutt/ee
set from="$EE_NAME $EE_USER@ee.iitb.ac.in"
set hostname="imaps://sandesh.ee.iitb.ac.in"
set folder="imaps://sandesh.ee.iitb.ac.in"
set imap_user="$EE_USER"
set imap_pass="$EE_PASS"
set record="+mail/Sent"
set postponed="=Drafts"
set spoolfile="imaps://sandesh.ee.iitb.ac.in/INBOX"
set signature="sign.txt"
set smtp_url="smtp://$GPO_USER@smtp-auth.iitb.ac.in:25"
set stmp_pass="$GPO_PASS"
set realname="$REAL_NAME"
my_hdr Organization: EE, IIT Bombay, http://www.ee.iitb.ac.in
```

In my personal opinion, this is a good solution to store password on a machine (don't commit your encrypted file to your Github/Gitlab repo.)

## Writing HTML emails

Mutt works great with text emails. But if you want to write emails for wncc group where code has to be written inline, html can help. See this question for potential solutions. [I have also written mine](http://unix.stackexchange.com/questions/108485/send-email-written-in-markdown-using-mutt).

## Resources:

1. Mutt wiki is a good reference guide.
2. [My first mutt](http://mutt.blackfish.org.uk/storage/)
3. [And this terminal lover blog](http://wcm1.web.rice.edu/mutt-tips.html)
