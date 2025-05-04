import React from "react";
import { ReactWGPU } from "./javascript_runtime/react_wgpu.ts";

ReactWGPU.render(
  <div
    style={{
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
      gap: 100,
      width: "100%",
      height: "100%",
      backgroundColor: "#fff",
    }}
  >
    <div style={{ width: 500, height: 500, backgroundColor: "#f00" }}></div>
    <div style={{ width: 500, height: 500, backgroundColor: "#00f" }}></div>
  </div>
);
