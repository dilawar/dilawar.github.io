---
title: self-hosted ntfy service with admin account 185e579bff898084a421defef72edeb9
updated: 2026-01-14 22:54:14Z
created: 2026-01-15 04:51:09Z
---

# self-hosted ntfy service with admin account

Published On: January 24, 2025
Author: dilawar
Status: Published
Type: Note
Tags: self-hosted, tools

- First, we need a script that setup admin account for us after starting the service. [`init.sh`](http://init.sh) file that you should put inside `ntfy/init.sh`
    
    ```bash
    #!/bin/sh
    ntfy serve &
    sleep 2
    NTFY_PASSWORD=${NTFY_PASSWORD} ntfy user add --role=admin ${NTFY_USER}
    wait
    ```
    
- Now the compose file `compose.yaml` or `docker-compose.yaml` file.
    
    ```yaml
    services:
      ntfy:
        image: binwiederhier/ntfy
        container_name: ntfy
        entrypoint: [ "/bin/sh", "/var/lib/ntfy/init.sh" ]
        environment:
          - TZ=UTC    # optional: set desired timezone
          - NTFY_BASE_URL=https://ntfy.dilawars.me
          - NTFY_AUTH_FILE=/var/lib/ntfy/auth.db
          - NTFY_CACHE_FILE=/var/lib/ntfy/cache.db
          - NTFY_AUTH_DEFAULT_ACCESS=deny-all
          - NTFY_ATTACHMENT_CACHE_DIR=/var/lib/ntfy/attachments
          - NTFY_ENABLE_LOGIN=true
          - NTFY_USER=admin
          - NTFY_PASSWORD=yoyodillusingh
        volumes:
          - /var/cache/ntfy:/var/cache/ntfy
          - /etc/ntfy:/etc/ntfy
          - ./ntfy/:/var/lib/ntfy
        ports:
          - 8001:80
        healthcheck: # optional: remember to adapt the host:port to your environment
            test: ["CMD-SHELL", "wget -q --tries=1 http://localhost:80/v1/health -O - | grep -Eo '\"healthy\"\\s*:\\s*true' || exit 1"]
            interval: 60s
            timeout: 10s
            retries: 3
            start_period: 40s
        restart: unless-stopped
    ```