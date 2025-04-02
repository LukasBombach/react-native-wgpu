import { TAFFY_STYLE_DEFAULTS as DEFAULTS } from "./taffy_style_defaults.ts";
import type { CSSProperties } from "react";

interface TaffyStyle {
  size?: {
    width: TaffyLength;
    height: TaffyLength;
  };
}

type TaffyLength =
  | {
      Length: number;
    }
  | {
      Percent: number; // 0.0 - 1.0
    }
  | "Auto";

type CSSLength = number | `${number}px` | `${number}%` | "auto";

function length(length: CSSLength | string | undefined): TaffyLength {
  if (length === undefined) {
    throw new Error("Length is undefined");
  }

  if (typeof length === "number") {
    return { Length: length };
  }

  if (length.endsWith("%")) {
    return { Percent: parseFloat(length.toString()) / 100 };
  }

  if (length.endsWith("px")) {
    return { Length: parseFloat(length.toString()) };
  }

  if (length === "auto") {
    return "Auto";
  }

  throw new Error(`Invalid CSS length: ${length}`);
}

export function cssToTaffy(css: CSSProperties) {
  const style: TaffyStyle = {};

  if (css.width) {
    style.size = style.size || DEFAULTS.size;
    style.size.width = length(css.width);
  }

  if (css.height) {
    style.size = style.size || DEFAULTS.size;
    style.size.height = length(css.height);
  }

  const unknownProps = Object.keys(css).filter(key => ["width", "height"].includes(key));
  if (unknownProps.length > 0) {
    throw new Error(`Unknown CSS properties: ${unknownProps.join(", ")}`);
  }
}
