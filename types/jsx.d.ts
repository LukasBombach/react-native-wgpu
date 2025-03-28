import "react";

declare module "react" {
  namespace JSX {
    interface IntrinsicElements {
      rectangle: {
        top: number;
        left: number;
        width: number;
        height: number;
        children?: React.ReactNode;
      };
    }
  }
}
