const print = (...args) => Deno.core.print(args.join(" ") + "\n");

print(typeof Deno.core.ops.Rect);

const rect = new Deno.core.ops.Rect(10, 10, 100, 50);

print("top", rect.top); // 10
rect.top = 20;
print("top", rect.top); // 20

print("left", rect.left); // 10
rect.left = 30;
print("left", rect.left); // 30

print("width", rect.width); // 100
rect.width = 200;
print("width", rect.width); // 200
