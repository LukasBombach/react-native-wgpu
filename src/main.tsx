import React from "npm:react";
import { debug } from "rn-wgpu:rect";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(
  <>
    <rectangle
      style={{
        size: {
          width: {
            Percent: 1.0,
          },
          height: {
            Percent: 1.0,
          },
        },
      }}
    />
    <div style={{ width: "100%", height: "100%" }} />
  </>
);

setTimeout(() => {
  console.log("");
  debug();
}, 100);
