---
title: "Saving Blob to a file"
date: "2020-11-02"
categories: 
  - "web"
---

- Create an url from a `Blob` using `createObjectUrl`.
- Create an element with tag `a` and use the url generated above. Set properties such as `a.download` and `a.href` on this `a`. Use `a.click` to click on this `a`.

Here is the function.

```javascript
saveBlob : function(blob, filename) {
      var url = window.URL.createObjectURL(blob);
      var a = document.createElement("a");
      a.target = "_blank";
      a.href = url;
      a.download = filename;
      a.onclick = function(e) {
        setTimeout( () => {
          window.URL.revokeObjectURL(url);
        }, 100);
      };
      a.click();
      a.remove();
},
```
