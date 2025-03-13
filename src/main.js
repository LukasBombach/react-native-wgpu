const id = Deno.core.ops.op_create_rect(100, 100, 200, 200);

Deno.core.ops.op_update_rect(id, 100, 100, 600, 600);

Deno.core.ops.op_append_rect_to_window(id);

Deno.core.print(`Added rect with id: ${id}\n`);
