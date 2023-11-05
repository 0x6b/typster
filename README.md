# typster

Naive Rust Library which provides a way to work with [Typst](https://typst.app/) document and PDF file programmatically.

## Features

1. Implements thin wrapper of [typstfmt](https://github.com/astrale-sharp/typstfmt) to format a document.
2. Implements a limited subset of [typst-cli](https://github.com/typst/typst/tree/a59666369b946c3a8b62db363659cbfca35f0a26/crates/typst-cli) [v0.9.0](https://github.com/typst/typst/releases/tag/v0.9.0) to produce a PDF document.
3. Implements PDF metadata updater with support for the following:

   | Metadata          | In Acrobat Reader              | In Apple Preview                   |
   |-------------------|--------------------------------|------------------------------------|
   | Title             | Title                          | Title                              |
   | Author            | Author                         | Author                             |
   | Application       | Application _and_ PDF Producer | PDF Producer _and_ Content creator |
   | Subject           | Subject _and_ Description      | Subject                            |
   | Copyright status  | Copyright Status               | (None)                             |
   | Copyright notice  | Copyright Notice               | Yes                                |
   | Keywords          | Keywords                       | Keywords                           |
   | Creation date     | Created                        | Creation date                      |
   | Modification date | Modified                       | Modification date                  |
   | Custom properties | Custom Properties              | (None)                             |

> [!Note]
> - All metadata will be overwritten, not merged.
> - Both creation and modification date are set automatically to the current date _without time information_ which means time is always 0:00 UTC, for some privacy reasons (or my preference.)

You can specify some of them with Typst. As of Typst [v0.9.0](https://github.com/typst/typst/releases/tag/v0.9.0), the following metadata is supported:

- Title
- Author
- Keywords
- Date

See [Document Function – Typst Documentation](https://typst.app/docs/reference/meta/document/#parameters-keywords) for details.

## Crate features

The crate provides the following features to embed additional fonts in the binary for easier deployment. Note that the typst-cli [defaults](https://github.com/typst/typst/blob/0.9/crates/typst-cli/src/fonts.rs#L126-L140) are always embedded.

- `embed_cmu_roman`: [Computer Modern Roman](https://www.fontsquirrel.com/fonts/computer-modern)
- `embed_ia_writer_duo`: [iA Writer Duo](https://github.com/iaolo/iA-Fonts/)
- `embed_noto_sans_jp`: [Noto Sans JP](https://fonts.google.com/noto/specimen/Noto+Sans+JP)
- `embed_noto_serif_jp`: [Noto Serif JP](https://fonts.google.com/noto/specimen/Noto+Serif+JP)
- `embed_additional_fonts`: all of the above

> [!Warning]
> The crate won't search system fonts to ensure the reproducibility. All fonts you need should be explicitly added via `CompileParams.font_paths`.

## Usage

```toml
# Cargo.toml
[dependencies]
typster = { git = "https://github.com/0x6b/typster", tag = "v0.12.0", features = ["embed_additional_fonts"] }
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

## References

- [typst/typst](https://github.com/typst/typst/)
- [astrale-sharp/typstfmt](https://github.com/astrale-sharp/typstfmt)
- [Extensible Metadata Platform (XMP) Specification: Part 1, Data Model, Serialization, and Core Properties](https://github.com/adobe/XMP-Toolkit-SDK/blob/main/docs/XMPSpecificationPart1.pdf), April, 2012
