---
title: "Python like string split in Haskell"
date: "2012-04-15"
categories: 
  - "language"
tags: 
  - "python-like-split-in-haskell"
  - "string-split-in-haskell"
---

Following function split a string into a list of strings at a given string. For example split "dilawar raw war" "aw" returns \["dil", "ar r" " war"\]

split :: String -> String -> \[String\]
split str pat = helper str pat \[\] \[\] where
    helper :: String -> String -> String -> String -> \[String\]
    helper \[\] ys n m = \[n\] ++ \[\]
    helper xs \[\] n m = \[n\] ++ (split xs pat)
    helper (x:xs) (y:ys) n m
        | x /= y = helper (xs) pat ((n++m)++\[x\]) m
        | otherwise = helper xs ys n (m++\[y\])
