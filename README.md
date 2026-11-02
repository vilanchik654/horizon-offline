# Horizon Offline

A fork of [Horizon](https://github.com/horizon-rust/horizon) — Minecraft: Java Edition 26.3
rewritten in Rust — for people who just want to type a name and play.

The game itself is upstream's, untouched. What changes is the launcher: it plays offline by
default, and it never replaces itself with a build downloaded from GitHub.

> Not an official Minecraft product, nor approved by or associated with Mojang or Microsoft.
> This repository contains none of Minecraft's code, assets or data: the launcher downloads them
> from Mojang's server and prepares them on your machine.

## What this fork changes

| | Upstream | Here |
| --- | --- | --- |
| Starting the game | needs a Microsoft account that owns Java Edition | asks for a name on the first run, then plays — no account, no ownership check |
| Preparing the game | Mojang's files, for an owner | the same public download, no account needed |
| Launcher updates | downloads and replaces itself | never; the launcher you built is the launcher you run |
| Game updates | Versions page, `horizon-launcher update` | unchanged |
| Online-mode servers | Microsoft account | still one (Settings → Account) — offline names can't join those |

The switch lives in Settings → Account ("Play without a Microsoft account", on by default) and in
`settings.json` as `offline_play`. Turning it off brings the old gate back.

What this fork touched: `launcher/src/name_sheet.rs` (the first-run name sheet),
`launcher/src/switch_page.rs` (the switch), `launcher/src/account.rs` and `launcher/src/backend.rs`
(`offline_play`, the download gate, and the removal of `backend::self_update`, `mac_update.rs` and
the launcher half of `horizon-launcher update`).

## Build

Rust is all it needs. Rust's MSVC toolchain should work too, but this build was made and tested
with its GNU target and MSYS2's gcc:

```powershell
# once: Rust (x86_64-pc-windows-gnu, https://rustup.rs) and MSYS2 (https://www.msys2.org,
# then: pacman -S mingw-w64-ucrt-x86_64-gcc)

$env:PATH="$env:USERPROFILE\.cargo\bin;C:\msys64\ucrt64\bin;$env:PATH"
cd launcher
cargo build --release
```

The binary lands in `launcher/target/release/horizon-launcher.exe` — or under
`$env:CARGO_TARGET_DIR\release\` when you set one, which is worth doing: the dependency graph is
about 15 GB. A rebuild after a source change takes five minutes; from scratch, roughly half an
hour.

On Linux, install `libasound2-dev` and `libudev-dev` first and run the same `cargo build
--release`. On macOS, build from a Mac — the launcher wraps itself in a `.app`.

## First start

1. Run the launcher. It asks what to call you — your computer's user name is filled in — and
   starts the game when you press Play.
2. That first Play downloads Minecraft 26.3 from Mojang (about 2 GB with the assets) and the Java
   it needs, then prepares the game's files. Later starts skip all of it.
3. Worlds, options and prepared files live where the launcher keeps them
   (`%APPDATA%\horizon-launcher` on Windows) — the same folders any other build of the launcher
   uses.

The launcher installs the game from upstream's releases, so the game you play is the official
Horizon build. Only the launcher is this fork's.

## Limits

- No launcher self-update: to pick up changes here, rebuild; to pick up upstream's launcher work,
  merge from upstream.
- Online-mode servers, Realms and account skins need a Microsoft account that owns the game.
  Offline play covers single player, LAN and offline-mode servers.
- Built and tested on Windows x64. Linux and macOS should build from the same sources, but this
  fork has shipped no binaries for them.

## Credits and licence

Horizon is MIT work by Ludovic Marie and the Horizon contributors: the original project, its
parity harnesses and everything under `crates/`, `docs/`, `mods/` and `tools/` are theirs. This
fork starts from upstream commit `41501a243` and keeps `LICENSE` and `THIRD_PARTY_NOTICES.md` as
they were. [FORK.txt](FORK.txt) has the short version of what changed.
