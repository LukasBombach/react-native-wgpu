import React from "npm:react";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(
  <rectangle style={{ width: "100%", height: "100%", alignItems: "center", justifyContent: "center", gap: "10%" }}>
    <rectangle style={{ width: "500px", height: "500px" }}></rectangle>
    <rectangle style={{ width: "500px", height: "500px" }}></rectangle>
  </rectangle>
);
