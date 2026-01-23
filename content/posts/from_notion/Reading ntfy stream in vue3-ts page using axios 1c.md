---
title: >-
  Reading ntfy stream in vue3-ts page using axios
  1c8e579bff8980359c83e77994f16a22
updated: 2026-01-14 22:54:14Z
created: 2026-01-15 04:51:09Z
---

# Reading ntfy stream in vue3-ts page using axios

Published On: April 1, 2025
Author: dilawar
Status: Published
Type: Note
Tags: axios, lang:js, ntfy

The following worked for me

```jsx

onMounted(async () => {
  refreshMetricInterval = setInterval((_) => {
    fetchData()
  }, C.DEFAULT_POLL_INTERVAL_SEC * 1000)

  // Subscribe to ntfy.
  const ntfy = await axios.get('https://ntfy.sh/mytopic/json', {
    headers: {
      Authorization: 'Bearer yout_token',
      Accept: 'text/event-stream',
    },
    responseType: 'stream',
    adapter: 'fetch',
  });

  //  This doesn't work. We are using fetch API and the below solutions work.s
  //
  // ```
  //  console.debug('axa1', ntfy.data)
  //  ntfy.data.on('message', (data) => {
  //    console.debug(444, data);
  //  });
  // ```
  for await (const data of ntfy.data) {
    const json = JSON.parse(new TextDecoder().decode(data));
    console.debug('Got line from ntfy', json);
  }
})

```