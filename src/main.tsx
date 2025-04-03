import React from "npm:react";
import { debug } from "rn-wgpu:rect";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(
  <rectangle style={{ width: "100%", height: "100%", alignItems: "center", justifyContent: "space-around" }}>
    <rectangle style={{ width: 500, height: 500, alignItems: "center", gap: 50 }}>
      <rectangle style={{ width: "20%", height: "20%" }} />
      <rectangle style={{ width: "20%", height: "20%" }} />
      <rectangle style={{ width: "20%", height: "20%" }} />
    </rectangle>
  </rectangle>
);

setTimeout(() => {
  console.log("");
  debug();
}, 100);
