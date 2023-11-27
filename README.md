# Macropinna

An all-in-one home screen, launcher, and remote controller for your home media server.

Featuring:

- Customizable pages with widgets like clocks, weather info, media players, and an audio visualizer
- An IP-enabled remote control from your phone
- Customizable launchers to run any app on your machine

## Installation

Macropinna is in active development and has not yet had a first release. If you want to try it, you'll have to build from source.

## Building

This project's development environment depends on **Tauri** and **npm**:
- [**Tauri** setup](https://tauri.app/v1/guides/getting-started/prerequisites)
    - [**Tauri CLI**](https://tauri.app/v1/guides/faq#node-or-cargo): there are 2 ways to install the CLI, through Cargo or npm. Commands in this readme are written for a Cargo installation (e.g. `cargo tauri dev`), but you can use the npm version as well.
- [**npm** setup](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

**Platform-specific**: [cpal](https://crates.io/crates/cpal) requires ALSA dev files - install `libasound2-dev` on Debian/Ubuntu and `alsa-lib-devel` on Fedora.

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

I also haven't gone out of my way to use any new features of any of the software that Macropinna depends on (which is why I'm not going to say _you need these exact versions_), but I can't say the same for its dependencies. So while it may build with older versions of all of the above, any "weird" build errors may be the result of incompatible versions. These will be locked down in the future.
