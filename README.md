# vstatus [![](https://img.shields.io/crates/v/vstatus.svg)](https://crates.io/crates/vstatus) [![License](https://img.shields.io/badge/license-ISC-blue.svg)](/LICENSE) [![Build status](https://travis-ci.org/Arkanosis/vstatus.svg?branch=master)](https://travis-ci.org/Arkanosis/vstatus)

**vstatus** is a lightning-fast version control status string generator that you can use wherever you want (eg. in your shell prompt: bash, zsh, fish…).

## Current Status

vstatus is still under active design and not yet ready for mainstream usage.

## Design goals

vstatus aims at the following objectives:
* be extremely lightweight, self-contained and easy to deploy;
* be lightning-fast, to avoid slowing down a terminal if used to draw an informative prompt;
* be extensible enough to support multiple VCS (git, mercurial, subversion, fossil, veracity…)
* be customizable enough to support arbitrary formatting options.

## Compilation

Run `cargo build --release` in your working copy.

## Installation

Copy the `vstatus` binary wherever you want.

## Usage

```
Usage: vstatus <format>
       vstatus -h | --help
       vstatus --version

Arguments:
    format      Version control status string format

Options:
    -h, --help  Show this screen.
    --version   Show version.
```

## Contributing and reporting bugs

Contributions are welcome through [GitHub pull requests](https://github.com/Arkanosis/vstatus/pulls).

Please report bugs and feature requests on [GitHub issues](https://github.com/Arkanosis/vstatus/issues).

## License

vstatus is copyright (C) 2018 Jérémie Roquet <jroquet@arkanosis.net> and licensed under the ISC license.
