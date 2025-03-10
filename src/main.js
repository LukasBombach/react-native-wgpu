const print = (...args) => Deno.core.print(args.join(" ") + "\n");

const rect = new Deno.core.ops.op_create_rect(100, 100, 200, 200);

//Deno.core.ops.op_sync_instance_buffer();
//Deno.core.ops.op_request_redraw();
