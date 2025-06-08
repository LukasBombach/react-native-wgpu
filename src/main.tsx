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
      padding: 20,
    }}
  >
    <div
      style={{
        width: "100%",
        aspectRatio: "16/9",
        backgroundColor: "#f00",
        borderRadius: 10,
      }}
    ></div>
    <div style={{ width: "100%", backgroundColor: "#f00", borderRadius: 0 }}>Hello, World!</div>
    <div style={{ width: "100%", backgroundColor: "#00f", borderRadius: 1 }}>This is rendered text using WGPU!</div>
  </div>
);
