import React from "npm:react";
import { debug } from "rn-wgpu:rect";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(
  <>
    <rectangle
      style={{
        size: {
          width: {
            Length: 300,
          },
          height: {
            Length: 300,
          },
        },
        margin: {
          top: {
            Length: 0,
          },
          left: {
            Length: 0,
          },
          bottom: {
            Length: 50,
          },
          right: {
            Length: 50,
          },
        },
        justify_self: "FlexEnd",
        align_self: "FlexEnd",
      }}
    />
  </>
);

debug();
