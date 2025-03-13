import { console, setTimeout } from "./std.js";

const create_rect = Deno.core.ops.op_create_rect;
const update_rect = Deno.core.ops.op_update_rect;
const append_rect_to_window = Deno.core.ops.op_append_rect_to_window;

const id = create_rect(100, 100, 200, 200);
update_rect(id, 100, 100, 600, 600);
append_rect_to_window(id);

console.log(`Added rect with id: ${id}`);

setTimeout(() => {
  console.log("Hello from setTimeout");
}, 750);
