---
title: "Mail server info of National Center for Biological Sciences (NCBS) Bangalore"
date: "2013-09-04"
categories: 
  - "linux"
---

If you are a mutt user and working at NCBS Bangalore, you can setup your account in mutt as following. `set from="REAL_NAME " set from="USER NAME " set hostname="imaps://imap.ncbs.res.in" set folder="imaps://imap.ncbs.res.in" set record="+INBOX.Sent" set imap_user="NCBS_USER" set imap_pass="NCBS_PASS" set postponed="=Drafts" set spoolfile="+INBOX" set signature="sign.txt" set smtp_url="smtp://NCBS_USER@mail.ncbs.res.in" smtp_pass="$NCBS_PASS" set realname="$REAL_NAME"` In short incoming server is **imap.ncbs.res.in** and outgoing server is **mail.ncbs.res.in**. Use default ports. To get smtp server information, you can always use (in terminal) `msmtp -v --host=mail.ncbs.res.in --serverinfo --tls=on --tls-certcheck=off --port 25`

## Setup in BlueMail or other Android clients

For most clients, following settings should work.

### ![IMG-20170811-WA0004](images/img-20170811-wa0004.jpg)

![IMG-20170811-WA0003-1](images/img-20170811-wa0003-1.jpg)
