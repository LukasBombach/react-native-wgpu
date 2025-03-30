import React from "npm:react";
import { debug } from "rn-wgpu:rect";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(
  <>
    <rectangle
      style={{
        size: {
          width: {
            Length: 100,
          },
          height: {
            Length: 100,
          },
        },
        /* margin: {
          top: {
            Length: -50,
          },
          left: {
            Length: -50,
          },
          bottom: {
            Length: 0,
          },
          right: {
            Length: 0,
          },
        }, */
        justify_self: "FlexEnd",
      }}
    />
  </>
);

debug();
