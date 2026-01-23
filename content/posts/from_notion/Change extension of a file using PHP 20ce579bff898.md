---
title: Change extension of a file using PHP 20ce579bff89801d80e5d78bdcfad366
updated: 2026-01-14 22:54:16Z
created: 2026-01-15 04:51:09Z
---

# Change extension of a file using PHP

Published On: June 9, 2025
Author: dilawar
Status: Published
Type: Note
Tags: lang:php

Following is not the most efficient solution out there. But if you are fan of regex, it is easy to read and maintain.

```php
/**
 * Change extension of a given filepath. If filename does not have extension,
 * the new extension is simply appended.
 *
 * @param string $newExt May or may not start with .
 */
function changeExtension(string $filepath, string $newExt): string 
{
    $ext = str_starts_with($newExt, '.') ? $newExt : '.' . $newExt;
    return preg_replace('/\.[^.]+$/', '', $filepath) . $ext;
}
```