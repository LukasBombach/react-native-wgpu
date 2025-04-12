import React from "npm:react";
import { ReactWGPU } from "./renderer.ts";
import { get_style_defaults } from "rn-wgpu:rect";

ReactWGPU.render(
  <rectangle style={{ width: "100%", height: "100%", alignItems: "center", justifyContent: "center", gap: "10%" }}>
    <rectangle style={{ width: "500px", height: "500px" }}></rectangle>
    <rectangle style={{ width: "500px", height: "500px" }}></rectangle>
  </rectangle>
);

// console.dir(get_style_defaults(), { depth: Infinity });
