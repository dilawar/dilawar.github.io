---
title: Using PostgreSQL ENUM in phinx migrations 294e579bff8980499accf85ef20a7898
updated: 2026-01-14 22:54:52Z
created: 2026-01-15 04:51:09Z
---

# Using PostgreSQL ENUM in phinx migrations

Published On: October 22, 2025
Author: dilawar
Status: Published
Type: Article
Tags: note

Currently, [Phinx](../../../undefined) doesn’t support `enum` for postgres: https://github.com/cakephp/phinx/issues/891.

Someone mentioned a solution on the issue (https://github.com/cakephp/phinx/issues/891#issuecomment-774496499).

```php
$this->execute("create type foo as enum ('bar1', 'bar2')");
$this->execute('alter table table_name add column foo_column foo');

```

This works! In my project, I’ve used this pattern multiple times. E.g.

```php
// table to track bag-sample relationship.
$bagSample = $this->table('bag_sample_relationv1');
$bagSample->addColumn('sample_bag_code', 'string', ['null' => false])
    ->addColumn("sample_code", "string", ['null' => false, 'limit' => 20])
    ->addColumn('added_at', 'datetime', ['default' => 'CURRENT_TIMESTAMP'])
    ->addIndex(['sample_bag_code', 'sample_code'], ['unique' => true])
    ->save();
        
 // add a column with ENUM type.
$this->execute("create type BAG_SAMPLE_STATUS as ENUM('INSIDE', 'OUTSIDE', 'REMOVED')");
$this->execute("ALTER TABLE bag_sample_relationv1 ADD COLUMN status BAG_SAMPLE_STATUS DEFAULT 'INSIDE'");
```