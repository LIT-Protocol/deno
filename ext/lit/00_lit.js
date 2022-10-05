"use strict";

((window) => {
  const core = window.Deno.core;
  const ops = core.ops;
  window.opLitTest = (p) => ops.op_lit_test(p);
  window.__bootstrap.lit = {
    opLitTest,
  };
})(this);
