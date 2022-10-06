"use strict";

((window) => {
  const core = window.Deno.core;
  const ops = core.ops;
  // window.opLitTest = (p) => ops.op_lit_test(p);
  // window.__bootstrap.lit = {
  //   opLitTest,
  // };

  const LitActions = {
    test: (p) => ops.op_lit_test(p),
    signEcdsa: function ({ toSign, publicKey, sigName }) {
      // return ops.op_sign_ecdsa(Array.from(toSign), publicKey, sigName);
      return core.opAsync(
        "op_sign_ecdsa",
        Array.from(toSign),
        publicKey,
        sigName
      );
    },
    // signBls: function ({ toSign, publicKey, sigName }) {
    //   return Deno.core.opAsync(
    //     "op_sign_bls",
    //     Array.from(toSign),
    //     publicKey,
    //     sigName
    //   );
    // },
    // decryptBls: function ({ toDecrypt, publicKey, decryptionName }) {
    //   return Deno.core.opAsync(
    //     "op_decrypt_bls",
    //     Array.from(toDecrypt),
    //     publicKey,
    //     decryptionName
    //   );
    // },
    // checkConditions: function ({ conditions, authSig, chain }) {
    //   return Deno.core.opAsync(
    //     "op_check_conditions",
    //     conditions,
    //     authSig,
    //     chain
    //   );
    // },
    // ethPersonalSignMessageEcdsa: function ({ message, publicKey, sigName }) {
    //   const messagePrepended = Uint8arrays.fromString(
    //     "\x19Ethereum Signed Message:\n" + message.length + message,
    //     "utf8"
    //   );
    //   const hashed = Deno.core.opSync(
    //     "op_keccak256",
    //     Array.from(messagePrepended)
    //   );
    //   return Deno.core.opAsync("op_sign_ecdsa", hashed, publicKey, sigName);
    // },
    // setResponse: function ({ response }) {
    //   return Deno.core.opSync("op_set_response", response);
    // },
    // call: function ({ ipfsId, params }) {
    //   return Deno.core.opAsync("op_call_child", ipfsId, params);
    // },
    // uint8arrayToString: Uint8arrays.toString,
    // uint8arrayFromString: Uint8arrays.fromString,
  };
  Object.freeze(LitActions);

  // // this empty block scopes oldFetch so that nobody can ever use it after
  // {
  //   let oldFetch = fetch;
  //   fetch = function () {
  //     let fetchCount = Deno.core.opSync("op_increment_fetch_count");
  //     console.log("fetchCount: " + fetchCount);
  //     return oldFetch.apply(null, arguments);
  //   };
  //   Object.freeze(fetch);
  // }

  window.LitActions = LitActions;
})(this);
