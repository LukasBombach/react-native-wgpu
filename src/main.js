for (let i = 0; i < 10; i++) {
  Deno.core.ops.op_add_rect(100 + i * 100, 100 + i * 70, 250, 250);
}
