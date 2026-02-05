---
title: Where Is The Center Of India dfbda6d68db948dfa0b793ce8d1d4040
updated: 2026-01-14 22:58:28Z
created: 2026-01-15 04:51:09Z
---

# Where Is The Center Of India?

Published On: June 14, 2020
Author: dilawar
Status: Published
Type: Note
Tags: calculation

Imagine yourself to be Krishna — he was fond of lifting mountains. You want to lift India on your fingertip. You have already sliced the country using a cosmic knife 1 km below sea level. Where would you keep your finger? Remember that it needs to be balanced. There is no perfect answer to this question. The estimates, however, are easier to make. One such calculation is below.

There are two markers on the map. The one in Madhya Pradesh (MP) is the middle point of the country. If you draw a map on paper, the center of the map is the centroid. But if you lift the country at the centroid, it won't be balanced. The Himalayas are heavy and make the country tilt to the north. If you do this, you will hit some Chinese heads. They won't be happy!

![Two centers of India. The first one is the simple geometric mean of boundary points, and it falls inside the rightly named Madhya Pradesh, and the second is the center of mass which is very close to the UP/Uttrakhand border. The country boundary data is from the excellent [DataMeet community](http://datameet.org).](../../../_resources/Untitled-3.png)

Two centers of India. The first one is the simple geometric mean of boundary points, and it falls inside the rightly named Madhya Pradesh, and the second is the center of mass which is very close to the UP/Uttrakhand border. The country boundary data is from the excellent [DataMeet community](http://datameet.org).

The [center of mass](https://en.wikipedia.org/wiki/Center_of_mass) of a country is a weighted centroid. Each pixel on the map below has a weight proportional to its elevation.

![3D topographic map of India. The higher the altitude, the brighter the pixel. This data is available at [https://figshare.com/articles/dataset/India_s_elevation_profile_in_a_GeoTIFF_file/12479306/2](https://figshare.com/articles/dataset/India_s_elevation_profile_in_a_GeoTIFF_file/12479306/2). ](../../../_resources/Untitled%201.png)

3D topographic map of India. The higher the altitude, the brighter the pixel. This data is available at [https://figshare.com/articles/dataset/India_s_elevation_profile_in_a_GeoTIFF_file/12479306/2](https://figshare.com/articles/dataset/India_s_elevation_profile_in_a_GeoTIFF_file/12479306/2). 

There are a few caveats to this answer. I am assuming uniform soil density, which is not very accurate. The sand in the Ganges plains has a lower density than the rocks of the Western Ghats or the Himalayas. Additionally, the boundaries of India and how they are mapped from the near-spherical earth to a two-dimensional paper must also be considered.

The technical details can be found at this repository, [https://github.com/dilawar/map-india-center](https://github.com/dilawar/map-india-center). This work can easily be extended to compute the center of mass of each state. I might do it in the future.