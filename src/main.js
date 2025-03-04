Deno.core.print("Hello runjs!\n");

for (let i = 0; i < 10; i++) {
  Deno.core.ops.op_add_rect({
    x: 400 + i * 100,
    y: 250 + i * 50,
    w: 250,
    h: 250,
  });
}
