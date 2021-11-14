---
title: Fixing git clone over https - Server certificate verification failed
author: Dilawar Singh
date: April 04, 2021
comments: true
---

As soon as I try to clone using `git clone
https://gitlab.subcom.tech/subcom/voicedb`, I run into the following problem:

```shell
admin@ip-172-26-13-71:~/Work$ git clone https://gitlab.subcom.tech/subcom/voicedb.git
Cloning into 'voicedb'...
fatal: unable to access 'https://gitlab.subcom.tech/subcom/voicedb.git/': server certificate verification failed. CAfile: none CRLfile: none
```
This is a big problem. The only protocol which works out of box is ssh. To use ssh protocol, everyone has to upload public key to the server. A bit of hassle if you are not working on your own machine :sad:.

First thing first, there is an issue with certificate. Lets debug this.

## Check for certificates

First, I'd like to know how certificates behaves on a working site.

### Base case: github.com certificates.

```shell
admin@ip-172-26-7-193:~$ openssl s_client -showcerts -servername github.com -connect github.com:443 </dev/null 2>/dev/null
CONNECTED(00000003)
---
Certificate chain
 0 s:C = US, ST = California, L = San Francisco, O = "GitHub, Inc.", CN = github.com
   i:C = US, O = "DigiCert, Inc.", CN = DigiCert High Assurance TLS Hybrid ECC SHA256 2020 CA1
-----BEGIN CERTIFICATE-----
... redacted ...
-----END CERTIFICATE-----
 1 s:C = US, O = "DigiCert, Inc.", CN = DigiCert High Assurance TLS Hybrid ECC SHA256 2020 CA1
   i:C = US, O = DigiCert Inc, OU = www.digicert.com, CN = DigiCert High Assurance EV Root CA
-----BEGIN CERTIFICATE-----
... redacted ...
-----END CERTIFICATE-----
---
Server certificate
subject=C = US, ST = California, L = San Francisco, O = "GitHub, Inc.", CN = github.com

issuer=C = US, O = "DigiCert, Inc.", CN = DigiCert High Assurance TLS Hybrid ECC SHA256 2020 CA1

---
No client certificate CA names sent
Peer signing digest: SHA256
Peer signature type: ECDSA
Server Temp Key: X25519, 253 bits
---
SSL handshake has read 2708 bytes and written 366 bytes
Verification: OK
---
New, TLSv1.3, Cipher is TLS_AES_128_GCM_SHA256
Server public key is 256 bit
Secure Renegotiation IS NOT supported
Compression: NONE
Expansion: NONE
No ALPN negotiated
Early data was not sent
Verify return code: 0 (ok)
---
```

Everything smoothly and now I know the expected output. I don't understand all
of it but I can make some sense out of it. Now let's try on my gitlab instance.

### gitlab.subcom.tech certificates


```shell
admin@ip-172-26-7-193:~$ openssl s_client -showcerts -servername gitlab.subcom.tech -connect gitlab.subcom.tech:443 </dev/null 2>/dev/null
CONNECTED(00000003)
---
Certificate chain
 0 s:CN = gitlab.subcom.tech
   i:C = US, O = Let's Encrypt, CN = R3
-----BEGIN CERTIFICATE-----
... redacted ...
-----END CERTIFICATE-----
---
Server certificate
subject=CN = gitlab.subcom.tech

issuer=C = US, O = Let's Encrypt, CN = R3

---
No client certificate CA names sent
Peer signing digest: SHA256
Peer signature type: RSA-PSS
Server Temp Key: X25519, 253 bits
---
SSL handshake has read 1884 bytes and written 390 bytes
Verification error: unable to verify the first certificate
---
New, TLSv1.3, Cipher is TLS_AES_256_GCM_SHA384
Server public key is 2048 bit
Secure Renegotiation IS NOT supported
Compression: NONE
Expansion: NONE
No ALPN negotiated
Early data was not sent
Verify return code: 21 (unable to verify the first certificate)
---
```

All right,  the problem seems to be that we are __unable to verify the first
certificate.__. Seems like there are multiple of certificates involved; and we
are unable to verify the first certificate itself!

To add to the confusion, the site works fine in browser
`https://gitlab.subcom.tech`. This means that certificates (I downloaded them
using `certbot`) are at least fine for browser. The git clone behaviour is
_definately_ different than the browser.

I did an internet sortie. Some useful pointers:
https://docs.microsoft.com/en-us/archive/blogs/phkelley/adding-a-corporate-or-self-signed-certificate-authority-to-git-exes-store
offers a solution. But still very confused.

I search in gitlab documentation. Their documentation is good and I got a
helpful hit:
__https://docs.gitlab.com/omnibus/settings/ssl.html#common-ssl-errors__. I had
to google few terms and read a bit more. For a person like me who has short
attention span, this is the hardest part. But after spending enough hours, I
finally figured out the solution.

# Solution

```terminal
admin@ip-172-26-7-193:/etc/gitlab/ssl$ ls /etc/gitlab/ssl/ -l
total 4
lrwxrwxrwx 1 root root   54 Apr  4 19:21 gitlab.subcom.tech.crt -> /etc/letsencrypt/live/gitlab.subcom.tech/cert.pem
lrwxrwxrwx 1 root root   52 Mar 12 15:25 gitlab.subcom.tech.key -> /etc/letsencrypt/live/gitlab.subcom.tech/privkey.pem
-r-------- 1 root root 1675 Mar 15 15:40 gitlab.subcom.tech.key-staging
```

Note that `gitlab.subcom.tech.crt` and `gitlab.subcom.tech.key` are links to
certificate and keys downloaded using `certbot`. The problem here is with
`cert.pem` which does not contain the "chain" of certificates but rather a
single certificate. This worked in browser but not using `git` (curious!).

After I realized that the chain of certificate is missing, I changed the
`gitlab.subcom.tech.crt` link.

```shell
admin@ip-172-26-7-193:/etc/gitlab/ssl$ ls /etc/gitlab/ssl/ -l
total 4
lrwxrwxrwx 1 root root   54 Apr  4 19:21 gitlab.subcom.tech.crt -> /etc/letsencrypt/live/gitlab.subcom.tech/fullchain.pem
lrwxrwxrwx 1 root root   52 Mar 12 15:25 gitlab.subcom.tech.key -> /etc/letsencrypt/live/gitlab.subcom.tech/privkey.pem
-r-------- 1 root root 1675 Mar 15 15:40 gitlab.subcom.tech.key-staging
```

Now the file (rather a link) `/etc/gitlab/ssl/gitlab.subcom.tech.crt` points to
`/etc/letsencrypt/live/gitlab.subcom.tech/fullchain.pem` rather than to
`/etc/letsencrypt/live/gitlab.subcom.tech/cert.pem`. This solved the issue for
me. `fullchain.pem` has all the certificates needed for verification to be
successful.


Finally, I did  a `gitlab-ctl reconfigure` followed by `gitlab.ctl restart`. I
can now clone the repositories over https protocol from 'gitlab.subcom.tech'
server. Phew!

