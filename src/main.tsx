import React from "npm:react";
// import { create_rect, append_rect_to_window } from "rn-wgpu:rect";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(<rect top={100} left={100} width={400} height={400} />);

/* const Rect = () => {
  return (
    <div>
      <h1>Rect</h1>
    </div>
  );
};

console.log("<Rect />", <Rect />);

function createAndAppendRect(x: number, y: number, width: number, height: number): void {
  const rectId = create_rect(x, y, width, height);
  append_rect_to_window(rectId);
  console.log("Rect", rectId);
}

createAndAppendRect(100, 100, 400, 400); */
