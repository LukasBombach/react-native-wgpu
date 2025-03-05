for (let i = 0; i < 10; i++) {
  const ptr = Deno.core.ops.op_create_rect(100 + i * 100, 100 + i * 70, 250, 250);
  Deno.core.print(JSON.stringify(Deno.core.ops.op_get_rect(ptr)) + "\n");
}
