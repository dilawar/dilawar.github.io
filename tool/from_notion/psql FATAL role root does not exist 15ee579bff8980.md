---
title: psql FATAL role root does not exist 15ee579bff898003b850d4068aae120a
updated: 2026-01-14 22:54:14Z
created: 2026-01-15 04:51:09Z
---

# psql: FATAL:  role "root" does not exist.

Published On: December 16, 2024
Author: dilawar
Status: Published
Type: Note
Tags: postgres

I was using `healthcheck` in postgres container which triggered this ominous error.

 I made the following changed and the problem went away (for obvious reason, I guess) — `pg_isready` needs to know which user is being problem.

Following is the diff.

```bash
index 4c91dcc..89ad702 100644
--- a/compose.yaml
+++ b/compose.yaml
@@ -6,8 +6,9 @@ services:
     shm_size: 512mb
     environment:
       POSTGRES_PASSWORD: test_postgres_123
+      POSTGRES_USER: test_postgres_user
     healthcheck:
-      test: ['CMD-SHELL', 'pg_isready' ]
+      test: "pg_isready -U $$POSTGRES_USER"
       interval: 1s
       timeout: 5s
       retries: 10

```

Thanks https://stackoverflow.com/questions/60193781/postgres-with-docker-compose-gives-fatal-role-root-does-not-exist-error