---
title: "CMake with Flex and Bison (Yacc) : ADD_CUSTOM_COMMAND"
date: "2013-01-30"
tags: 
  - "cmake-with-flex-and-bison"
---

In case I can not google it again. Here is this informative [post](http://www.cmake.org/pipermail/cmake/2002-September/003028.html). Following is the verbatim copy of this email.

\# Create target for the parser
 ADD\_CUSTOM\_TARGET(FooParser echo "Creating parser.c")
# Create custom command for flex/lex (note the outputs)
 ADD\_CUSTOM\_COMMAND(
   SOURCE ${Foo\_SOURCE\_DIR}/src/lexer.l
   COMMAND ${FLEX\_EXECUTABLE}
   ARGS -o${Foo\_BINARY\_DIR}/src/lexer.c
        ${Foo\_SOURCE\_DIR}/src/lexer.l
   TARGET FooParser
   OUTPUTS ${Foo\_BINARY\_DIR}/src/lexer.c)
# Create custom command for bison/yacc (note the DEPENDS)
 ADD\_CUSTOM\_COMMAND(
   SOURCE ${Foo\_SOURCE\_DIR}/src/parser.y
   COMMAND ${BISON\_EXECUTABLE}
   ARGS -y ${Foo\_SOURCE\_DIR}/src/parser.y
        -o ${Foo\_BINARY\_DIR}/src/parser.c
   TARGET FooParser
   DEPENDS ${Foo\_BINARY\_DIR}/src/lexer.c
   OUTPUTS ${Foo\_BINARY\_DIR}/src/parser.c)
# Add parser.c to the list of sources
 SET(Foo\_SRCS ${Foo\_SRCS} ${Foo\_BINARY\_DIR}/src/parser.c)
# Since parser.c does not exists yet when cmake is run, mark
# it as generated
 SET\_SOURCE\_FILES\_PROPERTIES(${Foo\_BINARY\_DIR}/src/parser.c GENERATED)
# Include binary directory to include lexer.c in parser.c
 INCLUDE\_DIRECTORIES(${Foo\_BINARY\_DIR}/src)
				Andy Cedilnik
				Kitware Inc.
