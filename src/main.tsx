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
      backgroundColor: "#000",
    }}
  >
    <div style={{ width: "500px", height: "500px", backgroundColor: "#f00" }}></div>
    <div style={{ width: "500px", height: "500px", backgroundColor: "#0f0" }}></div>
    <div style={{ width: "500px", height: "500px", backgroundColor: "#00f" }}></div>
  </div>
);
