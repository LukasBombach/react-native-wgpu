Deno.core.print("Hello runjs!\n");

for (let i = 0; i < 10; i++) {
  Deno.core.ops.op_add_rect(400 + i * 100, 250 + i * 50, 250, 250);
}
