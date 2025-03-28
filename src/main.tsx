import React from "npm:react";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(
  <rectangle top={200} left={200} width={400} height={400}>
    <rectangle top={400} left={400} width={400} height={400} />
    <rectangle top={500} left={500} width={200} height={200} />
  </rectangle>
);
