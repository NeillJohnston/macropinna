# Macropinna

A launcher app for home media servers.

Largely inspired by phone screens and the Android theming community. I'm making this because I wanted something nice and minimal to put on my projector for some ambience while I worked.

Features:
- [ ] Customizable home screens, including:
    - [X] Clock
    - [X] Weather (via OpenWeatherMap)
    - [X] Audio visualizer
    - [ ] Media player (via MPRIS2)
- [ ] Customizable launcher apps
- [ ] Remote controllable from a phone (over IP, no apps needed)
- [ ] Keystone correction (via `xrandr`)

## Installation

Macropinna is in active development and has not yet had a first release. If you want to try it, you'll have to build from source.

## Building

This project's development environment depends on **Tauri** and **npm**:
- [**Tauri** setup](https://tauri.app/v1/guides/getting-started/prerequisites)
- [**npm** setup](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

Now install packages for the two SvelteKit apps. From the project root:

```
npm i
```

And from /src-remote:

```
npm i
```

Finally, to build and run the project, the Tauri CLI can be run from the project root:

```
cargo tauri dev
```

### Development cycle

Since this project depends on an extra SvelteKit project (/src-remote) which isn't managed by Tauri, you can't just keep `cargo tauri dev` running like a normal Tauri project. Sorry about that, working on it.

From /src-remote:

```
npm run build:watch
```

To rebuild the remote project in watch mode. In a separate terminal, you can now run

```
cargo tauri dev
```

Like normal.

### Note on versions

I haven't tried building with any old versions of Tauri/npm/etc., but for reference:

```
$ cargo tauri -V
tauri-cli 1.4.0

$ rustc -V
rustc 1.72.0 (5680fa18f 2023-08-23)

$ npm -v
8.19.2

$ node -v
v18.12.1
```

I also haven't gone out of my way to use any new features of any of the software that Macropinna depends on (which is why I'm not going to say _you need these exact versions_), but I can't say the same for its dependencies. So while it may build with older versions of all of the above, any "weird" build errors may be the result of incompatible versions.