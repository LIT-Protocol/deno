// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.

use deno_core::error::type_error;
use deno_core::error::AnyError;
use deno_core::include_js_files;
use deno_core::op;
use deno_core::url::form_urlencoded;
use deno_core::url::quirks;
use deno_core::url::Url;
use deno_core::Extension;
use deno_core::OpState;
use deno_core::ZeroCopyBuf;
use std::path::PathBuf;

pub fn init() -> Extension {
  Extension::builder()
    .js(include_js_files!(
      prefix "deno:ext/lit",
      "00_lit.js",
    ))
    .ops(vec![op_lit_test::decl()])
    .build()
}

/// Parse `href` with a `base_href`. Fills the out `buf` with URL components.
#[op]
pub fn op_lit_test(state: &mut OpState, something: String) -> u32 {
  println!("Hello from Rust! Here's something: {}", something);
  return 42;
}
