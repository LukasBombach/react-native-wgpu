import React from "react";
import { ReactWGPU } from "./javascript_runtime/react_wgpu.ts";

ReactWGPU.render(
  <div
    style={{
      width: "100%",
      height: "100%",
      alignItems: "start",
      padding: "50px",
      backgroundColor: "#f0f0f0",
    }}
  >
    <div
      style={{
        width: "100%",
        aspectRatio: "16/9",
        backgroundColor: "#ffffff",
        borderRadius: 15,
        padding: "30px",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      Hello World! This is text rendering with WGPU and React Native.
    </div>
    <div
      style={{
        marginTop: "20px",
        padding: "20px",
        backgroundColor: "#e8f4f8",
        borderRadius: 8,
        width: "100%",
      }}
    >
      This is a second text block to test multiple text areas and line wrapping. It should wrap properly within the
      container bounds and update when the window is resized.
    </div>
  </div>
);
