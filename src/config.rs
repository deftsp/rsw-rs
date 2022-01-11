// wasm-pack build
// https://rustwasm.github.io/wasm-pack/book/commands/build.html
use std::{env, fs};
use serde_derive::Deserialize;
use anyhow::Result;
use toml;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct CrateConfig {
  pub name: String,
  pub out_dir: Option<String>,
  pub target: Option<String>,
  pub profile: Option<String>,
  pub mode: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct RswConfig {
  pub name: String,
  pub version: String,
  // npm | pnpm
  pub cli: Option<String>,
  // rust crates
  pub crates: Vec<Option<CrateConfig>>,
}

pub(crate) fn rsw_toml_parse() -> Result<RswConfig> {
  let rsw_file = env::current_dir().unwrap().join("rsw.toml");
  let rsw_content = fs::read_to_string(rsw_file).unwrap();
  let rsw_parse = toml::from_str(&rsw_content).unwrap();

  Ok(rsw_parse)
}
