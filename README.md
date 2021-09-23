# xtype

Prints weird unicode characters selected by alias.

`xty` script asks for the alias via `dmenu` and prints it into the current
window.
Requires: `xtype` in PATH, `xdotool`, `dmenu`.

List of characters and their aliases is hardcoded in `src/dict.rs`

### Build
`cargo build --release`

### Install
`cp xty target/release/xtype /usr/local/bin/`

### Test
`cargo test`
