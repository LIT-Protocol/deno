"use strict";

// this doesn't work yet...
((window) => {
  const core = window.Deno.core;
  const ops = core.ops;
  const opLitTest = (p) => ops.op_lit_test(p);
  window.__bootstrap.lit = {
    opLitTest,
  };
})(this);
