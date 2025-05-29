import React from "react";
import { ReactWGPU } from "./javascript_runtime/react_wgpu.ts";

ReactWGPU.render(
  <div
    style={{
      width: "100%",
      height: "100%",
      alignItems: "center",
      justifyContent: "center",
      padding: "10%",
      gap: "10%",
    }}
  >
    <div style={{ width: "500px", height: "500px", backgroundColor: "#f00", borderRadius: 20 }}></div>
    <div style={{ width: "500px", height: "500px", backgroundColor: "#0f0", borderRadius: 5 }}></div>
    <div style={{ width: "500px", height: "500px", backgroundColor: "#00f", borderRadius: 999 }}></div>
  </div>
);
