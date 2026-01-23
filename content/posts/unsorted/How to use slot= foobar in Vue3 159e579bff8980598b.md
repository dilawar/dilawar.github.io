---
title: How to use slot= foobar in Vue3 159e579bff8980598bf8cddd10cee262
updated: 2026-01-14 22:54:14Z
created: 2026-01-15 04:51:09Z
---

# How to use slot="foobar" in Vue3

Published On: December 11, 2024
Author: dilawar
Status: Published
Type: TIL
Tags: vue

Following is Vue2 code which is refactored for Vue3 below.

```html
<f7-list-input label="Search patient" :input="false">
    <vue-simple-suggest
	    slot="input"
      class="item-input-wrap"
      :max-suggestions="10"
      :nullable-select="false"
      :min-length="4"
      @update:model-select="onPatientSelected"
      :list="queryPatient"
    >
    </vue-simple-suggest>
</f7-list-input>
```

```html
<f7-list-input label="Search patient" :input="false">
  <template v-slot:input>
    <vue-simple-suggest
      class="item-input-wrap"
      :max-suggestions="10"
      :nullable-select="false"
      :min-length="4"
      @update:model-select="onPatientSelected"
      :list="queryPatient"
    >
    </vue-simple-suggest>
  </template>
</f7-list-input>
```