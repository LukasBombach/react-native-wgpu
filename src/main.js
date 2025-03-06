const print = obj => Deno.core.print(JSON.stringify(obj) + "\n");
const create = Deno.core.ops.op_create_rect;
const get = Deno.core.ops.op_get_rect;
const update = Deno.core.ops.op_update_rect;

const ptr = create(100, 100, 100, 100);
update(ptr, 100, 100, 600, 600);
