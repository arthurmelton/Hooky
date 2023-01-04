<p align="center">
  <img src="https://github.com/AMTitan/Hooky/raw/master/assets/hooky.png" alt="Hooky" style="width: 250px;">
</p>

### Requirements (for controller)

- For Windows
  - [Build Tools for Visual Studio 2022](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and select "Desktop development with C++"
  - If you are not on Windows 11
    - Install Evergreen Bootstrapper from [Microsoft](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section).
- For Mac
  - Open the terminal and run `xcode-select --install`
- [Rust](https://www.rust-lang.org/tools/install)
- Ability to open ports

### Running

Download the `zip`/`tar` of the latest build at the [Releases](https://github.com/AMTitan/Hooky/releases) tab. Unzip the folder and put it somewhere. Open a terminal in the folder called `command` and run.

```sh 
rustup target add wasm32-unknown-unknown
cargo install tauri-cli trunk
cargo tauri build -b none
```

Now go to the folder `target` then `release`, and run the program called `command`
