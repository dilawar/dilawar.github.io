---
title: A simple Leptos app 1c4e579bff8980e69450f7ba0456238c
updated: 2026-01-14 22:54:14Z
created: 2026-01-15 04:51:09Z
---

# A simple Leptos app

Published On: March 28, 2025
Author: dilawar
Status: Published
Type: Note

[rust-projects/leptos at main · dilawar/rust-projects](https://github.com/dilawar/rust-projects/tree/main/leptos)

![A form that stores data to `LocalStorage`. Its reactive!](../../../_resources/leptos_gif.gif)

A form that stores data to `LocalStorage`. Its reactive!

![Audio recording in the app.](../../../_resources/image-7.png)

Audio recording in the app.

There is also a QR scanner that may or may not work. I used a third party crate that I patched to compile with Leptos 0.7 but did not test is thoroughly.

`leptos-use` (https://leptos-use.rs/) makes it easy to access browser’s API such a audio/video streams. The audio recorder records the audio in chunk of a few seconds and plot the raw values on the canvas. The plotting is pretty bad!

I’ve used https://thawui.vercel.app/ components but I could not make it work with slightly complicated hooks that update the local storage. So I reverted back to standard leptos `input` component.

```rust
#[component]
pub fn Form() -> impl IntoView {
    let storage_key = RwSignal::new("".to_string());

    let (state, set_state, _) = use_local_storage::<KeyVal, JsonSerdeCodec>(storage_key);

    let upload_patient_consent_form = move |file_list: FileList| {
        let len = file_list.length();
        for i in 0..len {
            if let Some(file) = file_list.get(i) {
                tracing::info!("File to upload: {}", file.name());
            }
        }
    };

    view! {
        <h5>"Form"</h5>
        <Space vertical=true class=styles::ehr_list>

            // Everything starts with this key
            <ListItem label="Code".to_string()>
                <input bind:value=storage_key />
            </ListItem>

            // Patient
            <InputWithLabel key="phone".to_string() state set_state></InputWithLabel>
            <InputWithLabel key="name".to_string() state set_state></InputWithLabel>
            <SelectWithLabel
                key="gender".to_string()
                options=Gender::iter().map(|x| x.to_string()).collect()
                state
                set_state
            ></SelectWithLabel>
            <InputWithLabel key="extra".to_string() state set_state></InputWithLabel>

        </Space>
    }
}

#[derive(Debug, strum::EnumIter, strum::Display)]
enum Gender {
    Male,
    Female,
    Other,
}

#[component]
pub fn InputWithLabel(
    key: String,
    state: Signal<KeyVal>,
    set_state: WriteSignal<KeyVal>
) -> impl IntoView {
    let label = key.split("_").join(" ");
    let key1 = key.to_string();

    view! {
        <Flex>
            <Label>{label}</Label>
            // Could not get thaw::Input to change when value in parent changes.
            <input
                prop:value=move || {
                    state.get().0.get(&key1).map(|x| x.to_string()).unwrap_or_default()
                }
                on:input=move |e| {
                    set_state
                        .update(|s| {
                            s.0.insert(key.to_string(), event_target_value(&e));
                        })
                }
            />
        </Flex>
    }
}

// SelectWithLabel is not shown. See the linked repo for updated code.
```