for (let i = 0; i < 2; i++) {
  const id = Deno.core.ops.op_add_rect(100 + i * 100, 100 + i * 70, 250, 250);
  Deno.core.print(`Added rect with id: ${id}\n`);
}
