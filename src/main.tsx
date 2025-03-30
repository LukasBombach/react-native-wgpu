import React from "npm:react";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(
  <>
    <rectangle top={100} left={100} width={600} height={600} />
  </>
);
