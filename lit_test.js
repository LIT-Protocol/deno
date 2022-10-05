console.log(Deno.core.ops);
console.log(Deno.core.ops.op_lit_test);
let a = window.opLitTest("meow");
// let a = Deno.core.ops.op_lit_test("meow");
console.log("a =", a);
