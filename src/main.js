import { create_rect, append_rect_to_window, update_rect } from "rn-wgpu:rect";

const id = create_rect(100, 100, 400, 400);
append_rect_to_window(id);

console.log("Rect", id);
