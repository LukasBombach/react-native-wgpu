const print = (...args) => Deno.core.print(args.join(" ") + "\n");

const rect = new Deno.core.ops.Rect(10, 10, 100, 50);

Deno.core.ops.op_sync_instance_buffer();
Deno.core.ops.op_request_redraw();
