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
      }}
    />
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
      }}
    />
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
      }}
    />
  </>
);

setTimeout(() => {
  console.log("");
  debug();
}, 100);
