# typster

Naive Rust Library which provides a way to work with [Typst](https://typst.app/) document and PDF file programmatically.

## Features

1. Implements thin wrapper of [typstfmt](https://github.com/astrale-sharp/typstfmt) to format a document.
2. Implements limited subset of [typst-cli](https://github.com/typst/typst/tree/a59666369b946c3a8b62db363659cbfca35f0a26/crates/typst-cli) to produce a PDF document.
3. Implements PDF metadata updater with support of following metadata:
    
    | Metadata         | In Acrobat Reader         | In Apple Preview             |
    |------------------|---------------------------|------------------------------|
    | Title            | Title                     | Title                        |
    | Author           | Author                    | Author _and_ Content creator |
    | Application      | Application               | PDF Producer                 |
    | Subject          | Subject _and_ Description | Subject                      |
    | Copyright status | Copyright Status          | (None)                       |
    | Copyright notice | Copyright Notice          | Yes                          |
    | Keywords         | Keywords                  | (None)                       |

## Crate features

The crate provides following features to embed additional fonts in the binary for easier deployment. Note that the typst-cli [defaults](https://github.com/typst/typst/blob/a59666369b946c3a8b62db363659cbfca35f0a26/crates/typst-cli/src/fonts.rs#L101-L115) are always embedded.

- `embed_cmu_roman`: [Computer Modern Roman](https://www.fontsquirrel.com/fonts/computer-modern)
- `embed_ia_writer_duo`: [iA Writer Duo](https://github.com/iaolo/iA-Fonts/)
- `embed_noto_sans_jp`: [Noto Sans JP](https://fonts.google.com/noto/specimen/Noto+Sans+JP)
- `embed_noto_serif_jp`: [Noto Serif JP](https://fonts.google.com/noto/specimen/Noto+Serif+JP)
- `embed_additional_fonts`: all of the above

## Usage

```toml
# Cargo.toml
[dependencies]
typster = { git = "https://github.com/0x6b/typster", version = "0.7.0", features = ["embed_additional_fonts"] }
```

### Compiling a document

See [`examples/compile.rs`](examples/compile.rs).

```console
$ cargo run --example compile --features embed_additional_fonts
```

### Formatting a document

See [`examples/format.rs`](examples/format.rs).

```console
$ cargo run --example format
```

### Updating PDF metadata

See [`examples/update_metadata.rs`](examples/update_metadata.rs).

```console
$ cargo run --example update_metadata
```

## License

- The crate itself is licensed under the Apache License version 2.0, as same as [typst](https://github.com/typst/typst/). See [LICENSE](LICENSE) for details.
- Fonts in the [assets/](assets) directory are licensed under its own license.

  | Fonts                        | License                                                                                                                 |
      |------------------------------|-------------------------------------------------------------------------------------------------------------------------|
  | `assets/cmunrm.ttf`          | [LICENSE](https://www.fontsquirrel.com/fonts/computer-modern)                                                           |
  | `assets/DejaVuSansMono*.ttf` | [LICENSE](https://github.com/dejavu-fonts/dejavu-fonts/blob/9b5d1b2ffeec20c7b46aa89c0223d783c02762cf/LICENSE)           |
  | `assets/LinLibertine_*.ttf`  | [LICENSE](https://linuxlibertine.sourceforge.net/Libertine-EN.html#licence)                                             |
  | `assets/NewCM*.otf`          | [LICENSE](https://ctan.org/tex-archive/fonts/newcomputermodern)                                                         |
  | `assets/NotoSansJP-*.ttf`    | [LICENSE](https://fonts.google.com/noto/specimen/Noto+Sans+JP/about)                                                    |
  | `assets/NotoSerifJP-*.otf`   | [LICENSE](https://fonts.google.com/noto/specimen/Noto+Serif+JP/about)                                                   |
  | `assets/iAWriterDuoS-*.ttf`  | [LICENSE](https://github.com/iaolo/iA-Fonts/blob/f32c04c3058a75d7ce28919ce70fe8800817491b/iA%20Writer%20Duo/LICENSE.md) |
