# egov_law_abb

[e-gov略称法令名一覧](https://elaws.e-gov.go.jp/abb/)をスクレイピングして、
略称法令名一覧を作成するソフトウェア
## Install

```sh
cargo install --git "https://github.com/japanese-law-analysis/egov_law_abb.git"
```

## Use

```sh
  egov_law_abb --output "egov_abb.json"
```

のようにして使用します。

## 生成される情報

以下の2つのフィールドを持つobjectのリストが生成されます

- `num`：法令番号
- `abbs`：略称の文字列のリスト

---
[MIT License](https://github.com/japanese-law-analysis/listup_precedent/blob/master/LICENSE)
(c) 2023 Naoki Kaneko (a.k.a. "puripuri2100")

