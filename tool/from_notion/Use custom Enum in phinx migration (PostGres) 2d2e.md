---
title: Use custom Enum in phinx migration (PostGres) 2d2e579bff8980f8846dc3047f74a97c
updated: 2026-01-14 22:55:14Z
created: 2026-01-15 04:51:09Z
---

# Use custom Enum in phinx migration (PostGres)

Published On: December 23, 2025
Author: dilawar
Status: Published
Type: Note

- Use raw query to add custom enum
- Use `Literal` class to refer to it later in table

Example

```php
<?php

declare(strict_types=1);

use Phinx\Migration\AbstractMigration;
use Phinx\Util\Literal;

final class SampleKitTrackingTable extends AbstractMigration
{
    /**
     * Sample kit and location tracking page.
     */
    public function change(): void
    {
        $this->execute("CREATE TYPE shipment_status as ENUM('CREATED', 'PICKED_UP', 'IN_TRANSIT', 'DELIVERED', 'FAILED')");
        $table = $this->table('sample_kit_location_tracking_v1', ['id' => false, 'primary_key' => ['uid'] ]);
        $table->addColumn('uid', 'uuid')
            ->addColumn('sample_kit_uid', 'uuid', ['null' => false])
            ->addColumn('location_uid', 'uuid', ['null' => false])
            ->addColumn('status', Literal::from('shipment_status'), ['null' => false, 'default' => 'CREATED'])
            ->addColumn('notes', 'string')
            ->addColumn('created_by', 'string')
            ->addColumn('created_at', 'datetime', ['null' => false, 'default' => 'CURRENT_TIMESTAMP'])
            ->addColumn('updated_at', 'datetime', ['null' => false, 'default' => 'CURRENT_TIMESTAMP'])
            ->addForeignKey('sample_kit_uid', 'sample_kit_v1', 'uid', ['delete' => 'CASCADE'])
            ->addForeignKey('location_uid', 'locationv1', 'uid', ['delete' => 'CASCADE'])
            ->addIndex(['sample_kit_uid', 'location_uid', 'status'], ['unique' => true])
            ->create();
    }
}
```