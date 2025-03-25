import { create_rect, append_rect_to_window, update_rect } from "rn-wgpu:rect";

const id = create_rect(100, 100, 200, 200);
append_rect_to_window(id);

console.log(`Added rect with id: ${id}`);

setTimeout(() => {
  update_rect(id, 100, 100, 600, 600);
}, 500);
