---
title: "Awesome Vue : A personal list"
date: "2020-11-01"
categories: 
  - "web"
tags: 
  - "vue"
---

- To automatically create links from raw url and email links in `v-html` directive, use [vue-linkify](https://github.com/phanan/vue-linkify). Create a custom directory such as `v-linkified` and add this to the tag. For example,

```js
import linkify from 'vue-linkify'
Vue.directive('linkified', linkify)
```

```html
<div v-html="raw" v-linkified />
```

- [vue-dropzone](https://rowanwins.github.io/vue-dropzone/docs/dist/#/installation) is excellent for dragging/dropping images to a webpage and upload it to a server.
- [@codekraft-studio/vue-record](https://github.com/codekraft-studio/vue-record) for recording audio.
- [vue2-datepicker](https://www.npmjs.com/package/vue2-datepicker) is pretty cool to create date and time picker and it has been getting better.
- [framework7-vue](https://framework7.io/) is the coolest framework I know to create website and webapps. I can't praise it enough.
- To integrate google-analytics into your SPA, use [vue-ua](https://www.npmjs.com/package/vue-ua). Also checkout [vue-gtag](https://github.com/MatteoGabriele/vue-gtag). More options are listed [here](https://github.com/vuejs/awesome-vue#google-analytics).

_See this community maintained resource [for a comprehensive list](https://github.com/vuejs/awesome-vue)_.
