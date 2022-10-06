// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.

// use crate::encoding_utils;
// use crate::models;
use deno_core::error::type_error;
use deno_core::error::AnyError;
use deno_core::include_js_files;
use deno_core::op;
// use deno_core::op;
use deno_core::url::form_urlencoded;
use deno_core::url::quirks;
use deno_core::url::Url;
use deno_core::Extension;
use deno_core::OpState;
// use deno_core::OpState;
use deno_core::ZeroCopyBuf;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stderr, stdout, Write};
use std::path::PathBuf;
use std::rc::Rc;

pub fn init() -> Extension {
  Extension::builder()
    .js(include_js_files!(
      prefix "deno:ext/lit",
      "00_lit.js",
    ))
    .ops(vec![op_lit_test::decl(), op_sign_ecdsa::decl()])
    .build()
}

/// Parse `href` with a `base_href`. Fills the out `buf` with URL components.
#[op]
pub fn op_lit_test(state: &mut OpState, something: String) -> u32 {
  println!("Hello from Rust! Here's something: {}", something);
  return 42;
}

#[op]
async fn op_sign_ecdsa(
  state: Rc<RefCell<OpState>>,
  // state: &mut OpState,
  to_sign: Vec<u8>,
  pubkey: String,
  sig_name: String,
) -> Result<String, AnyError> {
  println!(
    "op_sign_ecdsa() called with to_sign: {:?}, pubkey: {}, sig_name: {}",
    // crate::encoding_utils::bytes_to_hex(&to_sign.clone()),
    to_sign,
    pubkey,
    sig_name
  );

  let fixed_pubkey = pubkey.replace("0x", "");

  // let borrowed_state = state.borrow();
  // let rust_js_comms = borrowed_state.borrow::<RustJsComms>();

  // // auth check
  // let auth_res = crate::functions::auth::check_pkp_auth(
  //   rust_js_comms.lit_action_ipfs_id.clone(),
  //   rust_js_comms.auth_sig.clone(),
  //   fixed_pubkey.clone(),
  // )
  // .await;

  // if let Err(err) = auth_res {
  //   return Err(err);
  // }

  // let is_authed = auth_res.unwrap();
  // if is_authed == false {
  //   return Err(anyhow::anyhow!(
  //           "Neither you nor this Lit Action are authorized to sign using this PKP: {}",
  //           pubkey
  //       ));
  // }

  // let public_key = encoding_utils::hex_to_bytes(&fixed_pubkey);
  // let result = rust_js_comms
  //   .deno_execution_env
  //   .ecdsa_state
  //   .sign_with_pubkey(to_sign.clone(), public_key)
  //   .await;
  // drop(rust_js_comms);
  // drop(borrowed_state);

  // println!("ECDSA signing complete");

  // // pad the pubkey with a zero at the front if it's odd because hex strings should be even and zero padded
  // let mut padded_pubkey = result.public_key;
  // if padded_pubkey.len() % 2 == 1 {
  //   padded_pubkey = "0".to_string() + &padded_pubkey;
  // }

  // let mut borrowed_state_mut = state.borrow_mut();
  // let rust_js_comms_mut = borrowed_state_mut.borrow_mut::<RustJsComms>();

  // // this state is persisted across calls by deno, and so we can use it to
  // // return data to the client that called this Lit function via HTTP
  // rust_js_comms_mut.signed_data.insert(
  //   sig_name.clone(),
  //   SignedData {
  //     sig_type: "ECDSA".to_string(),
  //     data_signed: crate::encoding_utils::bytes_to_hex(&to_sign),
  //     signature_share: result.signature_share,
  //     share_index: result.share_index as u32,
  //     local_x: result.local_x,
  //     local_y: result.local_y,
  //     public_key: padded_pubkey,
  //     sig_name: sig_name,
  //   },
  // );

  Ok("success".to_string())
}
