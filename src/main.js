const ptr = Deno.core.ops.op_create_rect(100, 100, 100, 100);

Deno.core.print(JSON.stringify(Deno.core.ops.op_get_rect(ptr)) + "\n");

Deno.core.ops.op_update_rect(ptr, 200, 200, 200, 200);

Deno.core.print(JSON.stringify(Deno.core.ops.op_get_rect(ptr)) + "\n");
