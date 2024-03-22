Spoopify
========

a DRM-free clone of spotify for local-file music libraries

Notes
=====

TODO; starting out with webview, may switch to FLTK wrapper later

So we've got an example working with Content::Html, and trying Content::Url file the "file:///" protocol just
ended with a crash

```
fatal runtime error: Rust cannot catch foreign exceptions
error: process didn't exit successfully: `target\debug\spoopify.exe` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
```

So I'm thinking we will be forced to embed a webserver and serve media via that. Might also help with the
workflow of the app, instead of using `invoke_handler`, `eval`, and `window.external.invoke`. Those would
work, but we will have issues with bandwidth; transferring lots of image and audio files will benefit from
caching and such built into the HTTP server/client technology already built into web browsers. this also
means we could create a headless version of spoopify, and allow graphical clients to access a remote GUI
hosted on a server on the network. However, communication between devices (playback sync, library sync)
will NOT be over HTTP APIs; I will look into syncthing and how it's backend supports finding devices;
I can't remember what this will look like, I'm reminded of when I read about UDP hole punching a while
ago. I think there might be libraries catered to this use case that we can use.

we might want to consider the tao crate for GUIs on android, since that project should have the support
we need for webviews on android. also supports iOS. https://crates.io/crates/tao

the webview component is named wry, which links to tools for creating template projects for android and ios
so this is definitely something to look into. I'm curious how tao/wry handles image/audio files, if it
uses a local HTTP server or something else. https://crates.io/crates/wry

looking at the tauri crate, it's scope is a bit heavier, if we wanted things like "custom menus and
tray-type interfaces", it would be better to use FLTK as mentioned above. but it can't hurt to look at
it's source for information/learning. https://github.com/tauri-apps/tauri/blob/dev/ARCHITECTURE.md
