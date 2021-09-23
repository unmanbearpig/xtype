# xtype

Prints weird unicode characters selected by alias.

`xty` script asks for the alias via `dmenu` and prints it into the current
window.
Requires: `xtype` in PATH, `xdotool`, `dmenu`.

List of characters and their aliases is hardcoded in `src/dict.rs`

### Examples

```sh
$ xtype lambda
λ
```

```sh
$ xtype --list
 ¨  - uml umlaut
 ä  - auml aumlaut
 Ä  - Auml Aumlaut
 ö  - ouml oumlaut
 Ö  - Ouml Oumlaut
 ü  - uuml uumlaut
 Ü  - Uuml Uumlaut
 ÿ  - yuml yumlaut
 Ÿ  - Yuml Yumlaut
 ß  - eszett eszet ss
 ẞ  - Eszett Eszet SS
 €  - eur euro
…
```

### Build
`cargo build --release`

### Install
`cp xty target/release/xtype /usr/local/bin/`

### Test
`cargo test`
