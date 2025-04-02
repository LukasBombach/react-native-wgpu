import React from "npm:react";
import { ReactWGPU } from "./renderer.ts";

ReactWGPU.render(
  <rectangle style={{ width: "50%", height: "50%", gap: 0, alignItems: "center", justifyContent: "space-around" }}>
    <rectangle style={{ width: 100, height: 100 }} />
    <rectangle style={{ width: 100, height: 100 }} />
    <rectangle style={{ width: 100, height: 100 }} />
    <rectangle style={{ width: 100, height: 100 }} />
  </rectangle>
);

/* setTimeout(() => {
  console.log("");
  debug();
}, 100); */
