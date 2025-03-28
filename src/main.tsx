import React from "npm:react";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(
  <>
    <rect top={200} left={200} width={400} height={400} />
    <rect top={400} left={400} width={400} height={400} />
  </>
);
