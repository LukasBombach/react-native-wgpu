import React from "react";
import { ReactWGPU } from "./javascript_runtime/react_wgpu.ts";

ReactWGPU.render(
  <div style={{ width: "100%", height: "100%", alignItems: "center", justifyContent: "center", gap: "10%" }}>
    <div style={{ width: "500px", height: "500px" }}></div>
    <div style={{ width: "500px", height: "500px" }}></div>
  </div>
);
