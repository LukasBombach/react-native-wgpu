import { reconciler } from "./reconciler.ts";
import type { ReactNode } from "react";

// from react-reconciler/constants, which cannot be imported with rustyscript
const ConcurrentRoot = 1;

function onRecoverableError(error: Error) {
  console.error("Recoverable error:", error);
}

export const ReactWGPU = {
  render(rootInstance: ReactNode) {
    const container = reconciler.createContainer(
      { type: "container" },
      ConcurrentRoot,
      null,
      true,
      null,
      "",
      onRecoverableError,
      null
    );
    reconciler.updateContainer(rootInstance, container, null, null);
  },
};
