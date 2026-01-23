---
title: Using monolog with codeigniter4 1f5e579bff8980799bdde426011b49f9
updated: 2026-01-14 22:54:16Z
created: 2026-01-15 04:51:09Z
---

# Using monolog with codeigniter4

Published On: May 16, 2025
Author: dilawar
Status: Published
Type: Note
Tags: lang:php, note

The standard logger in codeigniter4 is fine but its no monolog!

To use monolog codeigniter4, add the following to your `app/Config/Service.php` file. You can read more about supported handlers and formatters in monolog documentation. See https://github.com/Seldaek/monolog/blob/main/doc/02-handlers-formatters-processors.md#formatters. and there are many third party handlers and formatters available.

- Note that I am logging to syslog rather than to a file (this won’t work on Windows). I think logging to syslog is better if you already have a scrapper running. And also, correlating with web-server logs is bit easier. Perhaps I am mistaken.
- I am using a third party formatter http://github.com/bramus/monolog-colored-line-formatter to add colors to console logger. I think colors are a good idea if you need to scroll and look at the logs while developing.

```php
<?php

// app/Config/Services.php 

use Bramus\Monolog\Formatter\ColoredLineFormatter;
use Monolog\Formatter\LineFormatter;
use Monolog\Handler\BrowserConsoleHandler;
use Monolog\Handler\StreamHandler;
use Monolog\Handler\SyslogHandler;
use Monolog\Logger;

class Services Extends BaseService 
{
    // Other services are not shown.
         
    /**
     * Use monolog logger.
     *
     * - Logs to syslogs
     * - Logs to console (colored)
     * - Logs to browser console (development only).
     */
    public static function logger(bool $getShared = true): Logger
    {
        if ($getShared) {
            return static::getSharedInstance('logger');
        }

        $logger = new Logger('my-portal');
        $consoleHandler = new StreamHandler('php://stdout', \Monolog\Level::Info);
        $consoleHandler->setFormatter(new ColoredLineFormatter());
        $logger->pushHandler($consoleHandler);

        // Also log to syslog
        $facilityName = "local6"; // See list here https://en.wikipedia.org/wiki/Syslog#Facility_levels
        $sysLogHandler = new SyslogHandler('my-portal', $facilityName);
        $formatter = new LineFormatter("%channel%.%level_name%: %message% %extra%");
        $sysLogHandler->setFormatter($formatter);
        $logger->pushHandler($sysLogHandler);

        if(ENVIRONMENT === "development") {
            $logger->pushHandler(new BrowserConsoleHandler());
        }

        return $logger;
    }
}
```

Continue to use your `log_message` function as before, and perhaps comment out all but one handlers in `app/Config/Logger.php` file.

I think once service with alias `logger` is added, most handlers in file `app/Config/Logger.php` stops working by themselves.