# Alinio
> A library to assist in alignment and table generation in TUI applications 

This library contains utilties to align text, and build tables.
It contains a very powerful table struct that allows creation of responsive, padded and 
aligned tables that make creating complicated interfaces quite simple.

## Installation
Go ahead and add in `alinio = "0"` into your `Cargo.toml` under `[dependencies]`.

## Usage example
You can find a few examples under the documentation over on https://docs.rs/alinio

I've made sure to properly document this library, but please let me know if you don't understand anything.

## Release History

* 0.2.0
    * Added `partial_render` function to allow for rendering only part of the table
* 0.1.1
    * Fixed panic when there is not enough space to render the table when surround is `false`
* 0.1.0
    * Inital release

Distributed under the MIT license. See ``LICENSE`` for more information.
