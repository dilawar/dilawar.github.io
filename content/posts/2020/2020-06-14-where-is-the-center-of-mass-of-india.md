---
title: "Where is the center of mass of India?"
date: "2020-06-14"
categories: 
  - "visualization"
tags: 
  - "center-of-mass"
  - "centroid"
  - "india"
  - "india-map"
  - "map-of-india"
---

You want to lift India on your fingertip. You have already sliced the country using a cosmic knife 1 km below the sea level. Where would you keep your finger? Remember the Indian plate needs to be balanced.

There is no easy answer to this question. The estimates, however, are easier to make. One such estimate is below. There are two markers on the map. The one in the state of Madhya Pradesh (MP) is the centroid of the country. If you draw a map on a paper, the middle point of the map is the centroid. But you lift the country at the centroid, it won't be balanced; the Himalayas are heavy and it will tilt the country to the north.

The [center of mass](https://en.wikipedia.org/wiki/Center_of_mass) of a country is a weighted centroid. Each pixel on the map below has a weight that is proportional to its elevation.

![](images/india_com.png)

There are a few caveats to this answer. I am assuming uniform soil density which is not true. The sand in Ganges plains has less density than rocks of the Western Ghats or the Himalayas. Also, think about the boundaries and how precise they are. How are they mapped from near-spherical Earth to a flat paper?

The technical details can be found at this repository [https://github.com/dilawar/map-india-center](https://github.com/dilawar/map-india-center). This work can easily be extended to compute the center of mass of each state. I might do so it in the future. If you do it, let me know.
