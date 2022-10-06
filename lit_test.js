// console.log(Deno.core.ops);
// console.log(Deno.core.ops.op_lit_test);
// let a = window.opLitTest("meow");
// let a = Deno.core.ops.op_lit_test("meow");
// console.log("a =", a);

const toSign = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100];
const publicKey = "meow";
const sigName = "sig1";

const sigShare = await LitActions.signEcdsa({ toSign, publicKey, sigName });
console.log("sigShare: ", sigShare);

// let a = LitActions.test("meow");
// console.log("a: ", a);
