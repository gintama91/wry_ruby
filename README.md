# WryRuby

WryRuby is a project that provides experimental Ruby bindings for the [Tauri Wry library]https://github.com/tauri-apps/wry). It aims to facilitate using Wry functionality in Ruby applications.

**Note: This PROJECT and README is a work in progress and will be updated later.**

**Please note that the tests in this project are expected to fail in a continuous integration (CI) environment due to the inability to render graphics or open a display in a headless environment.**(currently looking for ways to work this)

## Usage

(TODO: Add usage instructions)

Currently, WryRuby supports the following features:

- **Trayid**: Provides functionality for creating system tray icons.
- **New Window**: Allows creating new windows for your application.
- **Clipboard**: Provides access to the system clipboard.
- **Event Loop**: Allows creating and managing event loops for your application. as of now only new

To see examples of how to use these features in Ruby, please refer to the tests directory in this project.

## Installation

To use WryRuby in your Ruby project, you can add it as a dependency in your Gemfile:

```ruby
gem 'wry_ruby'
```

Then, run the following command to install the gem:

```bash
bundle install
```

Alternatively, you can install it directly using gem:

```bash
gem install wry_ruby
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/gintama91/wry_ruby

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

