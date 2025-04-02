import type { CSSProperties } from "react";

export type TaffyDefaults = typeof defaults;

export const defaults = {
  display: "Flex",
  item_is_table: false,
  box_sizing: "BorderBox",
  overflow: { x: "Visible", y: "Visible" },
  scrollbar_width: 0,
  position: "Relative",
  inset: {
    left: "Auto",
    right: "Auto",
    top: "Auto",
    bottom: "Auto",
  },
  size: { width: "Auto", height: "Auto" },
  min_size: { width: "Auto", height: "Auto" },
  max_size: { width: "Auto", height: "Auto" },
  aspect_ratio: null,
  margin: {
    left: { Length: 0 },
    right: { Length: 0 },
    top: { Length: 0 },
    bottom: { Length: 0 },
  },
  padding: {
    left: { Length: 0 },
    right: { Length: 0 },
    top: { Length: 0 },
    bottom: { Length: 0 },
  },
  border: {
    left: { Length: 0 },
    right: { Length: 0 },
    top: { Length: 0 },
    bottom: { Length: 0 },
  },
  align_items: null,
  align_self: null,
  justify_items: null,
  justify_self: null,
  align_content: null,
  justify_content: null,
  gap: {
    width: { Length: 0 },
    height: { Length: 0 },
  },
  text_align: "Auto",
  flex_direction: "Row",
  flex_wrap: "NoWrap",
  flex_basis: "Auto",
  flex_grow: 0,
  flex_shrink: 1,
  grid_template_rows: [],
  grid_template_columns: [],
  grid_auto_rows: [],
  grid_auto_columns: [],
  grid_auto_flow: "Row",
  grid_row: { start: "Auto", end: "Auto" },
  grid_column: { start: "Auto", end: "Auto" },
} as const;

export interface Style {
  size?: Size<LengthPercentage>;
  gap?: Size<LengthPercentage>;
}

interface Size<T> {
  width: T;
  height: T;
}

type LengthPercentage = Length | Percent | Auto;

interface Length {
  Length: number;
}

interface Percent {
  Percent: number;
}

type Auto = "Auto";

export function taffyFromCss({ width, height, gap, columnGap, rowGap }: CSSProperties): Style {
  const style: Style = {};

  if (width) {
    style.size = style.size || defaults.size;
    style.size.width = length(width);
  }

  if (height) {
    style.size = style.size || defaults.size;
    style.size.height = length(height);
  }

  if (gap) {
    style.gap = style.gap || size(shorthand(gap));
  }

  if (columnGap) {
    style.gap = style.gap || defaults.gap;
    style.gap.width = length(columnGap);
  }

  if (rowGap) {
    style.gap = style.gap || defaults.gap;
    style.gap.height = length(rowGap);
  }

  return style;
}

function shorthand(value: string | number): string[] {
  return value.toString().split(" ");
}

function size(value: string[]): Size<LengthPercentage> {
  const [width, height] = value.map(length);
  return { width, height: height ?? width };
}

function length(value: string | number): LengthPercentage {
  if (typeof value === "number") return { Length: value };
  if (value.match(/^\d+$/)) return { Length: parseFloat(value) };
  if (value.endsWith("%")) return { Percent: parseFloat(value) / 100 };
  if (value.endsWith("px")) return { Length: parseFloat(value) };
  if (value === "auto") return "Auto";
  throw new Error(`Invalid CSS length: ${value} (${typeof value})`);
}
