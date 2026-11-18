# OpenPencil fork: tray + Windows autostart

This fork is based on the `v0.16.0` release tag. It keeps OpenPencil running in the Windows
system tray so its local MCP server (`127.0.0.1:7600`) stays available to agents, and it fixes
two Windows issues that block the build and headless rendering.

## What this fork adds

- **Tray icon** (`desktop/src/tray.rs`): left click restores the window; right click opens a menu
  with *Open OpenPencil*, *Start with Windows* (checkable) and *Quit OpenPencil*.
- **Close to tray**: closing the window hides it instead of quitting, so the app and its MCP
  server keep running. Toggle in **Settings → General → Desktop**.
- **Minimize to tray**: minimizing hides the window instead of leaving a taskbar button. Toggle
  in **Settings → General → Desktop**.
- **Start with Windows**: writes `HKCU\Software\Microsoft\Windows\CurrentVersion\Run\OpenPencil`
  pointing at the installed executable with `--hidden`, so Windows starts it straight in the
  tray. Toggle in **Settings → General → Desktop**, or in the tray menu.
- **`--hidden` launch flag**: starts the app with the window hidden (the tray icon is created
  before the window is shown, so there is no flash).
- **Second launch restores the window**: the single-instance callback now shows and focuses the
  existing window, so launching the shortcut while it sits in the tray brings it back.
- **Windows CanvasKit fix** (`packages/core/src/canvaskit.ts`,
  `packages/core/src/io/formats/raster/headless.ts`): `new URL(...).pathname` yields `/C:/…` on
  Windows, which CanvasKit cannot open. File URLs are now converted to real Windows paths.
- **Updater feed points at this fork** (`desktop/tauri.conf.json`), and updater artifacts are
  disabled, so a build here never replaces itself with an upstream build that lacks these
  patches.

Desktop settings live in `src/app/settings/preferences/store.ts` (`window.closeToTray`,
`window.minimizeToTray`); the native window handlers stay in step through the
`set_tray_preferences` command.

## Build on Windows (native, no Docker/WSL)

Prerequisites: Node 22+, Bun 1.4+, Rust (MSVC toolchain), Visual Studio Build Tools with the C++
workload, and the Windows SDK.

```powershell
git clone https://github.com/moesuito/open-pencil.git
cd open-pencil
bun install --linker=hoisted
bun run generate:icons --target desktop
bun run generate:tauri-menu
bun run build:packages
bun x vite build
bun x tauri build --config '{"build":{"beforeBuildCommand":""}}' --bundles nsis
```

`--config` skips the upstream `beforeBuildCommand` because its `bun run build` runs the full
lint, and the release tag has pre-existing lint errors in `tests/e2e/perf/pixel-grid.spec.ts`.
The individual build steps above produce the same frontend.

The installer is written to
`desktop/target/release/bundle/nsis/OpenPencil_0.16.0_x64-setup.exe`.

Install or upgrade silently:

```powershell
Start-Process -Wait 'desktop\target\release\bundle\nsis\OpenPencil_0.16.0_x64-setup.exe' -ArgumentList '/S'
```

## Autostart and MCP

After installing, enable **Settings → General → Desktop → Start with Windows** (or the tray
menu). The Run key value is:

```
"C:\Users\<you>\AppData\Local\OpenPencil\OpenPencil.exe" --hidden
```

The app spawns `openpencil-mcp-http` (installed globally from npm) and writes the discovery file
to `%LOCALAPPDATA%\OpenPencil\mcp.json`; `openpencil-mcp` stdio clients such as Claude Code
connect through it. No extra service or Task Scheduler entry is needed.

## Updating from upstream

The patch lives on the `feat/tray-autostart` branch, based on `v0.16.0`:

```powershell
git fetch upstream --tags
git switch feat/tray-autostart
git rebase v0.17.0        # or the newest tag
# resolve conflicts, then rebuild with the commands above
```

Keep `desktop/Cargo.toml` version, `desktop/tauri.conf.json` version, `package.json` version and
the globally installed `@open-pencil/mcp` on the same version: the desktop app checks the MCP
version before spawning it.
