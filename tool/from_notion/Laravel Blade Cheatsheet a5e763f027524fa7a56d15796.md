---
title: Laravel Blade Cheatsheet a5e763f027524fa7a56d15796f8fe250
updated: 2026-01-14 22:58:22Z
created: 2026-01-15 04:51:09Z
---

# Laravel Blade Cheatsheet

Published On: November 14, 2024
Author: dilawar
Status: Published
Type: Note
Tags: note

Original source is linked at the end of this page.

|  | What |
| --- | --- |
| `{{ $var }}` | Echo content |
| `{{ $var or 'default' }}` | Echo content with a default value |
| `{{{ $var }}}` | Echo escaped content |
| `{{-- Comment --}}` | A Blade comment |
| `@extends('layout')` | Extends a template with a layout |
| `@if(condition)` | Starts an if block |
| `@else` | Starts an else block |
| `@elseif(condition)` | Start a elseif block |
| `@endif` | Ends a if block |
| `@foreach($list as $key => $val)` | Starts a foreach block |
| `@endforeach` | Ends a foreach block |
| `@for($i = 0; $i < 10; $i++)` | Starts a for block |
| `@endfor` | Ends a for block |
| `@while(condition)` | Starts a while block |
| `@endwhile` | Ends a while block |
| `@unless(condition)` | Starts an unless block |
| `@endunless` | Ends an unless block |
| `@include(file)` | Includes another template |
| `@include(file, ['var' => $val,...])` | Includes a template, passing new variables. |
| `@each('file',$list,'item')` | Renders a template on a collection |
| `@each('file',$list,'item','empty')` | Renders a template on a collection or a different template if collection is empty. |
| `@yield('section')` | Yields content of a section. |
| `@show` | Ends section and yields its content |
| `@lang('message')` | Outputs message from translation table |
| `@choice('message', $count)` | Outputs message with language pluralization |
| `@section('name')` | Starts a section |
| `@stop` | Ends section |
| `@endsection` | Ends section |
| `@append` | Ends section and appends it to existing of section of same name |
| `@overwrite` | Ends section, overwriting previous section of same name |
| `@isset($records)
   $records is defined and is not null...  
@endisset` |  |
| `@production` 
    `// code to be displayed just when .env is set to production`  
`@endproduction` |  |
| `@auth` 
     `//code for logged-in users`  
`@endauth` |  |
| `@guest`  
     `//code for guest users` 
`@endguest` |  |

[https://gist.github.com/CiprianSpiridon/f4d7fe0d8a51f0714b62](https://gist.github.com/CiprianSpiridon/f4d7fe0d8a51f0714b62)