# WryRuby

[![License](https://img.shields.io/github/license/gintama91/wry_ruby)](https://opensource.org/licenses/MIT)
[![Issues](https://img.shields.io/github/issues/gintama91/wry_ruby)](https://github.com/gintama91/wry_ruby/issues)
[![Stars](https://img.shields.io/github/stars/gintama91/wry_ruby)](https://github.com/gintama91/wry_ruby/stargazers)

## Overview

WryRuby is an **EXPERIMENTAL** Ruby project that provides ruby bindings for [Tauri's Wry library](https://github.com/tauri-apps/wry).

**Please Note: This project is not ready to use yet**

## 🚀 Features

- **`Trayid`**: Simplified creation of system tray icons.
- **`New Window`**: Easy window creation for your Ruby application.
- **`Clipboard`**: Access to the system clipboard with convenience.
- **`Event Loop`**: Create and manage application event loops. *(Currently, only `new` is supported)*.
- **`window_with_html`**: Create new windows with HTML content.
- **`load_with_url`**:loads a URL into the window.


## 📝 Usage

For examples and usage instructions, please refer to the `tests` directory or `examples` directory.

## 💻 Installation

To get started with WryRuby, you can install it using:

```bash
gem install wry_ruby
```

### 🛠️ Development Setup

To contribute to the development of WryRuby, ensure you have Ruby and Rust installed on your system. Refer to the `.rust-version` and `.ruby-version` files for specific version requirements.

Install the dependencies based on your platform from [here](https://github.com/tauri-apps/wry#platform-specific-notes)

```bash
git clone https://github.com/gintama91/wry_ruby.git
bundle install
```

To run examples:

```bash
ruby examples/new_window.rb # Replace with the specific example name
```

## Contributing ❤️

We welcome bug reports and pull requests on our [GitHub repository](https://github.com/gintama91/wry_ruby). Join us in improving and expanding the capabilities of WryRuby.

## License 📜

WryRuby is an open-source project licensed under the [MIT License](https://opensource.org/licenses/MIT). Feel free to use it and contribute to the community.
