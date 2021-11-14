---
title: "How to detect architecture (32bit/64bit) of your system using cmake"
date: "2016-01-13"
---

Use the following snippet

if("${CMAKE\_SIZEOF\_VOID\_P}" EQUAL "8")
  MESSAGE("++ 64 bit architecture")
  #set(LIBSUFFIX "64")
  set(PKGARCH "amd64")
  # set(RPMPKGARCH "x86\_64")
 else()
  MESSAGE("++ 32 bit architecture")
  # set(LIBSUFFIX "")
  # set(PKGARCH "i386")
endif()
