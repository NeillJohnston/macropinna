# projector-ambience Architecture

`project-ambience` is split into a backend, an app UI, and a remote frontend. The backend hosts the app UI natively and also serves the remote frontend.

Users can connect to the locally-hosted web frontend to access a remote control for the app. _TODO: for security, access to the remote is only granted by either short password generated on the app (maybe 4 letters or 6 digits?), or via whitelist that can only be enabled on the host._

The remote frontend and app backend communicate via WebSocket. WwebSockets are chosen as opposed to a REST/RPC server because fast bidirectional communication is important. Once connected, a user with remote access can send specific control commands to navigate the app UI and perform special actions immediately (like adjusting keystone correction and switching to apps).

Besides the remote, the app is built with Svelte with TypeScript on top of Tauri. Svelte/TS is chosen largely for my own familiarity and its simplicity. Tauri is chosen for its speed, small binaries, and my own familiarity with Rust. The UI and backend are hosted on the same machine (in the same process as well?) but are functionally two separate sources written in different languages.

## Remote

## UI

## Backend