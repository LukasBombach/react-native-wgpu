for (let i = 0; i < 2; i++) {
  const id = Deno.core.ops.op_create_rect(100 + i * 100, 100 + i * 70, 250, 250);
  Deno.core.ops.op_append_rect_to_window(id);
  Deno.core.print(`Added rect with id: ${id}\n`);
}
