import { reconciler } from "./reconciler.ts";
import type { ReactNode } from "react";

// from react-reconciler/constants, which cannot be imported with rustyscript
const ConcurrentRoot = 1;

export const ReactWGPU = {
  render(rootInstance: ReactNode) {
    const container = reconciler.createContainer(
      { type: "container" },
      ConcurrentRoot,
      null,
      true,
      null,
      "",
      error => console.error("Recoverable error:", error),
      null
    );
    reconciler.updateContainer(rootInstance, container, null, null);
  },
};
