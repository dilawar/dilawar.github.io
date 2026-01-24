---
title: >-
  PostgreSQL Given a column, list all possible value
  1e1e579bff8980ea821ef9db4158ae4e
updated: 2026-01-14 22:54:16Z
created: 2026-01-15 04:51:09Z
---

# PostgreSQL: Given a column, list all possible values it can take (if it is an enum)

Published On: April 26, 2025
Author: dilawar
Status: Published
Type: Article
Tags: note, postgres

See https://dba.stackexchange.com/questions/35497/display-user-defined-types-and-their-details for other answers. The following, which I think is easy to read and simplest, worked for me.

The following list columns and their types where `USER-DEFINED` is replaced by its name.

```sql
select column_name, 
case when (data_type = 'USER-DEFINED') then udt_name 
    else data_type 
    end as data_type
from information_schema.columns 
where table_schema = 'public' and table_name = 'samplev1';
```

I get the following.

```
column_name                           |data_type                  |
--------------------------------------+---------------------------+
version                               |integer                    |
uid                                   |character varying          |
collection_location_id                |character varying          |
sample_type                           |sampletype                 |
weight_in_kg                          |double precision           |
height_in_cm                          |double precision           |
patient_uid                           |character varying          |
patient_age_in_years_at_collection_day|integer                    |
num_hours_passed_since_your_last_meal |numeric                    |
created_at                            |integer                    |
last_edited                           |integer                    |
last_meal_types                       |json                       |
is_valid                              |boolean                    |
reason_for_invalid                    |character varying          |
status_code                           |integer                    |
collection_datetime                   |timestamp without time zone|
```

Now you can use the following query to get all values of enum type e.g. `sampletype`

```sql
SELECT unnest(enum_range(null, null::sampletype))
```

```
unnest  |
--------+
Positive|
Benign  |
Healthy |
Chronic |
```