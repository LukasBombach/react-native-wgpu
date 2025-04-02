import React from "npm:react";
import { debug, get_style_defaults } from "rn-wgpu:rect";
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
    {/* <div style={{ width: "100%", height: "100%" }} /> */}
  </>
);

console.log(get_style_defaults());

/* setTimeout(() => {
  console.log("");
  debug();
}, 100); */
