import React from "npm:react";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(
  <>
    <rectangle
      style={{
        size: {
          width: {
            Length: 600,
          },
          height: {
            Length: 600,
          },
        },
      }}
    />
  </>
);
