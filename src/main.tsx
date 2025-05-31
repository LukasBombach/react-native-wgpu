import React from "react";
import { ReactWGPU } from "./javascript_runtime/react_wgpu.ts";

ReactWGPU.render(
  <div
    style={{
      width: "100%",
      height: "100%",
      alignItems: "start",
      padding: "70px 0px",
    }}
  >
    Hello, World! This is rendered text using WGPU!
  </div>
);
