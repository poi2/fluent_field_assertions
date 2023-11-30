FluentFieldAssertions を使って Rust のテストコードを綺麗にしよう

-----

この記事は [Rust Advent Calendar 2023](https://qiita.com/advent-calendar/2023/rust) 2日目の記事です。

# はじめに

[FluentFieldAssertions](https://crates.io/crates/fluent_field_assertions) という Rust のテストコードを綺麗に書ける crate について紹介します。
この記事ではおすすめする理由と使い方の紹介、そして競合との比較を行います。

# FluentFieldAssertions

自然言語に近い文法でテストコードを書くための Fluent Assertion ライブラリーというものが各言語には存在します。
`FluentFieldAssertions` もそのひとつで、本記事の作者によって作成された Rust の crate です。

## 使うとどうなる？

さっそく `FluentFieldAssertions` を使ってみましょう。
まずは Rust の標準のアサーションとの比較です。

```rust
let user = User {
    id: 1,
    name: "Alice".to_string(),
    age: 17,
};

// Rust の標準のアサーション
assert_eq!(user.name, "Alice".to_string());
assert_eq!(user.age, 17);

// FluentFieldAssertions
user.name_eq(&"Alice".to_string())
    .age_eq(&17);
```

`FluentFieldAssertions` を使うことで、テストコードを自然言語に近い文法で書けるようになりました。
人間の思考に近い形で書けるため、シンプルで読みやすい綺麗なコードになります。
また、記述量も減るため、メンテナンスコストも下がります。

## 使い方

まず `FluentFieldAssertions` をプロジェクトに導入しましょう。
`Cargo.toml` に `fluent_field_assertions` を追加します。

```toml
fluent_field_assertions = "0.2"
```

具体的な使い方は次のようになります。

```rust
// FluentFieldAssertions をインポートする。
use fluent_field_assertions::FluentFieldAssertions;

// User 構造体に FluentFieldAssertions を derive することで、
// User の各フィールドに対して以下のメソッドたちが追加される。
// - {field_name}_eq
// - {field_name}_ne
// - {field_name}_satisfies
#[derive(FluentFieldAssertions)]
struct User {
    id: usize,
    name: String,
    age: usize,
}

let user = User {
    id: 1,
    name: "Alice".to_string(),
    age: 17,
};

user.id_eq(&1)
    .name_eq(&"Alice".to_string())
    .age_satisfies(|age| age < &18);
```

詳細な使い方については [FluentFieldAssertions](https://github.com/poi2/fluent_field_assertions) リポジトリーの README を参照してください。

## 実践的な使い方

基本的にはこの機能はテストのときだけしか利用しないので、プロダクションのコードには含めたくはありません。
test の attribute や feature でのみ derive することで、テストのときだけ使えるようにすることができます。

```rust
#[cfg_attr(
    any(test, feature = "test"),
    derive(
        fluent_field_assertions::FluentFieldAssertions
    )
)]
struct User {
    id: usize,
    name: String,
    age: usize,
}
```

# 競合との比較

Rust には他にも Fluent Assertion crate が存在します。
例えば [speculoos](https://lib.rs/crates/speculoos) や [fluent-asserter](https://crates.io/crates/fluent-asserter) などです。
`FluentFieldAssertions` と並べて比較してみましょう。

```rust
// FluentFieldAssertions
user.name_eq(&"Alice".to_string())
    .age_eq(&17);

// speculoos
assert_that(user.name).is_equal_to("Alice".to_string());
assert_that(user.age).is_equal_to(17);

// fluent-asserter
assert_that!(user.name).is_equal_to("Alice".to_string());
assert_that!(user.age).is_equal_to(17);
```

制作者の贔屓目もありますが、`FluentFieldAssertions` のほうがコードのシンプルさ、短さ、読みやすさで優れていると思います。

一方、matcher は `speculoos` や `fluent-asserter` のほうが多く、ぴったりのテストコードを書きやすいです。
しかし、ユーザーにとっては学習コストは高くなりますし、ニッチな機能なので使いこなす機会も少ないです。
そう考えるとシンプルかつ自由に条件を書ける `FluentFieldAssertions` の `{field_name}_satisfies()` で十分ではないでしょうか。

どの crate もとても優秀ですが、総合すると `FluentFieldAssertions` が一番良いです。
ただし、いろいろな matcher を使いたい場合は `speculoos` や `fluent-asserter` を使うのも良いでしょう。

# まとめ

`FluentFieldAssertions` を使うとテストコードを綺麗に書けて、メンテナンスコストも低くなります。
便利なので、ぜひ使ってみてください。

`FluentFieldAssertions` はまだまだ開発中です。
バグや要望があればコメントか [issue](https://github.com/poi2/fluent_field_assertions/issues) に書いていただけると嬉しいです。
