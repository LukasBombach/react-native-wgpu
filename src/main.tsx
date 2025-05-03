import React from "react";
import { ReactWGPU } from "./javascript_runtime/react_wgpu.ts";

ReactWGPU.render(
  <rectangle style={{ width: "100%", height: "100%", alignItems: "center", justifyContent: "center", gap: "10%" }}>
    <rectangle style={{ width: "500px", height: "500px" }}></rectangle>
    <rectangle style={{ width: "500px", height: "500px" }}></rectangle>
  </rectangle>
);

// import { get_style_defaults } from "rn-wgpu:rect";
// console.dir(get_style_defaults(), { depth: Infinity });
