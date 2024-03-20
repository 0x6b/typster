#[cfg(feature = "compile")]
pub use compile::{compile, CompileParams};
#[cfg(feature = "compile")]
pub use fonts::{list_fonts, FontInformation, FontVariant};
#[cfg(feature = "format")]
pub use format::{format, FormatParams};
#[cfg(feature = "pdf_permission")]
pub use set_permission::{set_permission, PermissionParams, PrintPermission};
#[cfg(feature = "pdf_metadata")]
pub use update_metadata::{update_metadata, PdfMetadata};
pub use version::{typst_version, version};
#[cfg(feature = "watch")]
pub use watch::watch;

#[cfg(feature = "compile")]
mod compile;
#[cfg(feature = "compile")]
mod download;
#[cfg(feature = "compile")]
mod fonts;
#[cfg(feature = "format")]
mod format;
#[cfg(feature = "compile")]
mod package;
#[cfg(feature = "pdf_permission")]
mod set_permission;
#[cfg(feature = "pdf_metadata")]
mod update_metadata;
mod version;
#[cfg(feature = "watch")]
mod watch;
#[cfg(feature = "compile")]
mod world;

#[cfg(test)]
pub mod tests {
    use std::{
        collections::HashMap,
        error::Error,
        fs::{remove_file, File},
        io::copy,
        path::{Path, PathBuf},
        process::Command,
    };

    use sha2::{Digest, Sha256};

    use super::*;

    #[test]
    fn test_export_pdf() -> Result<(), Box<dyn Error>> {
        let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample-test-export-pdf.pdf");
        let params = CompileParams {
            input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("examples")
                .join("sample.typ"),
            output: output.clone(),
            font_paths: vec![],
            inputs: vec![("input".to_string(), "value".to_string())],
            ppi: None,
        };

        compile(&params)?;
        assert!(compile(&params).is_ok());
        assert!(&output.exists());
        assert_eq!(
            calculate_hash(&output)?,
            "f9f09e14e1a9906ca327649b94c7958e304f6e66bc1a378abe77c179f3c49cf0"
        );

        remove_file(&output)?;

        Ok(())
    }

    #[test]
    fn test_export_png() -> Result<(), Box<dyn Error>> {
        let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample-test-export-png.png");
        let params = CompileParams {
            input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("examples")
                .join("sample.typ"),
            output: output.clone(),
            font_paths: vec![],
            inputs: vec![("input".to_string(), "value".to_string())],
            ppi: None,
        };

        assert!(compile(&params).is_ok());
        assert!(&output.exists());
        assert_eq!(
            calculate_hash(&output)?,
            "6e75034f19b9046f4f304973e6371cfbce2c090c056e521ae3dad7553777fc10"
        );

        remove_file(&output)?;

        Ok(())
    }

    #[test]
    fn test_format() -> Result<(), Box<dyn Error>> {
        let params = FormatParams {
            input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("examples")
                .join("sample.typ"),
            config: None,
        };
        assert_eq!(
            format(&params)?,
            r#"// Shamelessly copied from https://zenn.dev/monaqa/articles/2023-04-19-typst-introduction
// Thank you!

#set document(
  title: "確率論の基礎",
  author: "typster",
  keywords: "確率論, 確率空間, 確率測度, 確率質量関数, 可測空間, 可測集合, 事象, Event, 確率, 定理, 定義, 例",
  date: auto,
)

// --------- ちょっとした設定 ---------

// フォント周り
#set text(font: "Noto Serif JP")
#show emph: set text(font: "Noto Sans JP")
#show strong: set text(
  font: "Noto Sans JP",
  fill: red,
)

// 段落での両端揃えを有効化・行送りの長さを指定
#set par(
  justify: true,
  leading: 0.75em,
)

// 箇条書きと別行立て数式の設定
#set list(indent: 0.5em)
#set enum(numbering: "(1)")
#set math.equation(numbering: "(1)")

// theorem 用カウンタの定義
#let theorem-number = counter("theorem-number")

// theorem 関数の定義。コマンドみたいに使える
#let theorem(title: none, kind: "定理", body) = {
  let title-text = {
    if title == none {
      emph[#kind 2.#theorem-number.display()]
    } else {
      emph[#kind 2.#theorem-number.display() 【#title】]
    }
  }

  box(stroke: (left: 1pt), inset: (left: 5pt, top: 2pt, bottom: 5pt))[
    #title-text #h(0.5em)
    #body
  ]

  theorem-number.step()
}

// 数式で用いるエイリアス（$\mathcal{F}$ 的なやつ）
#let cF = $cal(F)$

// 以降のテキストで現れる句読点を全角カンマピリオドに置換する。そんなこともできるの…
#show "、": "，"
#show "。": "．"

// --------- ここから本文のマークアップ ---------

#theorem(kind: "定義", title: [$sigma$-加法族])[
  $Omega$ の部分集合族 $cF$ が以下の性質を満たすとき、 $Omega$ を $sigma$-加法族という。

  + $Omega in cF$
  + $A in cF ==> A^c in cF$
  + $A_1, A_2, dots in cF$ に対して以下のことが成り立つ（_$sigma$-加法性、完全加法性、加算加法性_）:
    $ union.big_(i=1)^infinity A_i in cF $
]

$A subset Omega$ に「確率」を定めたい。矛盾なく「確率」が定まる集合をあらかじめ決めておきたい。
それが $sigma$-加法族である。
$Omega$ と $cF$ の組 $(Omega, cF)$ を#strong[可測空間]という。
また、$cF$ の元を#strong[可測集合]（または事象、Event）という。

#theorem(kind: "定義", title: [確率測度])[
  $(Omega, cF)$ を可測空間とする。 $cF$ 上の関数 $P$ が次を満たすとき、これを#strong[確率測度]という。

  - $0 <= P(A) <= 1 #h(0.5em) (forall A in cF)$
  - $P(Omega) = 1$
  - $A_1, A_2, dots in cF$ が $A_i sect A_j = nothing #h(0.25em) (forall i != j)$ のとき、
    次が成り立つ（$sigma$-加法性、完全加法性）:
    $ P(union.big_(i=1)^infinity A_i) = sum_(i=1)^infinity P(A_i) $
]

$P$ が $(Omega, cF)$ の確率測度のとき、 $(Omega, cF, P)$ を#strong[確率空間]という。

#theorem(kind: "例", title: [一定時間に到着するメールの数])[
  $Omega = {0, 1, 2, dots}$ で、
  $ P(A) = sum_(omega in A) (lambda^omega) / (omega!) e^(-lambda) $
  とすると、これも確率測度になっている（$A$ は強度 $lambda$ の Poisson 過程に従うという）。
]

$Omega$ が加算無限の場合、 $cF = 2^Omega$ を考えておけば問題ない。
$0 <= h(omega) <= 1$, $sum_(omega in Omega) h(omega) = 1$ となるような $h$ を用いて
$P(A) = sum_(omega in A) h(omega)$ とおけば、 $P$ は確率測度となる。
この $h(omega)$ のことを、確率質量関数という。"#
        );

        Ok(())
    }

    #[test]
    fn test_update_metadata() -> Result<(), Box<dyn std::error::Error>> {
        let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample-test-update-metadata.pdf");
        let params = CompileParams {
            input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("examples")
                .join("sample.typ"),
            output: output.clone(),
            font_paths: vec![],
            inputs: vec![("input".to_string(), "value".to_string())],
            ppi: None,
        };
        assert!(compile(&params).is_ok());

        let mut custom_properties = HashMap::new();
        custom_properties.insert("robots".to_string(), "noindex".to_string());
        custom_properties.insert("custom".to_string(), "properties".to_string());

        let metadata = PdfMetadata {
            title: "Title (typster)".to_string(),
            author: "Author (typster)".to_string(),
            application: "Application (typster)".to_string(),
            subject: "Subject (typster)".to_string(),
            copyright_status: true,
            copyright_notice: "Copyright notice (typster)".to_string(),
            keywords: vec!["typster".to_string(), "rust".to_string(), "pdf".to_string()],
            language: "en".to_string(),
            custom_properties,
        };

        assert!(update_metadata(&output, &metadata).is_ok());
        assert!(&output.exists());
        assert!(output.metadata()?.len() > 0);

        let props = get_properties(&output)?;
        assert_eq!(props.get("Title"), Some(&"Title (typster)".to_string()));
        assert_eq!(props.get("Author"), Some(&"Author (typster)".to_string()));
        assert_eq!(props.get("Creator"), Some(&"Application (typster)".to_string()));
        assert_eq!(props.get("Producer"), Some(&"Application (typster)".to_string()));
        assert_eq!(props.get("Creator Tool"), Some(&"Application (typster)".to_string()));
        assert_eq!(props.get("Subject"), Some(&"Subject (typster)".to_string()));
        assert_eq!(props.get("Marked"), Some(&"True".to_string()));
        assert_eq!(props.get("Rights"), Some(&"Copyright notice (typster)".to_string()));
        assert_eq!(props.get("Keywords"), Some(&"typster, rust, pdf".to_string()));
        assert_eq!(props.get("Language"), Some(&"en".to_string()));
        assert_eq!(props.get("Robots"), Some(&"noindex".to_string()));
        assert_eq!(props.get("Custom"), Some(&"properties".to_string()));

        remove_file(&output)?;

        Ok(())
    }

    #[test]
    fn test_set_permission() -> Result<(), Box<dyn Error>> {
        let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample-test-set-permission.pdf");
        let output_protected = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample-test-set-permission-protected.pdf");
        let params = CompileParams {
            input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("examples")
                .join("sample.typ"),
            output: output.clone(),
            font_paths: vec![],
            inputs: vec![("input".to_string(), "value".to_string())],
            ppi: None,
        };
        assert!(compile(&params).is_ok());

        assert!(set_permission(
            output.clone(),
            output_protected.clone(),
            &PermissionParams {
                owner_password: "owner".to_string(),
                allow_print: PrintPermission::None,
                ..Default::default()
            },
        )
        .is_ok());
        assert!(&output_protected.exists());
        assert!(output_protected.metadata()?.len() > 0);

        let props = get_properties(&output_protected)?;
        assert!(props.get("Encryption").is_some());
        assert_eq!(props.get("User Access"), Some(&"Copy, Annotate, Extract".to_string()));

        // since set_permission embeds time, we can't compare the file hash

        remove_file(&output)?;
        remove_file(&output_protected)?;

        Ok(())
    }

    #[test]
    fn test_version() -> Result<(), Box<dyn Error>> {
        assert_eq!(typst_version(), "0.11.0");
        Ok(())
    }

    fn calculate_hash(path: &Path) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        copy(&mut file, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn get_properties(path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let out = String::from_utf8(Command::new("exiftool").arg(path).output()?.stdout)?;
        let props = out
            .split('\n')
            .map(|line| line.split(':'))
            .filter_map(|mut line| {
                let key = line.next().unwrap_or_default().trim().to_string();
                let value = line.next().unwrap_or_default().trim().to_string();
                if !key.is_empty() {
                    Some((key, value))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        Ok(props)
    }
}
