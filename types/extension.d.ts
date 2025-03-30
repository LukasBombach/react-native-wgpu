declare module "rn-wgpu:rect" {
  export function create_instance(x: number, y: number, width: number, height: number): number;
  export function create_instance2(style: any): number;
  export function append_child_to_container(rectId: number): void;
}
