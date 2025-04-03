import React from "npm:react";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(
  <rectangle style={{ width: 500, height: 500, alignItems: "center", justifyContent: "space-around" }}>
    <rectangle style={{ width: "50%", height: "50%" }}></rectangle>
  </rectangle>
);
