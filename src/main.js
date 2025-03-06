const console = {
  log: value => Deno.core.print(JSON.stringify(value) + "\n"),
};

class Rect {
  constructor(x, y, width, height) {
    this.ptr = Deno.core.ops.op_create_rect(x, y, width, height);
  }

  get rect() {
    return Deno.core.ops.op_get_rect(this.ptr);
  }

  update(x, y, width, height) {
    Deno.core.ops.op_update_rect(this.ptr, x, y, width, height);
  }
}

const rect = new Rect(100, 100, 100, 100);

console.log(rect.rect);
rect.update(100, 100, 600, 600);
console.log(rect.rect);
