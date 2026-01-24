---
title: Use curl to upload files to S3 bucket 21ae579bff8980df9b0bcc9f87d4f942
updated: 2026-01-14 22:54:18Z
created: 2026-01-15 04:51:09Z
---

# Use curl to upload files to S3 bucket

Published On: June 22, 2025
Author: dilawar
Status: Published
Type: Article
Tags: note

First, export the relevant environment variables e.g. `S3_ACCESS_KEY`, `S3_SECRET_KEY`, `S3_REGION` 

To upload a file `foo.gz` to path `/backups/db/foo.gz` , use the following command.

```bash
curl --progress-bar \
          -X PUT \
          --user "${S3_ACCESS_KEY}":"${S3_SECRET_KEY}" \
          --aws-sigv4 "aws:amz:${S3_REGION}:s3" \
          --upload-file foo.gz \
          "https://${S3_BUCKET}.s3.${S3_REGION}.amazonaws.com/backups/db/foo.gz"
```

Note that you would a recent version (Feb 2021) of curl that natively supports aws v4 signature authentication (https://curl.se/docs/manpage.html#--aws-sigv4). for older version of curl, you may to send special headers (idk)!