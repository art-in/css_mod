Example of using `css_mod` in web app created with [yew](https://yew.rs/) and bundled with [trunk](https://trunkrs.dev/).

This is implementation of [TodoMVC](https://todomvc.com/) web app.

## Concepts

-   Component styles encapsulation (localized class/animation names)
-   Sharing styles between multiple components (in `src/shared.css`)
-   Global styles (in `src/global.css`)
-   Prebuilding css assets for `trunk` (see `Trunk.toml`)

## Install & Run

```sh
> rustup target add wasm32-unknown-unknown
> cargo install trunk
> trunk serve
```
