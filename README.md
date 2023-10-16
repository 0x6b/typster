# typster

Naive Rust library for compiling [typst](https://typst.app/) documents.

## Features

- Limited subset of [typst-cli](https://github.com/typst/typst/tree/a59666369b946c3a8b62db363659cbfca35f0a26/crates/typst-cli).
- Support PDF (default) and PNG output. Output format can be determined by file extension.
- Have features to embed additional fonts in the binary for easier deployment. Note that the typst-cli [defaults](https://github.com/typst/typst/blob/a59666369b946c3a8b62db363659cbfca35f0a26/crates/typst-cli/src/fonts.rs#L101-L115) are always embedded.
    - `embed_noto_sans_jp`: [Noto Sans JP](https://fonts.google.com/noto/specimen/Noto+Sans+JP)
    - `embed_noto_serif_jp`: [Noto Serif JP](https://fonts.google.com/noto/specimen/Noto+Serif+JP)
    - `embed_ia_writer_duo`: [iA Writer Duo](https://github.com/iaolo/iA-Fonts/)
    - `embed_additional_fonts`: all of the above

## Usage

```toml
# Cargo.toml
[dependencies]
typster = { git = "https://github.com/0x6b/typster", version = "0.3.1", features = ["embed_additional_fonts"] }
```

See [examples/main.rs](examples/main.rs) for an example of how to use the library.

```console
$ cargo run --example main --features embed_additional_fonts
```

## License

- The crate itself is licensed under the Apache License version 2.0, as same as [typst](https://github.com/typst/typst/). See [LICENSE](LICENSE) for details.
- Fonts in the [assets/](assets) directory are licensed under its own license.

  | Fonts                        | License                                                                                                                 |
  |------------------------------|-------------------------------------------------------------------------------------------------------------------------|
  | `assets/DejaVuSansMono*.ttf` | [LICENSE](https://github.com/dejavu-fonts/dejavu-fonts/blob/9b5d1b2ffeec20c7b46aa89c0223d783c02762cf/LICENSE)           |
  | `assets/LinLibertine_*.ttf`  | [LICENSE](https://linuxlibertine.sourceforge.net/Libertine-EN.html#licence)                                             |
  | `assets/NewCM*.otf`          | [LICENSE](https://ctan.org/tex-archive/fonts/newcomputermodern)                                                         |
  | `assets/NotoSansJP-*.ttf`    | [LICENSE](https://fonts.google.com/noto/specimen/Noto+Sans+JP/about)                                                    |
  | `assets/NotoSerifJP-*.otf`   | [LICENSE](https://fonts.google.com/noto/specimen/Noto+Serif+JP/about)                                                   |
  | `assets/iAWriterDuoS-*.ttf`  | [LICENSE](https://github.com/iaolo/iA-Fonts/blob/f32c04c3058a75d7ce28919ce70fe8800817491b/iA%20Writer%20Duo/LICENSE.md) |
