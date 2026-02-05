---
title: >-
  Notes on interface, type and class in Typescript
  1d3e579bff898029b0acf869ffbb8211
updated: 2026-01-14 22:54:16Z
created: 2026-01-15 04:51:09Z
---

# Notes on interface, type and class in Typescript

Published On: June 22, 2025
Author: dilawar
Status: Published
Type: Note
Tags: lang:ts

- Use `interface` for behavior and `type` for data.
    
    > `interface` can be extended which I think fits better with behavior, `type` can be *intersected* which fits better with composibility of data.
    > 
    > 
    > `interface` can be implicitly merged.
    > 
    > ```
    > interface Calculator {
    >     add: (a: number, b: number) => number
    > }
    > 
    > interface Calculator {
    >     sub: (a: number, b: number) => number
    > }
    > ```
    > 
    > `interface` is mutable, `type` is immutable. I think that fits better with behavior being mutable and data being immutable.
    > — https://www.reddit.com/r/typescript/comments/qc6u5d/comment/hheyh54/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    > 
- Perhaps interface are good for API and type of internal use since type can’t be ‘merged’
    
    > Are you publishing type declarations for your API? Then it might be helpful for your users to be able to merge in new fields via an `interface` when the API changes. So use `interface`. But for a type that's used internally in your project, declaration merging is likely to be a mistake. So prefer `type`. (Vanderkam 56)
    > 
    

[10 Insights from Adopting TypeScript at Scale | Bloomberg LP](https://www.bloomberg.com/company/stories/10-insights-adopting-typescript-at-scale/)