---
title: "Adding google-analytics to your vue application"
date: "2020-11-10"
categories: 
  - "notes"
  - "web"
tags: 
  - "google-analytics"
  - "vue"
  - "vue-gtag"
---

I am using Google's global site gags (gtags) on my framework7+vue application to collect data. The gtag official documentation [has an easy](https://developers.google.com/analytics/devguides/collection/gtagjs#install_the_global_site_tag) to use example. Anyway, I prefer `vue-gtag` library.

- install vue-gtag `npm install vue-gtag`
- Add following to your vue app. Usually you have to edit `src/js/app.js` file or equivalent file where vue is being initialized.

```js
// Init google analytics
import VueGtag from "vue-gtag";
Vue.use(VueGtag, {
   config: { id: "UA-180198765-1" }
});
```

where `id` key in `config` dictionary is your tracking id.

Official documentation here [https://matteo-gabriele.gitbook.io/vue-gtag/](https://matteo-gabriele.gitbook.io/vue-gtag/)

### Footnotes

- `[vue-gtag] Ops! Something happened and gtag.js couldn't be loaded`.

Ad-blocker?
