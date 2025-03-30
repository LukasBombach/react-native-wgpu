import "react";
import type { TODO_TAFFY_STYLE_TYPE_DEFS } from "rn-wgpu:rect";

declare module "react" {
  namespace JSX {
    interface IntrinsicElements {
      rectangle: {
        style: TODO_TAFFY_STYLE_TYPE_DEFS;
        children?: React.ReactNode;
      };
    }
  }
}
