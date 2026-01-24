---
title: >-
  php-cs-fixer configuration for codeigniter4 projec
  221e579bff898000b498f722cf8766d4
updated: 2026-01-14 22:54:18Z
created: 2026-01-15 04:51:09Z
---

# php-cs-fixer configuration for codeigniter4 project

Published On: June 29, 2025
Author: dilawar
Status: Published
Type: Note
Tags: lang:php

I’ve been using the following `.php-cs-fixer.php` script for my codeigniter projects. It is edited from https://roman-huliak.medium.com/my-own-php-cs-fixer-setup-a-practical-example-7a5f015da8fe. 

If you need to tweak the formatting behavior, have a look at https://mlocati.github.io/php-cs-fixer-configurator/#version:3.75 to learn more about its configurations.

- `composer require --dev friendsofphp/php-cs-fixer` to install the tool.
- Copy the following file to `.php-cs-fixer.php` file.
    
    ```php
    <?php
    
    // filename: .php-cs-fixer.php
    // Modified from https://roman-huliak.medium.com/my-own-php-cs-fixer-setup-a-practical-example-7a5f015da8fe 
    
    $finder = PhpCsFixer\Finder::create()->in([
        __DIR__ . '/app',
        __DIR__ . '/tests',
    ]);
    
    return (new PhpCsFixer\Config())
        ->setParallelConfig(PhpCsFixer\Runner\Parallel\ParallelConfigFactory::detect())
        ->setRules([
            '@Symfony' => true,
            '@PSR12' => true,
            'array_indentation' => true,
            'array_syntax' => ['syntax' => 'short'],
            'combine_consecutive_unsets' => true,
            'single_quote' => true,
            'ordered_imports' => [
                'sort_algorithm' => 'alpha',
                'imports_order' => ['const', 'class', 'function'],
            ],
            'no_useless_else' => true,
            'phpdoc_order' => true,
            'header_comment' => [
                'header' => "This file is part of the proprietary project.\n\nThis file and its contents are confidential and protected by copyright law.\nUnauthorized copying, distribution, or disclosure of this content\nis strictly prohibited without prior written consent from the author or\ncopyright owner.\n\nFor the full copyright and license information, please view the LICENSE.md\nfile that was distributed with this source code.",
                'separate' => 'both'
            ]
        ])
        ->setFinder($finder);
    ```
    
- Run `./vendor/bin/php-cs-fixer fix` to format the code and fix the coding violations.