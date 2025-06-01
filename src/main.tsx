import React from "react";
import { ReactWGPU } from "./javascript_runtime/react_wgpu.ts";

ReactWGPU.render(
  <div
    style={{
      display: "flex",
      flexDirection: "column",
      width: "100%",
      height: "100%",
      alignItems: "start",
      backgroundColor: "#000",
    }}
  >
    <div style={{ width: "100%", backgroundColor: "#f00" }}>Hello, World!</div>
    <div style={{ width: "100%", backgroundColor: "#00f" }}>This is rendered text using WGPU!</div>
  </div>
);
