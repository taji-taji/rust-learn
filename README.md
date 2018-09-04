# rust-learn

Rust book (2018 Edition) の学習用リポジトリ  

- https://doc.rust-lang.org/book/2018-edition/foreword.html

---

# 学習メモ

# Chapter 3

## 3.1 - Variables and Mutability

- 変数は基本Immutable
- ImmutableとMutableのトレードオフ
  - 大きなデータ構造の場合はインスタンスのコピーを生成するよりもインスタンスを変更した方が早い
  - 小さなデータ構造の場合は新しいインスタンスを生成して、関数型な書き方で書いた方がわかりやすい
- Constantはどのスコープでも宣言可能
- Shadowingで同じ名前の変数を再宣言できる
- Shadowingで再宣言した変数は元の変数とは別の型として宣言可能