//! [e-gov略称法令名一覧](https://elaws.e-gov.go.jp/abb/)をスクレイピングして、
//! 略称法令名一覧を作成するソフトウェア
//! # Install
//!
//! ```sh
//! cargo install --git "https://github.com/japanese-law-analysis/egov_law_abb.git"
//! ```
//! 
//! # Use
//!
//! ```sh
//!   egov_law_abb --output "egov_abb.json"
//! ```
//! 
//! のようにして使用します。
//! 
//! # 生成される情報
//! 
//! 以下の2つのフィールドを持つobjectのリストが生成されます
//! 
//! - `num`：法令番号
//! - `abbs`：略称の文字列のリスト
//! 
//! ---
//! [MIT License](https://github.com/japanese-law-analysis/listup_precedent/blob/master/LICENSE)
//! (c) 2023 Naoki Kaneko (a.k.a. "puripuri2100")
//!

use anyhow::Result;
use clap::Parser;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct Abb {
  num: String,
  abbs: Vec<String>,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long)]
  output: String,
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();

  let url = "https://elaws.e-gov.go.jp/abb/";

  let body = reqwest::get(url).await?.text().await?;

  let doc = Html::parse_document(&body);

  let td_lst_selector = Selector::parse("tbody > tr").unwrap();
  let td_selector = Selector::parse("td").unwrap();

  let mut lst = Vec::new();
  let mut tmp_num = String::new();
  for tds in doc.select(&td_lst_selector) {
    let mut l = Vec::new();
    for (i, td) in tds.select(&td_selector).enumerate() {
      if i == 1 {
        tmp_num = td.text().map(|s| s.trim()).collect::<String>();
      } else if i >= 2 {
        let s = td.text().map(|s| s.trim()).collect::<String>();
        if !s.is_empty() {
          l.push(s)
        }
      }
    }
    lst.push(Abb {
      num: tmp_num,
      abbs: l,
    });
    tmp_num = String::new();
  }

  let abb_lst_str = serde_json::to_string_pretty(&lst)?;

  let mut f = File::create(&args.output).await?;
  f.write_all(abb_lst_str.as_bytes()).await?;

  Ok(())
}
