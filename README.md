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
- Compound Types
  - The Tuple Type
    - 最初に宣言した長さから変更できない
    - タプルの各要素へは `x.0` のように `{変数}.{index}` でアクセスできる
  - The Array Type
    - 他の多くの言語と違って、固定長
    - 可変長な型でvectorというのがあるらしい
    - 型を指定する場合は `[型名; 長さ]` を記述
    - Arrayの長さより大きいindexで要素にアクセスするとruntime errorになる

### 3.3 - Functions

- Statements and Expressions
  - Statementは値を返さない
    - 故に `let x = (let y = 6);` のようにStatementを変数に入れることはできない
    - 他言語にあるような `x = y = 6` のような書き方もできない
  - `{}` で新しいスコープのブロックを作ることができる
    - `{}` ブロックの中は最終的に値を返すので、Expressionであり、Expressionに最後のセミコロンはいらない
    - 故にブロックの最後の行の処理はセミコロンが不要になる
    - `let x = 5;` と `let x = { 5 };` は同じになる
- Functions with Return Values
  - 値を返す関数の処理ブロック内の最終行はExpressionなのでセミコロンは不要

```rust
// 下記の例のそれぞれの x + 1 は
// Expression なのでセミコロン不要

let y = {
    let x = 5;
    x + 1
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

### 3.5 - Control Flow

- `if` Expressions
  - if文の条件式は `bool` でなければならない
    - 暗黙的に `bool` 以外の型を `bool` に変換しないので、明示的に `bool` で条件を書く必要がある
  - `if` は Expression なので、`let` statement の右辺として使用できる
    - `if` Expression が返す値の型は統一されていないといけない
      - `if` で返す値と `else` で返す値が違うとエラー
- Repetition with `loop`
  - `loop` は Expression なので、 `let` statement の右辺として使用できる

```rust
// numberがi32の場合、次のようなエラーになる

error[E0308]: mismatched types
  --> src/main.rs:10:8
   |
10 |     if number {
   |        ^^^^^^ expected bool, found integral variable
   |
   = note: expected type `bool`
              found type `{integer}`
```

## Chapter 4

### 4.1 - What is Ownership?

- Ownership Rules
  - 3つのルール
    1. Rustの各値値には `owner` という変数をもつ
    2. 一度に所有できる `owner` は1つだけ
    3. `owner` がスコープから外れると、その値は削除される
- The `String` Type
  - `String` は mutable にできるが、 `string literal` は immutable でしか扱えない
  - メモリの使い方が違うことによる違い
- Memory and Allocation
  - `string literal` はハードコードされるので、コンパイル時に内容がわかる
  - 故に `string literal` は高速であるが、immutableしか扱えない
  - `String` 型はmutableで扱うことができ、動的に内容を変えることができるが、それ故にコンパイル時に必要なメモリサイズが分からない
  - 変数がScopeから外れると、Rustは `drop` という特別な関数を自動で呼ぶ
  - integerの場合、値を代入した変数を別の変数に代入した時には値のコピーが代入される
    - 値はそれぞれstackにpushされる
  - `String` 型の場合だと、コピーが代入されるわけではない
    - `String` は **ポインタの位置**, **length**, **capacity** のデータをstackに持つ
    - `String` の値自体はheapに保存される
    - 代入時にコピーされるのはstackに持っているデータなので、値自体は同じポインタ位置から参照することになる
    - 代入すると、代入元の変数は参照を失い（無効化され）、それ以降使用できなくなる
    - 代わりに代入先の変数が参照を持つことになる -> 代入元変数が代入先変数に `move` した
    - これによって、スコープを抜ける時に複数変数から参照されているポインタを二重で解放する操作がなくなる
  - `String` 型でコピーを作成する場合は `clone` メソッドを使用する
    - stackのデータだけでなく、heapのデータもコピーされる
    - 場合によっては負荷が高い処理になることを頭に入れて使用すること
  - Stack-Only Data
    - `Integer` などはStackに値が溜まるので `clone` などは不要
    - 代入後も代入元の変数は無効化されず、使用できる
    - これらを `Copy` 特性と呼ぶ
    - `Copy` 特性のある型
      - integer
      - floating point
      - bool
      - char
      - 要素が `Copy` 特性のみで構成される tuple