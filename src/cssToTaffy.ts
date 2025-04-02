import { TAFFY_STYLE_DEFAULTS as DEFAULTS } from "./taffy_style_defaults.ts";
import type { CSSProperties } from "react";

interface TaffyStyle {
  size?: TaffySize;
  gap?: TaffySize;
}

interface TaffySize {
  width: TaffyLength;
  height: TaffyLength;
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

function length(value: string | number | undefined): TaffyLength {
  if (value === undefined) {
    throw new Error("Length is undefined");
  }

  if (typeof value === "number") {
    return { Length: value };
  }

  if (value.endsWith("%")) {
    return { Percent: parseFloat(value.toString()) / 100 };
  }

  if (value.endsWith("px")) {
    return { Length: parseFloat(value.toString()) };
  }

  if (value === "auto") {
    return "Auto";
  }

  throw new Error(`Invalid CSS length: ${value}`);
}

function lengthShorthand(value: string | number): TaffySize {
  const [width, height] = value.toString().split(" ");
  return {
    width: length(width),
    height: height ? length(height) : length(width),
  };
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

  if (css.gap) {
    style.gap = style.gap || DEFAULTS.gap;
    style.gap = lengthShorthand(css.gap);
  }

  if (css.columnGap) {
    style.gap = style.gap || DEFAULTS.gap;
    style.gap.width = length(css.width);
  }

  if (css.rowGap) {
    style.gap = style.gap || DEFAULTS.gap;
    style.gap.height = length(css.height);
  }

  const unknownProps = Object.keys(css).filter(key => !["width", "height"].includes(key));
  if (unknownProps.length > 0) {
    throw new Error(`Unknown CSS properties: ${unknownProps.join(", ")}`);
  }

  return style;
}
