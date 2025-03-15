const id = RnWGPU.create_rect(100, 100, 200, 200);
RnWGPU.append_rect_to_window(id);

console.log(`Added rect with id: ${id}`);

setTimeout(() => {
  RnWGPU.update_rect(id, 100, 100, 600, 600);
}, 500);

console.log("Hello from main.js");
