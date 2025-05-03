import { match, P } from "ts-pattern";
import { pipe } from "fp-ts/lib/function.js";
import * as z from "zod";

export interface Style {
  display: Display;
  item_is_table: boolean;
  box_sizing: BoxSizing;
  overflow: Point<Overflow>;
  scrollbar_width: number;
  position: Position;
  inset: Rect<LengthPercentageAuto>;
  size: Size<Dimension>;
  min_size: Size<Dimension>;
  max_size: Size<Dimension>;
  aspect_ratio: Option<number>;
  margin: Rect<LengthPercentageAuto>;
  padding: Rect<LengthPercentage>;
  border: Rect<LengthPercentage>;
  align_items: Option<AlignItems>;
  align_self: Option<AlignSelf>;
  justify_items: Option<AlignItems>;
  justify_self: Option<AlignSelf>;
  align_content: Option<AlignContent>;
  justify_content: Option<JustifyContent>;
  gap: Size<LengthPercentage>;
  text_align: TextAlign;
  flex_direction: FlexDirection;
  flex_wrap: FlexWrap;
  flex_basis: Dimension;
  flex_grow: number;
  flex_shrink: number;
  grid_template_rows: GridTrackVec<TrackSizingFunction>;
  grid_template_columns: GridTrackVec<TrackSizingFunction>;
  grid_auto_rows: GridTrackVec<NonRepeatedTrackSizingFunction>;
  grid_auto_columns: GridTrackVec<NonRepeatedTrackSizingFunction>;
  grid_auto_flow: GridAutoFlow;
  grid_row: Line<GridPlacement>;
  grid_column: Line<GridPlacement>;
}

export type Display = "Block" | "Flex" | "Grid" | "None";

export type BoxSizing = "BorderBox" | "ContentBox";

export interface Point<T> {
  x: T;
  y: T;
}

export type Overflow = "Visible" | "Clip" | "Hidden" | "Scroll";

export type Position = "Relative" | "Absolute";

export interface Rect<T> {
  left: T;
  right: T;
  top: T;
  bottom: T;
}

export type LengthPercentageAuto = Length<number> | Percent<number> | Auto;

export interface Length<T> {
  Length: T;
}

export interface Percent<T> {
  Percent: T;
}

export type Auto = "Auto";

export interface Size<Width, Height = Width> {
  width: Width;
  height: Height;
}

export type Dimension = Length<number> | Percent<number> | Auto;

export type Option<T> = T | undefined;

export type LengthPercentage = Length<number> | Percent<number>;

export type AlignItems = "Start" | "End" | "FlexStart" | "FlexEnd" | "Center" | "Baseline" | "Stretch";

export type AlignSelf = AlignItems;

export type AlignContent =
  | "Start"
  | "End"
  | "FlexStart"
  | "FlexEnd"
  | "Center"
  | "Stretch"
  | "SpaceBetween"
  | "SpaceEvenly"
  | "SpaceAround";

export type JustifyContent = AlignContent;

export type TextAlign = "Auto" | "LegacyLeft" | "LegacyRight" | "LegacyCenter";

export type FlexDirection = "Row" | "Column" | "RowReverse" | "ColumnReverse";

export type FlexWrap = "NoWrap" | "Wrap" | "WrapReverse";

export type GridTrackVec<T> = Array<T>;

export type TrackSizingFunction =
  | Single<NonRepeatedTrackSizingFunction>
  | Repeat<GridTrackRepetition, GridTrackVec<NonRepeatedTrackSizingFunction>>;

export interface Single<T> {
  Single: T;
}

export type Repeat<R, T> = [R, T];

export type NonRepeatedTrackSizingFunction = MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>;

export interface MinMax<Min, Max> {
  min: Min;
  max: Max;
}

export type MinTrackSizingFunction = Fixed<LengthPercentage> | MinContent | MaxContent | Auto;

export type MaxTrackSizingFunction =
  | Fixed<LengthPercentage>
  | MinContent
  | MaxContent
  | FitContent<LengthPercentage>
  | Auto
  | Fraction<number>;

export interface Fixed<T> {
  Fixed: T;
}

export interface FitContent<T> {
  FitContent: T;
}

export interface Fraction<T> {
  Fraction: T;
}

export type MinContent = "MinContent";
export type MaxContent = "MaxContent";

export type GridTrackRepetition = AutoFill | AutoFit | Count<number>;

export type AutoFill = "AutoFill";

export type AutoFit = "AutoFit";

export interface Count<T> {
  Count: T;
}

export type GridAutoFlow = "Row" | "Column" | "RowDense" | "ColumnDense";

export interface Line<T> {
  start: T;
  end: T;
}

export type GridPlacement = GenericGridPlacement<GridLine>;

export type GenericGridPlacement<LineType> = Auto | Line<LineType> | Span<number>;

export interface GridLine<T = number> {
  GridLine: T;
}

export interface Span<T = number> {
  Span: T;
}

export function taffyFromCss<T extends Record<string, unknown>>(css: T): Partial<Style> {
  const size: Size<Dimension> = { width: "Auto", height: "Auto" };
  const min_size: Size<Dimension> = { width: "Auto", height: "Auto" };
  const max_size: Size<Dimension> = { width: "Auto", height: "Auto" };

  const taffy: Partial<Style> = { size, min_size, max_size };

  for (const [key, value] of Object.entries(css)) {
    match(key)
      .with("display", () => {
        taffy.display = pipe(value, isString, toDisplay);
      })
      .with("boxSizing", () => {
        taffy.box_sizing = pipe(value, isString, toBoxSizing);
      })
      .with("overflow", () => {
        taffy.overflow = pipe(value, isString, toShorthand2, map2(toOverflow), toPoint);
      })
      .with("position", () => {
        taffy.position = pipe(value, isString, toPosition);
      })
      .with("inset", () => {
        taffy.inset = pipe(value, isStringOrNum, toShorthand4, map4(toLengthPercentageAuto), toRect);
      })
      .with("width", () => {
        size.width = pipe(value, isStringOrNum, toLengthPercentageAuto);
      })
      .with("height", () => {
        size.height = pipe(value, isStringOrNum, toLengthPercentageAuto);
      })
      .with("minWidth", () => {
        min_size.width = pipe(value, isStringOrNum, toLengthPercentageAuto);
      })
      .with("minHeight", () => {
        min_size.height = pipe(value, isStringOrNum, toLengthPercentageAuto);
      })
      .with("maxWidth", () => {
        max_size.width = pipe(value, isStringOrNum, toLengthPercentageAuto);
      })
      .with("maxHeight", () => {
        max_size.height = pipe(value, isStringOrNum, toLengthPercentageAuto);
      })
      .with("aspectRatio", () => {
        taffy.aspect_ratio = pipe(value, isStringOrNum, toAspectRatio);
      })
      .with("margin", () => {
        taffy.margin = pipe(value, isStringOrNum, toShorthand4, map4(toLengthPercentageAuto), toRect);
      })
      .with("padding", () => {
        taffy.padding = pipe(value, isStringOrNum, toShorthand4, map4(toLengthPercentage), toRect);
      })
      .with("border", () => {
        taffy.border = pipe(value, isStringOrNum, toShorthand4, map4(toLengthPercentage), toRect);
      })
      .with("alignItems", () => {
        taffy.align_items = pipe(value, isString, toAlignItems);
      })
      .with("alignSelf", () => {
        taffy.align_self = pipe(value, isString, toAlignItems);
      })
      .with("justifyItems", () => {
        taffy.justify_items = pipe(value, isString, toAlignItems);
      })
      .with("justifySelf", () => {
        taffy.justify_self = pipe(value, isString, toAlignItems);
      })
      .with("alignContent", () => {
        taffy.align_content = pipe(value, isString, toAlignContent);
      })
      .with("justifyContent", () => {
        taffy.justify_content = pipe(value, isString, toAlignContent);
      })
      .with("gap", () => {
        taffy.gap = pipe(value, isStringOrNum, toShorthand2, map2(toLengthPercentage), toSize);
      })
      .with("textAlign", () => {
        taffy.text_align = pipe(value, isString, toTextAlign);
      })
      .with("flexDirection", () => {
        taffy.flex_direction = pipe(value, isString, toFlexDirection);
      })
      .with("flexWrap", () => {
        taffy.flex_wrap = pipe(value, isString, toFlexWrap);
      })
      .with("flexBasis", () => {
        taffy.flex_basis = pipe(value, isStringOrNum, toLengthPercentageAuto);
      })
      .with("flexGrow", () => {
        taffy.flex_grow = pipe(value, isNumber);
      })
      .with("flexShrink", () => {
        taffy.flex_shrink = pipe(value, isNumber);
      })
      .with("gridTemplateRows", () => {
        taffy.grid_template_rows = pipe(value, isString, split, map(toTrackSizingFunction));
      });
  }

  return taffy;
}

/*
 * to values
 */

function toDisplay(value: string): Display {
  return match<string, Display>(value)
    .with("block", () => "Block")
    .with("flex", () => "Flex")
    .with("grid", () => "Grid")
    .with("none", () => "None")
    .otherwise(unknownProp("display", value));
}

function toBoxSizing(value: string): BoxSizing {
  return match<string, BoxSizing>(value)
    .with("border-box", () => "BorderBox")
    .with("content-box", () => "ContentBox")
    .otherwise(unknownProp("boxSizing", value));
}

function toOverflow(value: string): Overflow {
  return match<string, Overflow>(value)
    .with("visible", () => "Visible")
    .with("hidden", () => "Hidden")
    .with("clip", () => "Clip")
    .with("scroll", () => "Scroll")
    .otherwise(unknownProp("overflow", value));
}

function toPosition(value: string): Position {
  return match<string, Position>(value)
    .with("absolute", () => "Absolute")
    .with("relative", () => "Relative")
    .otherwise(unknownProp("position", value));
}

function toAlignItems(value: string): AlignItems {
  return match<string, AlignItems>(value)
    .with("start", () => "Start")
    .with("end", () => "End")
    .with("flex-start", () => "FlexStart")
    .with("flex-end", () => "FlexEnd")
    .with("center", () => "Center")
    .with("baseline", () => "Baseline")
    .with("stretch", () => "Stretch")
    .otherwise(unknownValue(value));
}

function toAlignContent(value: string): AlignContent {
  return match<string, AlignContent>(value)
    .with("start", () => "Start")
    .with("end", () => "End")
    .with("flex-start", () => "FlexStart")
    .with("flex-end", () => "FlexEnd")
    .with("center", () => "Center")
    .with("stretch", () => "Stretch")
    .with("space-between", () => "SpaceBetween")
    .with("space-evenly", () => "SpaceEvenly")
    .with("space-around", () => "SpaceAround")
    .otherwise(unknownValue(value));
}

function toTextAlign(value: string): TextAlign {
  return match<string, TextAlign>(value)
    .with("auto", () => "Auto")
    .with("legacy-left", () => "LegacyLeft")
    .with("legacy-right", () => "LegacyRight")
    .with("legacy-center", () => "LegacyCenter")
    .otherwise(unknownProp("textAlign", value));
}

function toFlexDirection(value: string): FlexDirection {
  return match<string, FlexDirection>(value)
    .with("row", () => "Row")
    .with("column", () => "Column")
    .with("row-reverse", () => "RowReverse")
    .with("column-reverse", () => "ColumnReverse")
    .otherwise(unknownProp("flexDirection", value));
}

function toFlexWrap(value: string): FlexWrap {
  return match<string, FlexWrap>(value)
    .with("nowrap", () => "NoWrap")
    .with("wrap", () => "Wrap")
    .with("wrap-reverse", () => "WrapReverse")
    .otherwise(unknownProp("flexWrap", value));
}

function toLengthPercentageAuto(value: string | number): LengthPercentageAuto {
  return match(value)
    .with(P.string.endsWith("%"), v => toPercent(parseFloat(v) / 100))
    .with(P.string.endsWith("px"), v => toLength(parseFloat(v)))
    .with(P.number, toLength)
    .with("auto", toAuto)
    .otherwise(unknownValue(value));
}

function toLengthPercentage(value: string | number): LengthPercentage {
  return match(value)
    .with(P.string.endsWith("%"), v => toPercent(parseFloat(v) / 100))
    .with(P.string.endsWith("px"), v => toLength(parseFloat(v)))
    .with(P.number, toLength)
    .otherwise(unknownValue(value));
}

function toAspectRatio(value: string | number): number {
  return match(value)
    .with(P.number, v => v)
    .with(P.string, v => {
      return z
        .tuple([z.string(), z.string()])
        .pipe(z.transform(([a, b]) => parseFloat(a) / parseFloat(b)))
        .pipe(z.number())
        .parse(v.split("/"));
    })
    .otherwise(unknownValue(value));
}

function toTrackSizingFunction(value: string): TrackSizingFunction {
  const repeat = /repeat\(\s*(?<count>\S+)\s*,\s*(?<value>\S+)\s*\)/;

  return match(value)
    .with(P.string.regex(repeat), v => {
      return z
        .tuple([z.string(), z.string(), z.string()])
        .pipe(
          z.transform(([, count, value]) =>
            toRepeat(toGridTrackRepetition(count), pipe(value, split, map(toNonRepeatedTrackSizingFunction)))
          )
        )
        .parse(v.match(repeat)?.groups);
    })
    .with(P.string, v => {
      return z
        .tuple([z.string(), z.string()])
        .pipe(z.transform(([, value]) => toSingle(toNonRepeatedTrackSizingFunction(value))))
        .parse(v.split(/\s+/));
    })
    .otherwise(unknownValue(value));
}

function toNonRepeatedTrackSizingFunction(value: string): NonRepeatedTrackSizingFunction {
  const minMax = /minmax\(\s*(?<min>\S+)\s*,\s*(?<max>\S+)\s*\)/;

  return match(value)
    .with(P.string.regex(minMax), v => {
      return z
        .tuple([z.string(), z.string(), z.string()])
        .pipe(z.transform(([, min, max]) => toMinMax([toMinTrackSizingFunction(min), toMaxTrackSizingFunction(max)])))
        .parse(v.match(minMax)?.groups);
    })
    .otherwise(unknownValue(value));
}

function toMinTrackSizingFunction(value: string): MinTrackSizingFunction {
  return match<string, MinTrackSizingFunction>(value)
    .with("min-content", () => "MinContent")
    .with("max-content", () => "MaxContent")
    .with(P.string, v => pipe(v, toLengthPercentage, toFixed))
    .otherwise(unknownValue(value));
}

function toMaxTrackSizingFunction(value: string): MaxTrackSizingFunction {
  const fitContent = /fit-content\(\s*(?<arg>\S+)\s*\)/;
  const fraction = /(?<value>[0-9.]+)fr/;

  return match<string, MaxTrackSizingFunction>(value)
    .with("min-content", () => "MinContent")
    .with("max-content", () => "MaxContent")
    .with(P.string.regex(fraction), v => {
      return z
        .tuple([z.string(), z.string()])
        .pipe(z.transform(([, fragVal]) => toFraction(parseFloat(fragVal))))
        .parse(v.match(fraction));
    })
    .with(P.string.regex(fitContent), v => {
      return z
        .tuple([z.string(), z.string()])
        .pipe(z.transform(([, arg]) => toLengthPercentage(arg)))
        .pipe(z.transform(toFitContent))
        .parse(v.match(fitContent));
    })
    .with(P.string, v => pipe(v, toLengthPercentage, toFixed))
    .otherwise(unknownValue(value));
}

function toGridTrackRepetition(value: string): GridTrackRepetition {
  return match<string, GridTrackRepetition>(value)
    .with("auto-fill", () => "AutoFill")
    .with("auto-fit", () => "AutoFit")
    .with(P.string, v => {
      return z
        .string()
        .pipe(z.transform(([, count]) => toCount(parseInt(count))))
        .parse(v);
    })
    .otherwise(unknownValue(value));
}

/*
 * checks
 */

function isString(value: unknown): string {
  return z.string().parse(value);
}

function isNumber(value: unknown): number {
  return z.number().parse(value);
}

function isStringOrNum(value: unknown): string | number {
  return z.union([z.string(), z.number()]).parse(value);
}

/*
 * containers
 */

function toLength<T>(Length: T): Length<T> {
  return { Length };
}

function toPercent<T>(Percent: T): Percent<T> {
  return { Percent };
}

function toFixed<T>(Fixed: T): Fixed<T> {
  return { Fixed };
}

function toFitContent<T>(FitContent: T): FitContent<T> {
  return { FitContent };
}

function toFraction<T>(Fraction: T): Fraction<T> {
  return { Fraction };
}

function toCount<T>(Count: T): Count<T> {
  return { Count };
}

function toSingle<T>(Single: T): Single<T> {
  return { Single };
}

function toRepeat<T, R>(Count: T, value: R): Repeat<T, R> {
  return [Count, value];
}

function toPoint<T>([x, y]: [T, T]): Point<T> {
  return { x, y };
}

function toSize<T>([width, height]: [T, T]): Size<T> {
  return { width, height };
}

function toMinMax<Min, Max>([min, max]: [Min, Max]): MinMax<Min, Max> {
  return { min, max };
}

function toRect<T>([left, right, top, bottom]: [T, T, T, T]): Rect<T> {
  return { left, right, top, bottom };
}

function toAuto(): Auto {
  return "Auto";
}

/*
 * multiples
 */

function map<T, U>(fn: (value: T) => U): (values: T[]) => U[] {
  return values => values.map(fn);
}

function map2<T, U>(fn: (value: T) => U): (values: [T, T]) => [U, U] {
  return values => values.map(fn) as [U, U];
}

function map4<T, U>(fn: (value: T) => U): (values: [T, T, T, T]) => [U, U, U, U] {
  return values => values.map(fn) as [U, U, U, U];
}

function toShorthand2(value: string): [string, string];
function toShorthand2(value: number): [number, number];
function toShorthand2(value: string | number): [string, string] | [number, number];
function toShorthand2(value: string | number): [string, string] | [number, number] {
  return match(value)
    .with(P.number, v => z.tuple([z.number(), z.number()]).parse([v, v]))
    .with(P.string, v =>
      z
        .array(z.string())
        .min(1)
        .max(2)
        .transform(([first, second]) => [first, second ?? first])
        .pipe(z.tuple([z.string(), z.string()]))
        .parse(v.split(/\s+/))
    )
    .exhaustive();
}

function toShorthand4(value: string): [string, string, string, string];
function toShorthand4(value: number): [number, number, number, number];
function toShorthand4(value: string | number): [string, string, string, string] | [number, number, number, number];
function toShorthand4(value: string | number): [string, string, string, string] | [number, number, number, number] {
  return match(value)
    .with(P.number, v => z.tuple([z.number(), z.number(), z.number(), z.number()]).parse([v, v, v, v]))
    .with(P.string, v =>
      z
        .array(z.string())
        .min(1)
        .max(4)
        .transform(values => {
          const [a, b, c, d] = values;
          switch (values.length) {
            case 1:
              return [a, a, a, a];
            case 2:
              return [a, b, a, b];
            case 3:
              return [a, b, c, b];
            case 4:
              return [a, b, c, d];
            default:
              throw new Error("Invalid number of values for shorthand4");
          }
        })
        .pipe(z.tuple([z.string(), z.string(), z.string(), z.string()]))
        .parse(v.split(/\s+/))
    )
    .exhaustive();
}

function split(value: string): string[] {
  return value.split(/\s+/);
}

/*
 * error handling
 */

function unknownProp(key: string, value: unknown): () => never {
  return () => {
    throw new Error(`Unknown CSS property "${key}: ${value}"`);
  };
}

function unknownValue(value: unknown): () => never {
  return () => {
    throw new Error(`Unknown Value "${value}"`);
  };
}
