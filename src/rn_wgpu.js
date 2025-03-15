import { op_create_rect, op_update_rect, op_append_rect_to_window, op_remove_rect_from_window } from "ext:core/ops";

globalThis.RnWGPU = {
  create_rect: op_create_rect,
  update_rect: op_update_rect,
  append_rect_to_window: op_append_rect_to_window,
  op_remove_rect_from_window: op_remove_rect_from_window,
};
