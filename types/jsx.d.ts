import "react";
import type { CSSProperties } from "react";

declare module "react" {
  namespace JSX {
    interface IntrinsicElements {
      rectangle: {
        style: Pick<
          CSSProperties,
          | "width"
          | "height"
          | "gap"
          | "columnGap"
          | "rowGap"
          | "alignItems"
          | "alignSelf"
          | "justifyItems"
          | "justifySelf"
          | "alignContent"
          | "justifyContent"
        >;
        children?: React.ReactNode;
      };
    }
  }
}
