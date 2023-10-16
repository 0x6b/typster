# typster

Naive Rust library for compiling [typst](https://typst.app/) documents.

## Features

- Limited subset of [typst-cli](https://github.com/typst/typst/tree/a59666369b946c3a8b62db363659cbfca35f0a26/crates/typst-cli)
- Support PDF (default) and PNG output. Output format can be determined by file extension.
- Have features to embed additional fonts in the binary for easier deployment. Note that the typst-cli defaults are always embedded.
    - `embed_noto_sans_jp`: [Noto Sans JP](https://fonts.google.com/noto/specimen/Noto+Sans+JP)
    - `embed_noto_serif_jp`: [Noto Serif JP](https://fonts.google.com/noto/specimen/Noto+Serif+JP)
    - `embed_ia_writer_duo`: [iA Writer Duo](https://github.com/iaolo/iA-Fonts/)
    - `embed_additional_fonts`: all of the above

## Usage

```console
$ cargo run --example main --features embed_additional_fonts
```

See [examples/main.rs](examples/main.rs) for an example of how to use the library.

## License

- The crate itself is licensed under the Apache License version 2.0, as same as [typst](https://github.com/typst/typst/). See [LICENSE](LICENSE) for details.
- Fonts in the [assets/](assets) directory are licensed under its own license.
    - [`DejaVuSansMono-Bold.ttf`](assets/DejaVuSansMono-Bold.ttf), [`DejaVuSansMono-BoldOblique.ttf`](assets/DejaVuSansMono-BoldOblique.ttf), [`DejaVuSansMono-Oblique.ttf`](assets/DejaVuSansMono-Oblique.ttf), [`DejaVuSansMono.ttf`](assets/DejaVuSansMono.ttf): See [LICENSE](https://github.com/dejavu-fonts/dejavu-fonts/blob/9b5d1b2ffeec20c7b46aa89c0223d783c02762cf/LICENSE) for details
    - [`LinLibertine_R.ttf`](assets/LinLibertine_R.ttf), [`LinLibertine_RB.ttf`](assets/LinLibertine_RB.ttf), [`LinLibertine_RBI.ttf`](assets/LinLibertine_RBI.ttf), [`LinLibertine_RI.ttf`](assets/LinLibertine_RI.ttf): See [LICENSE](https://linuxlibertine.sourceforge.net/Libertine-EN.html#licence) for details
    - [`NewCM10-Bold.otf`](assets/NewCM10-Bold.otf), [`NewCM10-BoldItalic.otf`](assets/NewCM10-BoldItalic.otf), [`NewCM10-Italic.otf`](assets/NewCM10-Italic.otf), [`NewCM10-Regular.otf`](assets/NewCM10-Regular.otf), [`NewCMMath-Book.otf`](assets/NewCMMath-Book.otf), [`NewCMMath-Regular.otf`](assets/NewCMMath-Regular.otf): See [LICENSE](https://ctan.org/tex-archive/fonts/newcomputermodern) for details
    - [`NotoSansJP-Black.ttf`](assets/NotoSansJP-Black.ttf), [`NotoSansJP-Bold.ttf`](assets/NotoSansJP-Bold.ttf), [`NotoSansJP-ExtraBold.ttf`](assets/NotoSansJP-ExtraBold.ttf), [`NotoSansJP-ExtraLight.ttf`](assets/NotoSansJP-ExtraLight.ttf), [`NotoSansJP-Light.ttf`](assets/NotoSansJP-Light.ttf), [`NotoSansJP-Medium.ttf`](assets/NotoSansJP-Medium.ttf), [`NotoSansJP-Regular.ttf`](assets/NotoSansJP-Regular.ttf), [`NotoSansJP-SemiBold.ttf`](assets/NotoSansJP-SemiBold.ttf), [`NotoSansJP-Thin.ttf`](assets/NotoSansJP-Thin.ttf): See [LICENSE](https://fonts.google.com/noto/specimen/Noto+Sans+JP/about) for details
    - [`NotoSerifJP-Black.otf`](assets/NotoSerifJP-Black.otf), [`NotoSerifJP-Bold.otf`](assets/NotoSerifJP-Bold.otf), [`NotoSerifJP-ExtraLight.otf`](assets/NotoSerifJP-ExtraLight.otf), [`NotoSerifJP-Light.otf`](assets/NotoSerifJP-Light.otf), [`NotoSerifJP-Medium.otf`](assets/NotoSerifJP-Medium.otf), [`NotoSerifJP-Regular.otf`](assets/NotoSerifJP-Regular.otf), [`NotoSerifJP-SemiBold.otf`](assets/NotoSerifJP-SemiBold.otf): See [LICENSE](https://fonts.google.com/noto/specimen/Noto+Serif+JP/about) for details
    - [`iAWriterDuoS-Bold.ttf`](assets/iAWriterDuoS-Bold.ttf), [`iAWriterDuoS-BoldItalic.ttf`](assets/iAWriterDuoS-BoldItalic.ttf), [`iAWriterDuoS-Italic.ttf`](assets/iAWriterDuoS-Italic.ttf), [`iAWriterDuoS-Regular.ttf`](assets/iAWriterDuoS-Regular.ttf): See [LICENSE](https://github.com/iaolo/iA-Fonts/blob/f32c04c3058a75d7ce28919ce70fe8800817491b/iA%20Writer%20Duo/LICENSE.md) for details
