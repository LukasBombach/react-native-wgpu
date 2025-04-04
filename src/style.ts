import type { CSSProperties } from "react";

interface TaffyStyle {
  display?: "Block" | "Flex" | "Grid" | "None";
  item_is_table?: boolean;
  box_sizing?: "BorderBox" | "ContentBox";
  overflow?: Point<Overflow>;
  scrollbar_width?: number;
  position?: Position;
  inset?: Rect<Length>;
  size?: Size<Length>;
  min_size?: Size<Length>;
  max_size?: Size<Length>;
  aspect_ratio?: number;
  margin?: Rect<Length>;
  padding?: Rect<Length>;
  border?: Rect<Length>;
  align_items?: Align;
  align_self?: Align;
  justify_items?: Align;
  justify_self?: Align;
  align_content?: AlignContent;
  justify_content?: AlignContent;
  gap?: Size<Length>;
  text_align?: TextAlign;
  flex_direction?: FlexDirection;
  flex_wrap?: FlexWrap;
  flex_basis?: Length;
  flex_grow?: number;
  flex_shrink?: number;
}

type Size<T> = { width: T; height: T };
type Point<T> = { x: T; y: T };
type Rect<T> = { left: T; right: T; top: T; bottom: T };
type Length = { Length: number } | { Percent: number } | "Auto";
type Overflow = "Visible" | "Clip" | "Hidden" | "Scroll";
type Position = "Relative" | "Absolute";
type Align = "Start" | "End" | "FlexStart" | "FlexEnd" | "Center" | "Baseline" | "Stretch";
type AlignContent =
  | "Start"
  | "End"
  | "FlexStart"
  | "FlexEnd"
  | "Center"
  | "Stretch"
  | "SpaceBetween"
  | "SpaceEvenly"
  | "SpaceAround";
type TextAlign = "Auto" | "LegacyLeft" | "LegacyRight" | "LegacyCenter";
type FlexDirection = "Row" | "Column" | "RowReverse" | "ColumnReverse";
type FlexWrap = "NoWrap" | "Wrap" | "WrapReverse";

const align = {
  start: "Start",
  end: "End",
  center: "Center",
  baseline: "Baseline",
  stretch: "Stretch",
  "flex-start": "FlexStart",
  "flex-end": "FlexEnd",
};

const alignContent = {
  start: "Start",
  end: "End",
  center: "Center",
  stretch: "Stretch",
  "flex-start": "FlexStart",
  "flex-end": "FlexEnd",
  "space-between": "SpaceBetween",
  "space-evenly": "SpaceEvenly",
  "space-around": "SpaceAround",
};

function shorthand(value: string | number): string[] {
  return value.toString().split(" ");
}

function size(value: string[]): Size<Length> {
  const [width, height] = value.map(length);
  return { width, height: height ?? width };
}

function length(value: string | number): Length {
  if (typeof value === "number") return { Length: value };
  if (value.match(/^\d+$/)) return { Length: parseFloat(value) };
  if (value.endsWith("%")) return { Percent: parseFloat(value) / 100 };
  if (value.endsWith("px")) return { Length: parseFloat(value) };
  if (value === "auto") return "Auto";
  throw new Error(`Invalid CSS length: ${value} (${typeof value})`);
}

function mapProp(
  taffy: TaffyStyle,
  css: CSSProperties,
  cssProp: keyof CSSProperties,
  taffyProp: keyof TaffyStyle,
  map: Record<string, string>
) {
  const cssValue = css[cssProp];
  if (cssValue !== undefined) {
    const taffyValue = map[cssValue as keyof typeof map];
    // @ts-expect-error WORK IN PROGRESS
    taffy[taffyProp] = taffyValue;
  }
}

export function taffyFromCss({ width, height, gap, columnGap, rowGap, ...css }: CSSProperties): TaffyStyle {
  const taffy: TaffyStyle = {};

  if (width) {
    taffy.size = taffy.size || defaults.size;
    taffy.size.width = length(width);
  }

  if (height) {
    taffy.size = taffy.size || defaults.size;
    taffy.size.height = length(height);
  }

  if (gap) {
    taffy.gap = taffy.gap || size(shorthand(gap));
  }

  if (columnGap) {
    taffy.gap = taffy.gap || defaults.gap;
    taffy.gap.width = length(columnGap);
  }

  if (rowGap) {
    taffy.gap = taffy.gap || defaults.gap;
    taffy.gap.height = length(rowGap);
  }

  // todo refactor with effect.ts
  mapProp(taffy, css, "alignItems", "align_items", align);
  mapProp(taffy, css, "alignSelf", "align_self", align);
  mapProp(taffy, css, "justifyItems", "justify_items", align);
  mapProp(taffy, css, "justifySelf", "justify_self", align);
  mapProp(taffy, css, "alignContent", "align_content", alignContent);
  mapProp(taffy, css, "justifyContent", "justify_content", alignContent);

  return taffy;
}

const defaults = {
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
  size: { width: "Auto", height: "Auto" }, // ✅
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
  align_items: null, // ✅
  align_self: null, // ✅
  justify_items: null, // ✅
  justify_self: null, // ✅
  align_content: null, // ✅
  justify_content: null, // ✅
  // ✅
  gap: {
    width: { Length: 0 }, // ✅
    height: { Length: 0 }, // ✅
  }, // ✅
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
