import React from "react";
import { ReactWGPU } from "./javascript_runtime/react_wgpu.ts";

ReactWGPU.render(
  <div
    style={{
      width: "100%",
      height: "100%",
      alignItems: "start",
      padding: "100px",
      flexDirection: "column",
      gap: "10px",
    }}
  >
    <div
      style={{
        width: "100%",
        aspectRatio: "16/9",
        backgroundColor: "#fff",
        // borderRadius: 10,
      }}
    ></div>

    <div
      style={{
        width: "100%",
        backgroundColor: "#f00",
        borderRadius: 1,
        padding: 10,
      }}
    >
      Hello world
    </div>
  </div>
);
