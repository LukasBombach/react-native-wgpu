import React from "react";
import { ReactWGPU } from "./javascript_runtime/react_wgpu.ts";

ReactWGPU.render(
  <div
    style={{
      width: "100%",
      height: "100%",
      alignItems: "start",
      padding: "100px",
    }}
  >
    <div
      style={{
        width: "100%",
        aspectRatio: "16/9",
        backgroundColor: "#fff",
        borderRadius: 10,
      }}
    ></div>
  </div>
);
