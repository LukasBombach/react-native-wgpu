/* import { console, setTimeout, setInterval } from "./std.js";

const create_rect = Deno.core.ops.op_create_rect;
const update_rect = Deno.core.ops.op_update_rect;
const append_rect_to_window = Deno.core.ops.op_append_rect_to_window;
const op_remove_rect = Deno.core.ops.op_remove_rect;

const id = create_rect(100, 100, 200, 200);
append_rect_to_window(id);

console.log(`Added rect with id: ${id}`);

setTimeout(() => {
  update_rect(id, 100, 100, 600, 600);
}, 500); */

console.log("Hello from main.js");
