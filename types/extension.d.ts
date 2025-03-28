declare module "rn-wgpu:rect" {
  export function create_rect(x: number, y: number, width: number, height: number): number;
  export function append_rect_to_window(rectId: number): void;
  export function update_rect(rectId: number, x: number, y: number, width: number, height: number): void;
  export function remove_rect_from_window(rectId: number): void;
}
