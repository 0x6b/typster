# typster

A naive Rust library that provides a way to work with [Typst](https://typst.app/) document and PDF file programmatically.

## Usage

```toml
typster = { git = "https://github.com/0x6b/typster", tag = "vx.x.x", features = ["full"] }
```

## Crate Features

Specify `full` to enable all the following features. Note that embedding fonts will produce a large binary.

### `compile`

You can compile a Typst document to a PDF or a PNG file; a limited subset of [typst-cli](https://github.com/typst/typst/tree/v0.11.0/crates/typst-cli) [v0.11.0](https://github.com/typst/typst/releases/tag/v0.11.0).

See [`examples/compile.rs`](examples/compile.rs) for usage.

```console
$ cargo run --example compile --features embed_additional_fonts
```

### `format`

You can format a Typst document with [typstyle](https://github.com/Enter-tainer/typstyle).

See [`examples/format.rs`](examples/format.rs) for usage.

```console
$ cargo run --example format --features format
```

### `pdf_metadata`

You can update PDF metadata. Following metadata is supported:

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
>
> - All metadata will be overwritten, not merged.
> - Both creation and modification date are set automatically to the current date
    >
    _without
    >
    time
    >
    information_ which means time is always 0:00 UTC, for some privacy reasons (or my preference.)

You can specify some of them with Typst. As of Typst [v0.11.0](https://github.com/typst/typst/releases/tag/v0.11.0), the following metadata is supported:

- Title
- Author
- Keywords
- Date

See [Document Function – Typst Documentation](https://typst.app/docs/reference/model/document/) for details.

See [`examples/update_metadata.rs`](examples/update_metadata.rs) for usage.

```console
$ cargo run --example update_metadata --features pdf_metadata
```

### `pdf_permission`

You can set PDF permission update. Following PDF 1.7 permissions are supported:

- user password, which is required to open the document. Set to `None` to allow anyone to open.
- owner password, which is required to change permissions. Set to `None` to allow anyone to change.
- content copying for accessibility.
- page extraction.
- document assembly.
- commenting and form filling.
- form field fill-in or signing.
- other modifications.
- printing (high, low, or disallow).
- encrypt metadata.

See [`examples/set_permission.rs`](examples/set_permission.rs) for usage.

```console
$ cargo run --example set_permission --features pdf_permission
```

### `watch`

You'll be able to preview your Typst document live. Changes for `typ` file, along with files with extension `cbor`, `csv`, `gif`, `htm`, `html`, `jpeg`, `jpg`, `json`, `png`, `svg`, `toml`, `txt`, `xml`, `yaml`, and `yml` in the same directory, recursively, will be watched. This is inspired by [ItsEthra/typst-live](https://github.com/ItsEthra/typst-live/).

See [`examples/watch.rs`](examples/watch.rs) for usage.

```console
$ cargo run --example watch --features watch
```

This feature also enables `compile` feature.

### Embedding Fonts

You can embed additional fonts in the binary for easier deployment. Each feature also enables `compile` feature.

- `embed_cmu_roman`: [Computer Modern Roman](https://www.fontsquirrel.com/fonts/computer-modern)
- `embed_ia_writer_duo`: [iA Writer Duo](https://github.com/iaolo/iA-Fonts/)
- `embed_noto_sans_jp`: [Noto Sans JP](https://fonts.google.com/noto/specimen/Noto+Sans+JP)
- `embed_noto_serif_jp`: [Noto Serif JP](https://fonts.google.com/noto/specimen/Noto+Serif+JP)
- `embed_recursive`: [Recursive Sans & Mono](https://github.com/arrowtype/recursive/)
- `embed_source_code_pro`: [Source Code Pro](https://fonts.google.com/specimen/Source+Code+Pro)
- `embed_additional_fonts`: all of the above

> [!Note]
> typst-cli [defaults](https://github.com/typst/typst-assets/blob/5ca2a6996da97dcba893247576a4a70bbbae8a7a/src/lib.rs#L67-L80) are always embedded.

> [!Warning]
> The crate won't search system fonts to ensure the reproducibility. All fonts you need should be explicitly added via [`CompileParams.font_paths`](https://github.com/0x6b/typster/blob/main/src/compile.rs#L21).

## Testing

Naive tests are available. You can run them with:

```console
$ cargo test --all-features
```

Note that you have to install `exiftool` to run all tests.

## License

- The crate itself is licensed under the Apache License version 2.0, as same as [Typst](https://github.com/typst/typst/). See [LICENSE](LICENSE) for details.
- Fonts under the [assets/fonts](assets/fonts) directory are licensed under its own license.
    - `assets/fonts/ComputerModern/cmunrm.ttf`: [LICENSE](assets/fonts/ComputerModern/SIL%20Open%20Font%20License.txt)
    - `assets/fonts/iAWriterDuo/iAWriterDuoS-*.ttf`: [LICENSE](assets/fonts/iAWriterDuo/LICENSE.md)
    - `assets/fonts/NotoSansJP/NotoSansJP-*.ttf`: [LICENSE](assets/fonts/NotoSansJP/OFL.txt)
    - `assets/fonts/NotoSerifJP/NotoSerifJP-*.otf`: [LICENSE](assets/fonts/NotoSerifJP/OFL.txt)
    - `assets/fonts/Recursive/recursive-static-OTFs.otc`: [LICENSE](assets/fonts/Recursive/OFL.txt)
    - `assets/fonts/SourceCodePro/SourceCodePro-*.ttf`: [LICENSE](assets/fonts/SourceCodePro/OFL.txt)

## Acknowledgements

- [typst/typst](https://github.com/typst/typst/)
- [Enter-tainer/typstyle](https://github.com/Enter-tainer/typstyle)
- [ItsEthra/typst-live](https://github.com/ItsEthra/typst-live/)
- All the fonts authors and contributors

## Reference

- [Extensible Metadata Platform (XMP) Specification: Part 1, Data Model, Serialization, and Core Properties](https://github.com/adobe/XMP-Toolkit-SDK/blob/main/docs/XMPSpecificationPart1.pdf), April, 2012
