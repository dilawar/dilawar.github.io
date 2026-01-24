---
title: >-
  Note Updating UI after a successful HTMX request
  2bce579bff89809dabf7c95b2f3b92d8
updated: 2026-01-14 22:55:06Z
created: 2026-01-15 04:51:09Z
---

# Note: Updating UI after a successful HTMX request

Published On: December 1, 2025
Author: dilawar
Status: Published
Type: Note
Tags: note

- 👎🏾Reload the page. Send response header `HX-Refersh` which ensures that page is reloaded afters successful request. This is not the greatest of options because refreshing page causes UI to move.
    
    ```php
        /**
        * Add HX-Refresh header to response. The client will refresh the page.
        */
        private function addRefreshHeader(): void
        {
            $this->response->setHeader('HX-Refresh', "true");
        }
       
    ```
    
- 👍🏾 Trigger another HTMX read request that fetches the content on success. If you are coming from react/vue, this will feel natural. Send a header like `HX-Trigger: HX-Updated-My-Table` and setup a `hx-get` to be triggered on this event e.g.
    
    <aside>
    💡
    
    Ensure that you’ve exposed `HX-Trigger` header using [https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Expose-Headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Expose-Headers)
    
    </aside>
    
    ```html
    <h2>My Table</h2>
       ...
      <tbody id="mytable-table" hx-get="/read/my-table" hx-trigger="HX-Updated-My-Table from:body">
        ...
      </tbody>
    </table>
    
    <form hx-put="/put/my-table">
      <label>
        Name
            <input name="key" type="text">  
      </label>
      <label>
        Email
            <input name="value" type="text">  
      </label>
    </form>
    ```
    
- There are more interesting options https://htmx.org/examples/update-other-content/