# rust-learn

Rust book (2018 Edition) の学習用リポジトリ  

- https://doc.rust-lang.org/book/2018-edition/foreword.html

---

## 学習メモ

## Chapter 3

### 3.1 - Variables and Mutability

- 変数は基本Immutable
- ImmutableとMutableのトレードオフ
  - 大きなデータ構造の場合はインスタンスのコピーを生成するよりもインスタンスを変更した方が早い
  - 小さなデータ構造の場合は新しいインスタンスを生成して、関数型な書き方で書いた方がわかりやすい
- Constantはどのスコープでも宣言可能
- Shadowingで同じ名前の変数を再宣言できる
- Shadowingで再宣言した変数は元の変数とは別の型として宣言可能

### 3.2 - Data Types

- Scalar Types
  - Integer Types
    - `8-bit`, `16-bit`, `32-bit`, `64-bit`, `128-bit`, `arch` の長さでそれぞれsignedとunsignedがある
    - signedは `i` から、unsignedは `u` から始まる
      - unsigned 32-bitの場合 `u32`
    - signedの長さ(nはbit数)
      - -(2<sup>n-1</sup>) ~ 2<sup>n-1</sup> - 1
    - unsignedの長さ(nはbit数)
      - 2<sup>n</sup> - 1
    - `arch` (`isize` と `usize`) は実行環境に依存する
    - 3桁ずつ `_` で区切って表現可能 (`10000` と `10_000` は同じ)
    - `0x` から始まるHex表記, `0o` から始まるOctal表記, `0b` から始まるBinary表記などが可能
    - デフォルトは `i32` 基本的にはこれを使うのが速い
  - Floating Point Type
    - `f64` (64bit)と `f32` (32bit)がある
  - Character Types
    - シングルクオートで囲まれる
    - Unicode Scalar Valueなので、ASCII以外にも日本語などを表すこともできる