//import { NoEventPriority } from "npm:react-reconciler/constants";
import { reconciler } from "./reconciler.ts";

import type { ReactNode } from "react";

const ConcurrentRoot = 1;

function onRecoverableError(error: Error) {
  console.error("Recoverable error:", error);
}

export const ReactWGPU = {
  render(rootInstance: ReactNode) {
    const container = reconciler.createContainer(null, ConcurrentRoot, null, true, null, "", onRecoverableError, null);
    reconciler.updateContainer(rootInstance, container, null, null);
  },
};
