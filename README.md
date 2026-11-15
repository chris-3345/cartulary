<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/Wordmarks/Cartulary-Wordmark-TM-DarkColors.svg">
    <source media="(prefers-color-scheme: light)" srcset="media/Wordmarks/Cartulary-Wordmark-TM-LightColors.svg">
    <img alt="Cartulary" src="media/Wordmarks/Cartulary-Wordmark-TM-DarkColors.svg" width="420">
  </picture>
</p>

<p align="center">
  <b>A personal game journal, like GoodReads, but for games.</b><br>
  Fully offline. Your library stays on your computer.
</p>

<p align="center">
  <a href="https://github.com/chris-3345/cartulary/releases/latest">Download</a> ·
  <a href="#building-from-source">Build from source</a> ·
  <a href="#license">License</a>
</p>

---

## What it does

Cartulary keeps track of the games you're playing, have finished, or want to play.

- **Library:** all your games as cards, showing platform, store, status, and rating (wowie!)
- **Entries:** title, year, platform, store, status, a half-star rating out of 5, start/finish dates, and free-form notes (I know it's amazing)
- **Inline editing:** open any game to edit it, then save or revert your changes
- **Offline by design:** no accounts, no sync, and no telemetry because I'm too lazy to do that and don't have servers :)

> [!NOTE]
> Cartulary is early software (0.x). It works, I think. Just back up your library.

## Installing

Grab the latest build from the [Releases page](https://github.com/chris-3345/cartulary/releases/latest):

| Platform | File |
|---|---|
| Windows | `.exe` installer |
| macOS (Apple Silicon) | `.dmg` |
| Linux | `.AppImage` |

The builds aren't signed so your OS will warn you the first time:

- **Windows:** SmartScreen says "Windows protected your PC". Click **More info → Run anyway**.
- **macOS:** the app is blocked completely until you run a terminal command (you'll get a warning about the app being "damaged"). Open your terminal and run:
`xattr -dr com.apple.quarantine /Applications/Cartulary.app`
- **Linux:** mark the AppImage as executable (`chmod +x Cartulary*.AppImage`) and run it.

## Data storage

Your library is a single file, `library.json`:

| OS | Location |
|---|---|
| Windows | `%APPDATA%\chris_3345\Cartulary\data\library.json` |
| macOS | `~/Library/Application Support/io.github.chris_3345.Cartulary/library.json` |
| Linux | `~/.local/share/cartulary/library.json` |

To back up your library, copy that file. If the file is ever unreadable, Cartulary moves it aside (as `library_BAD_<id>.json`) instead of overwriting it.

## Building from source

You'll need a recent stable [Rust toolchain](https://rustup.rs).

```sh
git clone https://github.com/chris-3345/cartulary
cd cartulary
cargo run --release
```

To build installers locally, use [cargo-packager](https://github.com/crabnebula-dev/cargo-packager):

```sh
cargo install cargo-packager --locked
cargo packager --release --formats nsis      # Windows
cargo packager --release --formats dmg       # macOS
cargo packager --release --formats appimage  # Linux
```

Output goes to `dist/`. Release builds for all three platforms are also built automatically by GitHub Actions whenever a release is published.

Built with [Rust](https://www.rust-lang.org) and [Iced](https://iced.rs).
(👁️❤️🦀🧊)

## Roadmap/To do
[ ] Fix icon being cut off on macOS
[ ] Add library export/import and sorting (alphabetical)
[ ] Add Settings page with theme changer
[ ] Import game artwork (user upload or from API)

## License

- **Code:** [AGPL-3.0-or-later](LICENSE)
- **Logos, wordmarks, and icons** (everything in [`media/`](media)): [CC BY-SA 4.0](media/LICENSE)
- **"Cartulary" and the Cartulary logo are trademarks.** You're welcome to fork the code, but forks need their own name and logo. See the [trademark policy](media/TRADEMARKS) for details.

Third-party license notices are available in the app under **Third Party Licenses**.
If you are a maintainer of one of the crates we use and we forgot to add your license to CREDITS.txt/the app menu, please open an Issue.
