// Shamelessly copied from https://zenn.dev/monaqa/articles/2023-04-19-typst-introduction
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
#show strong: set text(font: "Noto Sans JP", fill: red)

// 段落での両端揃えを有効化・行送りの長さを指定
#set par(justify: true, leading: 0.75em)

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
    $
    union.big_(i=1)^infinity A_i in cF
    $
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
    $
    P(union.big_(i=1)^infinity A_i) = sum_(i=1)^infinity P(A_i)
    $
]

$P$ が $(Omega, cF)$ の確率測度のとき、 $(Omega, cF, P)$ を#strong[確率空間]という。

#theorem(kind: "例", title: [一定時間に到着するメールの数])[
  $Omega = {0, 1, 2, dots}$ で、
  $
  P(A) = sum_(omega in A) (lambda^omega) / (omega!) e^(-lambda)
  $
  とすると、これも確率測度になっている（$A$ は強度 $lambda$ の Poisson 過程に従うという）。
]

$Omega$ が加算無限の場合、 $cF = 2^Omega$ を考えておけば問題ない。
$0 <= h(omega) <= 1$, $sum_(omega in Omega) h(omega) = 1$ となるような $h$ を用いて
$P(A) = sum_(omega in A) h(omega)$ とおけば、 $P$ は確率測度となる。
この $h(omega)$ のことを、確率質量関数という。
