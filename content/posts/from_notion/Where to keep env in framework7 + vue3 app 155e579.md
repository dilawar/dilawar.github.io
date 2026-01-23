---
title: Where to keep env in framework7 + vue3 app 155e579bff898029a515cb9f933e0b2a
updated: 2026-01-14 22:54:14Z
created: 2026-01-15 04:51:09Z
---

# Where to keep .env in framework7 + vue3 app?

Published On: December 7, 2024
Author: dilawar
Status: Published
Type: TIL

In `src` folder because `root` is defined to be this directory in `vite.config.js`. 

```jsx

import path from 'path';
import vue from '@vitejs/plugin-vue';

**const SRC_DIR = path.resolve(__dirname, './src');**
const PUBLIC_DIR = path.resolve(__dirname, './public');
export default async () => {
  return  {
    plugins: [
      vue({ template: { compilerOptions: { isCustomElement: (tag) => tag.includes('swiper-') } } }),,
    ],
    **root: SRC_DIR, // <-- We are looking for this.**
    base: '',
		
  };
}

```