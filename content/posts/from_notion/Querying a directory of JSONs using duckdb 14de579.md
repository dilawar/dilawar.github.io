---
title: Querying a directory of JSONs using duckdb 14de579bff898013bac0e8bceb97b0de
updated: 2026-01-14 22:54:14Z
created: 2026-01-15 04:51:09Z
---

# Querying a directory of JSONs using duckdb

Published On: November 30, 2024
Author: dilawar
Status: Published
Type: Note
Tags: data, db:duckdb

<aside>
💡

**TLDR**

Works great if all of your JSON files have same schema or all JSON files can be loaded into table at the same time using `read_json("**/*.json", union_by_name = True)` function.

</aside>

To make this problem concrete, I’ve downloaded database of all CVEs from https://github.com/CVEProject/cvelistV5 using `git clone`. I want to create API that can query this repository. I can think of the following available options.

1. `grep` based tool e.g. `git grep` , `rg` , `grep` etc.
2. https://jqlang.github.io/jq/ like tools.
3. Somehow ingest JSONs into a relational database and use `SQL` either using a third party tool or using a custom solution.

I’d have preferred 1 or 2 if I was writing a cli application. Since I am building a RESTful API, I’d prefer 3 since queries will be easier to write and integrate into other applications. Custom solution is also not a bad idea if existing tooling is not good enough or there are other constraints. Most databases like PostgreSQL and sqlite3 allows JSON to be inserted and queried.

While searching for such plugins, I came across https://duckdb.org/ which seems to support this use cases. See for example https://duckdb.org/docs/data/multiple_files/overview 💯.

For this exercise, I am not interested in performance. I can use caching to improve performance drastically later is need arise. I am looking for a solution that has the best DX — and if I am lucky doesn’t easily allow stupid mistakes!

### Installation

The installation was a breeze. Single binary! It can also be used as Rust/Python library and third party integration for PHP are also available 💯. 

```bash
$ wget https://github.com/duckdb/duckdb/releases/download/v1.1.3/duckdb_cli-linux-amd64.zip

$ unzip duckdb_cli-linux-amd64.zip 
Archive:  duckdb_cli-linux-amd64.zip
  inflating: duckdb                  
(PY311) [dilawar@rasmalai cve_database_git (master)]$ ./duckdb 
v1.1.3 19864453f7
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
D 
```

Great, seems to work!

### Querying JSON files

For a sanity check, I did a “loopback” — print what you read. Just to make sure that engine is parsing JSON files. I passed a glob that matches all JSON files. Loading all files together raised an error — mismatch in schema.

```bash
D SELECT * FROM 'cves/**/*.json';
Invalid Input Error: JSON transform error in file "cves/2001/1xxx/CVE-2001-1517.json", in record/value 1: Object {"affected":[{"product":"n/a","vendor":"n/a","vers... has unknown key "tags"
Try increasing 'sample_size', reducing 'maximum_depth', specifying 'columns', 'format' or 'records' manually, setting 'ignore_errors' to true, or setting 'union_by_name' to true when reading multiple files with a different structure.
D 
```

Fair enough warning about inconsistent schema across files and it suggested what I should explore. I like tools that hints at what to do in case of error💯. 

I am not sure which one is the best option here though: `ignore_errors` or perhaps `union_by_name` is a better idea🤔? I am going ahead with `union_by_name`. I had to tweak the query little bit: `SELECT * FROM read_json('cves/2000/**/*.json', union_by_name = true);` 

![image.png](../../../_resources/7f2cbeea-6a8e-4ea0-b561-fd5adda26238.png)

Nice!

Note that we are  not yet inserting these JSON record into to a SQL table but rather processing them on the fly. I say this because I don’t see the opened database `test.duck.db` size to increase at all!

To insert these JSON records into SQL table that can be queries later, we use the following 

```bash
D CREATE TABLE cve AS SELECT * FROM read_json_auto('./cves/2003/*/*.json', union_by_name = true);
100% ▕████████████████████████████████████████████████████████████▏ 
D select COUNT(*) FROM cve;
┌──────────────┐
│ count_star() │
│    int64     │
├──────────────┤
│         1553 │
└──────────────┘
```

I am ready to append to this table and write queries. 

### Using python module

I did the same exercise as before using duckdb python module. 

```python
 import duckdb                                                                                                                               
                                                                                                                                                
 git_repo = _cve_repo_dir()                                                                                                                  
 logger.info(f"Syching duckdb from {git_repo}...")                                                                                           
 json_files = list(git_repo.glob("cves/*/**/*.json"))                                                                                        
                                                                                                                                                
 con = duckdb.connect("cve.duck.db")                                                                                                         
                                                                                                                                                
 # create table if it doesn't exists using a JSON.                                                                                           
 first_json = json_files[-1]                                                                                                                 
 logger.info(f"> Creating table using {first_json}")                                                                                         
 con.sql(                                                                                                                                    
    f"CREATE TABLE IF NOT EXISTS cves AS SELECT * FROM read_json('{str(first_json)}', union_by_name = True)"                                
 )                                                                                                                                           
 s = con.sql("SELECT * FROM cves;")                                                                                                          
 print(s)                                                                                                                                    
```

```bash
2024-11-30 11:46:24.723 | INFO     | utils:sync_duckdb:40 - > Creating table using /home/dilawar/Work/SUBCOM/keeda/keeda-py/cve_json_db.git/cves/2024/9xxx/CVE-2024-9999.json
┌────────────_─────────────_──────────────────────_────────────────────────────────────────────────────────────────────────────────────────────┐
│  dataType  │ dataVersion │     cveMetadata      │                                         containers                                         │
│  varchar   │   varchar   │ struct(cveid varch_  │ struct(cna struct(affected struct(defaultstatus varchar, platforms varchar[], product va_  │
├────────────┼─────────────┼──────────────────────┼────────────────────────────────────────────────────────────────────────────────────────────┤
│ CVE_RECORD │ 5.1         │ {'cveId': CVE-2024_  │ {'cna': {'affected': [{'defaultStatus': unaffected, 'platforms': [Windows], 'product': W_  │
└────────────┴─────────────┴──────────────────────┴────────────────────────────────────────────────────────────────────────────────────────────┘
```

Great, once I have table initialized with schema, I can insert entries from other files: `INSERT INTO cves SELECT * FROM read_json('{str(file)}')`. One has to ensure that other files have same schema. If not, then we get error while inserting. 

```bash
TypeMismatchException: Mismatch Type Error: Type STRUCT(cveId VARCHAR, assignerOrgId UUID, state VARCHAR, dateReserved VARCHAR, dateUpdated 
VARCHAR, dateRejected VARCHAR, assignerShortName VARCHAR) does not match with STRUCT(cveId VARCHAR, assignerOrgId UUID, state VARCHAR, 
assignerShortName VARCHAR, dateReserved VARCHAR, datePublished VARCHAR, dateUpdated VARCHAR). Cannot cast STRUCTs - element "dateRejected" in 
source struct was not found in target struct

```

In this case, the other files have some fields added over time and it make sense to add those columns/fields to the schema on the fly (ref: https://duckdb.org/docs/sql/statements/update.html#update-from-other-table).

But I just couldn’t figure out how to the resolve the following error. Looks like updating an existing table to support the schema of a new file is not supported out of the box.

```bash
TypeMismatchException: Mismatch Type Error: Type STRUCT(defaultStatus VARCHAR, platforms VARCHAR[], product VARCHAR, vendor VARCHAR, versions 
STRUCT(lessThan VARCHAR, status VARCHAR, "version" VARCHAR, versionType VARCHAR)[]) does not match with STRUCT(collectionURL VARCHAR, 
defaultStatus VARCHAR, packageName VARCHAR, product VARCHAR, versions STRUCT(lessThanOrEqual VARCHAR, status VARCHAR, "version" VARCHAR, 
versionType VARCHAR)[], vendor VARCHAR). Cannot cast STRUCTs of different size

```

The solution to this problem in this case was simple. Each sub-directory contains files with the same schema. I created a table for each sub-directory!

2.7GB worth of CVEs were stored in 456MB of database file.

```bash
[dilawar@khaja keeda-py (download_cve_database)]$ du -sh cve_json_db.git/cves/
2.7G    cve_json_db.git/cves/
[dilawar@khaja keeda-py (download_cve_database)]$ ls -ltrh cve.duck.db 
-rw-r--r-- 1 dilawar dilawar 456M Nov 30 20:56 cve.duck.db
[dilawar@khaja keeda-py (download_cve_database)]$
```

## Querying

Now the querying part. I want to query CVEs that affect a product called `wpzoom`. Thanks https://github.com/duckdb/duckdb/issues/9901#issuecomment-1843178389

```bash
D select * from cve_in_year_2024_9xxx where len(list_filter(containers->>'$..affected[*].vendor', x-> x == 'wpzoom'));
┌────────────_─────────────_──────────────────────_────────────────────────────────────────────────────────────────────────────────────┐
│  dataType  │ dataVersion │     cveMetadata      │                                     containers                                     │
│  varchar   │   varchar   │ struct(cveid varch_  │ struct(cna struct(providermetadata struct(orgid uuid, shortname varchar, dateupd_  │
├────────────┼─────────────┼──────────────────────┼────────────────────────────────────────────────────────────────────────────────────┤
│ CVE_RECORD │ 5.1         │ {'cveId': CVE-2024_  │ {'cna': {'providerMetadata': {'orgId': b15e7b5b-3da4-40ae-a43c-f7aa60e62599, 'sh_  │
└────────────┴─────────────┴──────────────────────┴────────────────────────────────────────────────────────────────────────────────────┘
D 
```

## Notes

1. https://www.reddit.com/r/DuckDB/comments/1dvxcd6/importreading_large_json_file/
2. https://github.com/duckdb/duckdb/issues/9901#issuecomment-1843178389