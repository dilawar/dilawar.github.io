---
title: "Djikestra Shortest Path Algorithm : LEDA performance"
date: "2011-12-08"
categories: 
  - "graph-theory"
  - "mathematics"
tags: 
  - "djikstra-path-algorithm"
  - "leda"
  - "performance-of-djikestra"
  - "shortest-path-algorithm"
---

I have written this program which tests LEDA library performance for shortest path algorithm performance. This library is pretty good. Much easier to use than boost libraries. \[sourcecode language="c"\] /\* \* ===================================================================================== \* \*       Filename:  dikkstra.cpp \* \*    Description:  Dijksjtra algorithm for calculating shortest path. \* \*        Version:  1.0 \*        Created:  Tuesday 06 December 2011 04:45:57  IST \*       Revision:  none \*       Compiler:  gcc \* \*         Author:  Dilawar Singh (Graduate Student, EE IITB), dilawar@ee.iitb.ac.in \*      Institute:  IIT Bombay \* \* ===================================================================================== \*/ #include    <LEDA/graph/graph.h> #include    <LEDA/graph/node\_pq.h> using namespace leda; using namespace std; /\* fun : DIJKESTRA \* Args : a const graph, starting node \*        array of edges. \*        array of nodes \*/ void DIJKESTRA ( const graph &G, node s, const edge\_array<double>& cost, node\_array<double>& dist) { node\_pq<double> PQ(G); /\* priority queue \*/ node v; edge e; forall\_nodes(v,G) { if (v == s) dist\[v\] = 0; else dist\[v\] = MAXDOUBLE; PQ.insert(v, dist\[v\]); } while( !PQ.empty()) { node u = PQ.del\_min(); forall\_out\_edges(e,u) { v = target(e); double c = dist\[u\] + cost\[e\]; if(c < dist\[v\]) { PQ.decrease\_p(v,c); dist\[v\] = c; } } } } int main() { int n = 1; // = read\_int("number of nodes = "); int m = 1; // = read\_int("number of edges = "); graph G; edge e; cout << "\\nTime, Nodes, Edges\\n"; int i; for(i = 0; i < 50000; i++) { random\_graph(G,n,m); edge\_array<double> cost(G); node\_array<double> dist(G); forall\_edges(e,G) cost\[e\] = ((double) rand\_int(0,100)); float T = used\_time(); DIJKESTRA(G, G.first\_node(), cost, dist); cout<<used\_time(T)<<","<<n<<","<<m<<"\\n"; n = (i+10)\*(i+10) - 9\*i; m = rand\_int(1,3)\*n; } return 0; } &nbsp; \[/sourcecode\] Give appropriate path and libraries to compiler.
