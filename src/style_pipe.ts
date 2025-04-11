import { pipe } from "npm:fp-ts/lib/function.js";
import * as R from "npm:fp-ts/lib/Record.js";
import * as A from "npm:fp-ts/lib/Array.js";
import { match, P } from "npm:ts-pattern";
import * as toCase from "npm:change-case";

interface Point<T> {
  x: T;
  y: T;
}

interface Size<T> {
  width: T;
  height: T;
}

interface Rect<T> {
  top: T;
  right: T;
  bottom: T;
  left: T;
}

type Length = {
  Length: number;
};

type Percent = {
  Percent: number;
};

type Auto = "Auto";
type LP = Length | Percent;
type LPA = Length | Percent | Auto;

type GridTemplateRows = (Single | Repeat)[];

type Single = {
  Single: MinMax;
};

type Repeat = {
  Repeat: [GridTrackRepetition, MinMax[]];
};

type GridTrackRepetition =
  | "AutoFill"
  | "AutoFit"
  | {
      Count: number;
    };

type TrackListValue = Single;
type TrackList = TrackListValue[];

interface MinMax<Min = MinTrackSizingFunction, Max = MaxTrackSizingFunction> {
  min: Min;
  max: Max;
}

type MinTrackSizingFunction = "MinContent" | "MaxContent" | "Auto" | Fixed;
type MaxTrackSizingFunction = MinTrackSizingFunction | { FitContent: LP } | { Fraction: number };

type Fixed = {
  Fixed: LP;
};

type Prop<Value> = readonly [key: string, value: Value];

export function toTaffy(css: Record<string, string | number>) {
  return pipe(
    css,
    R.toEntries,
    A.map(([key, value]): Prop<string | number | Point<string> | Rect<LPA> | Size<LPA> | TrackList> => {
      return (
        match([key, value])
          .with(["display", P.string], str)
          .with(["boxSizing", P.string], str)
          .with(["overflow", P.string], pair => pipe(pair, shorthand2, point))
          .with(["position", P.string], str)
          .with(["inset", P.union(P.string, P.number)], pair => pipe(pair, shorthand4, rect))
          //.with(["size", P.union(P.string, P.number)], pair => pipe(pair, shorthand2, size))
          .with(["width", P.union(P.string, P.number)], length_pair)
          .with(["height", P.union(P.string, P.number)], length_pair)
          .with(["minSize", P.union(P.string, P.number)], pair => pipe(pair, shorthand2, size))
          .with(["maxSize", P.union(P.string, P.number)], pair => pipe(pair, shorthand2, size))
          .with(["aspectRatio", P.union(P.string, P.number)], aspectRatio)
          .with(["margin", P.union(P.string, P.number)], pair => pipe(pair, shorthand4, rect))
          .with(["padding", P.union(P.string, P.number)], pair => pipe(pair, shorthand4, rect))
          .with(["border", P.union(P.string, P.number)], pair => pipe(pair, shorthand4, rect))
          .with(["alignItems", P.string], str)
          .with(["alignSelf", P.string], str)
          .with(["justifyItems", P.string], str)
          .with(["alignContent", P.string], str)
          .with(["justifySelf", P.string], str)
          .with(["alignContent", P.string], str)
          .with(["justifyContent", P.string], str)
          .with(["gap", P.union(P.string, P.number)], pair => pipe(pair, shorthand2, size))
          .with(["flexDirection", P.string], str)
          .with(["flexWrap", P.string], flexWrap)
          .with(["flexBasis", P.union(P.string, P.number)], pair => pipe(pair, shorthand2, size))
          .with(["flexGrow", P.union(P.string, P.number)], num)
          .with(["flexShrink", P.union(P.string, P.number)], num)
          .with(
            ["gridTemplateRows", P.union(P.string, P.number)],
            (pair): Prop<TrackList> => [
              toCase.snakeCase(pair[0]),
              pipe(pair, valueList, ([, values]) => values.map(v => trackListValue(v))),
            ]
          )
          .run()
      );
    }),
    // convert the entries ["width", ..], ["height", ..] into ["size", { width: .., height: .. }]
    mergeSize,
    A.map((a): [key: string, value: string | number | Point<string> | Rect<LPA> | Size<LPA> | TrackList] => [...a]), // turn readonly into mutable
    R.fromEntries
  );
}

function mergeSize(entries: any[]) {
  const rest = entries.filter(([k]) => k !== "width" && k !== "height");

  const width = entries.find(([k]) => k === "width")?.[1] ?? "Auto";
  const height = entries.find(([k]) => k === "height")?.[1] ?? "Auto";

  return [...rest, ["size", { width, height }]];
}

function trackListValue(value: string): Single {
  return match(value)
    .with(P.string.endsWith("px"), v => single(minMax(fixed(length(v)))))
    .with(P.string.endsWith("%"), v => single(minMax(fixed(percent(v)))))
    .with(P.string.regex(/^\d+$/), v => single(minMax(fixed(length(v)))))
    .with(P.number, v => single(minMax(fixed(length(v)))))
    .with("min-content", () => single(minMax("MinContent")))
    .with("max-content", () => single(minMax("MaxContent")))
    .with("auto", () => single(minMax("Auto")))
    .otherwise(() => {
      throw new Error(`Invalid value for track sizing function: ${value}`);
    });
}

function length_pair([key, value]: Prop<string>): Prop<LPA> {
  return [toCase.snakeCase(key), lpa(value)];
}

function length(value: string): Length {
  return { Length: parseFloat(value) };
}

function percent(value: string): Percent {
  return { Percent: parseFloat(value) / 100 };
}

function fixed(value: LP): Fixed {
  return { Fixed: value };
}

function single(value: MinMax): Single {
  return { Single: value };
}

function minMax<Min extends MinTrackSizingFunction, Max extends MaxTrackSizingFunction = Min>(
  min: Min,
  max?: Max
): Max extends undefined ? MinMax<Min, Min> : MinMax<Min, Max> {
  return { min, max: max ?? min } as Max extends undefined ? MinMax<Min, Min> : MinMax<Min, Max>;
}

function str([key, value]: Prop<string>): Prop<string> {
  return [toCase.snakeCase(key), toCase.pascalCase(value)];
}

function num([key, value]: Prop<string | number>): Prop<number> {
  return match(value)
    .with(P.string.endsWith("px"), (s): Prop<number> => [toCase.snakeCase(key), parseFloat(s)])
    .with(P.string.endsWith("%"), (s): Prop<number> => [toCase.snakeCase(key), parseFloat(s) / 100])
    .with(P.number, (n): Prop<number> => [toCase.snakeCase(key), n])
    .otherwise(() => {
      throw new Error(`Invalid value for length or percent: ${value}`);
    });
}

function aspectRatio([key, value]: Prop<string | number>): Prop<number> {
  return match(value)
    .with(P.number, (n): Prop<number> => [toCase.snakeCase(key), n])
    .with(P.string.regex(/^\d+\/\d+$/), (s): Prop<number> => {
      const [a, b] = s.split("/");
      return [toCase.snakeCase(key), parseFloat(a) / parseFloat(b)];
    })
    .otherwise(() => {
      throw new Error(`Invalid value for aspect ratio: ${value}`);
    });
}

function point([key, [x, y]]: Prop<[string, string]>): Prop<Point<string>> {
  return [toCase.snakeCase(key), { x: toCase.pascalCase(x), y: toCase.pascalCase(y) }];
}

function lpa(value: string | number): LPA {
  return match(value)
    .with("auto", (): Auto => "Auto")
    .with(P.string.endsWith("%"), s => ({ Percent: parseFloat(s) / 100 }))
    .with(P.string.endsWith("px"), s => ({ Length: parseFloat(s) }))
    .with(P.string.regex(/^\d+$/), s => ({ Length: parseFloat(s) }))
    .with(P.number, n => ({ Length: n }))
    .otherwise(() => {
      throw new Error(`Invalid value for length or percent: ${value}`);
    });
}

function rect([key, [top, right, bottom, left]]: Prop<[string, string, string, string]>): Prop<Rect<LPA>> {
  return [
    toCase.snakeCase(key),
    {
      top: lpa(top),
      right: lpa(right),
      bottom: lpa(bottom),
      left: lpa(left),
    },
  ];
}

function size([key, [width, height]]: Prop<[string | number, string | number]>): Prop<Size<LPA>> {
  return [
    toCase.snakeCase(key),
    {
      width: lpa(width),
      height: lpa(height),
    },
  ];
}

function shorthand2([key, value]: Prop<string | number>): Prop<[string, string]> {
  const [a, b] = String(value).split(" ");
  return [key, [a, b || a]];
}

function shorthand4<V = string | number>([key, value]: Prop<V>): Prop<[string, string, string, string]> {
  const values = String(value).split(" ");
  const [a, b, c, d] = values;
  switch (values.length) {
    case 1:
      return [key, [a, a, a, a]];
    case 2:
      return [key, [a, b, a, b]];
    case 3:
      return [key, [a, b, c, b]];
    case 4:
      return [key, [a, b, c, d]];
    default:
      throw new Error("Invalid number of values for shorthand");
  }
}

function valueList([key, value]: Prop<string | number>): Prop<string[]> {
  return [key, String(value).split(" ")];
}

function flexWrap([key, value]: Prop<string>): Prop<string> {
  return match(value)
    .with("nowrap", (): Prop<string> => [toCase.snakeCase(key), "NoWrap"])
    .with("wrap", (): Prop<string> => [toCase.snakeCase(key), "Wrap"])
    .with("wrap-reverse", (): Prop<string> => [toCase.snakeCase(key), "WrapReverse"])
    .otherwise(() => {
      throw new Error(`Invalid value for flex-wrap: ${value}`);
    });
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest;

  const rect = {
    px: (top: number, right: number, bottom: number, left: number) => ({
      top: { Length: top },
      right: { Length: right },
      bottom: { Length: bottom },
      left: { Length: left },
    }),
    percent: (top: number, right: number, bottom: number, left: number) => ({
      top: { Percent: top },
      right: { Percent: right },
      bottom: { Percent: bottom },
      left: { Percent: left },
    }),
    auto: (top: Auto, right: Auto, bottom: Auto, left: Auto) => ({
      top,
      right,
      bottom,
      left,
    }),
  };

  const size = {
    px: (width: number, height: number) => ({
      width: { Length: width },
      height: { Length: height },
    }),
    percent: (width: number, height: number) => ({
      width: { Percent: width },
      height: { Percent: height },
    }),
    auto: (width: Auto, height: Auto) => ({
      width,
      height,
    }),
  };

  test("display", () => {
    expect(toTaffy({ display: "block" })).toEqual({ display: "Block" });
    expect(toTaffy({ display: "flex" })).toEqual({ display: "Flex" });
    expect(toTaffy({ display: "grid" })).toEqual({ display: "Grid" });
    expect(toTaffy({ display: "none" })).toEqual({ display: "None" });
  });

  test("position", () => {
    expect(toTaffy({ position: "relative" })).toEqual({ position: "Relative" });
    expect(toTaffy({ position: "absolute" })).toEqual({ position: "Absolute" });
  });

  test("box-sizing", () => {
    expect(toTaffy({ boxSizing: "border-box" })).toEqual({ box_sizing: "BorderBox" });
    expect(toTaffy({ boxSizing: "content-box" })).toEqual({ box_sizing: "ContentBox" });
  });

  test("overflow", () => {
    expect(toTaffy({ overflow: "visible" })).toEqual({ overflow: { x: "Visible", y: "Visible" } });
    expect(toTaffy({ overflow: "clip" })).toEqual({ overflow: { x: "Clip", y: "Clip" } });
    expect(toTaffy({ overflow: "hidden" })).toEqual({ overflow: { x: "Hidden", y: "Hidden" } });
    expect(toTaffy({ overflow: "scroll" })).toEqual({ overflow: { x: "Scroll", y: "Scroll" } });
    expect(toTaffy({ overflow: "visible scroll" })).toEqual({ overflow: { x: "Visible", y: "Scroll" } });
    expect(toTaffy({ overflow: "clip hidden" })).toEqual({ overflow: { x: "Clip", y: "Hidden" } });
  });

  test("inset", () => {
    expect(toTaffy({ inset: 10 })).toEqual({ inset: rect.px(10, 10, 10, 10) });
    expect(toTaffy({ inset: "10px" })).toEqual({ inset: rect.px(10, 10, 10, 10) });
    expect(toTaffy({ inset: "10%" })).toEqual({ inset: rect.percent(0.1, 0.1, 0.1, 0.1) });
    expect(toTaffy({ inset: "auto" })).toEqual({ inset: rect.auto("Auto", "Auto", "Auto", "Auto") });
    expect(toTaffy({ inset: "1px 2px" })).toEqual({ inset: rect.px(1, 2, 1, 2) });
    expect(toTaffy({ inset: "1px 2px 3px" })).toEqual({ inset: rect.px(1, 2, 3, 2) });
    expect(toTaffy({ inset: "1px 2px 3px 4px" })).toEqual({ inset: rect.px(1, 2, 3, 4) });
  });

  test("size", () => {
    expect(toTaffy({ size: 10 })).toEqual({ size: size.px(10, 10) });
    expect(toTaffy({ size: "10px" })).toEqual({ size: size.px(10, 10) });
    expect(toTaffy({ size: "10%" })).toEqual({ size: size.percent(0.1, 0.1) });
    expect(toTaffy({ size: "auto" })).toEqual({ size: size.auto("Auto", "Auto") });
    expect(toTaffy({ size: "1px 2px" })).toEqual({ size: size.px(1, 2) });
  });

  test("min-size", () => {
    expect(toTaffy({ minSize: 10 })).toEqual({ min_size: size.px(10, 10) });
    expect(toTaffy({ minSize: "10px" })).toEqual({ min_size: size.px(10, 10) });
    expect(toTaffy({ minSize: "10%" })).toEqual({ min_size: size.percent(0.1, 0.1) });
    expect(toTaffy({ minSize: "auto" })).toEqual({ min_size: size.auto("Auto", "Auto") });
    expect(toTaffy({ minSize: "1px 2px" })).toEqual({ min_size: size.px(1, 2) });
  });

  test("max-size", () => {
    expect(toTaffy({ maxSize: 10 })).toEqual({ max_size: size.px(10, 10) });
    expect(toTaffy({ maxSize: "10px" })).toEqual({ max_size: size.px(10, 10) });
    expect(toTaffy({ maxSize: "10%" })).toEqual({ max_size: size.percent(0.1, 0.1) });
    expect(toTaffy({ maxSize: "auto" })).toEqual({ max_size: size.auto("Auto", "Auto") });
    expect(toTaffy({ maxSize: "1px 2px" })).toEqual({ max_size: size.px(1, 2) });
  });

  test("aspect-ratio", () => {
    expect(toTaffy({ aspectRatio: 1 })).toEqual({ aspect_ratio: 1 });
    expect(toTaffy({ aspectRatio: 1.5 })).toEqual({ aspect_ratio: 1.5 });
    expect(toTaffy({ aspectRatio: "1/2" })).toEqual({ aspect_ratio: 0.5 });
    expect(toTaffy({ aspectRatio: "16/9" })).toEqual({ aspect_ratio: 1.7777777777777777 });
  });

  test("margin", () => {
    expect(toTaffy({ margin: 10 })).toEqual({ margin: rect.px(10, 10, 10, 10) });
    expect(toTaffy({ margin: "10px" })).toEqual({ margin: rect.px(10, 10, 10, 10) });
    expect(toTaffy({ margin: "10%" })).toEqual({ margin: rect.percent(0.1, 0.1, 0.1, 0.1) });
    expect(toTaffy({ margin: "auto" })).toEqual({ margin: rect.auto("Auto", "Auto", "Auto", "Auto") });
    expect(toTaffy({ margin: "1px 2px" })).toEqual({ margin: rect.px(1, 2, 1, 2) });
    expect(toTaffy({ margin: "1px 2px 3px" })).toEqual({ margin: rect.px(1, 2, 3, 2) });
    expect(toTaffy({ margin: "1px 2px 3px 4px" })).toEqual({ margin: rect.px(1, 2, 3, 4) });
  });

  test("padding", () => {
    expect(toTaffy({ padding: 10 })).toEqual({ padding: rect.px(10, 10, 10, 10) });
    expect(toTaffy({ padding: "10px" })).toEqual({ padding: rect.px(10, 10, 10, 10) });
    expect(toTaffy({ padding: "10%" })).toEqual({ padding: rect.percent(0.1, 0.1, 0.1, 0.1) });
    expect(toTaffy({ padding: "1px 2px" })).toEqual({ padding: rect.px(1, 2, 1, 2) });
    expect(toTaffy({ padding: "1px 2px 3px" })).toEqual({ padding: rect.px(1, 2, 3, 2) });
    expect(toTaffy({ padding: "1px 2px 3px 4px" })).toEqual({ padding: rect.px(1, 2, 3, 4) });
  });

  test("border", () => {
    expect(toTaffy({ border: 10 })).toEqual({ border: rect.px(10, 10, 10, 10) });
    expect(toTaffy({ border: "10px" })).toEqual({ border: rect.px(10, 10, 10, 10) });
    expect(toTaffy({ border: "10%" })).toEqual({ border: rect.percent(0.1, 0.1, 0.1, 0.1) });
    expect(toTaffy({ border: "1px 2px" })).toEqual({ border: rect.px(1, 2, 1, 2) });
    expect(toTaffy({ border: "1px 2px 3px" })).toEqual({ border: rect.px(1, 2, 3, 2) });
    expect(toTaffy({ border: "1px 2px 3px 4px" })).toEqual({ border: rect.px(1, 2, 3, 4) });
  });

  test("align-items", () => {
    expect(toTaffy({ alignItems: "start" })).toEqual({ align_items: "Start" });
    expect(toTaffy({ alignItems: "end" })).toEqual({ align_items: "End" });
    expect(toTaffy({ alignItems: "flex-start" })).toEqual({ align_items: "FlexStart" });
    expect(toTaffy({ alignItems: "flex-end" })).toEqual({ align_items: "FlexEnd" });
    expect(toTaffy({ alignItems: "center" })).toEqual({ align_items: "Center" });
    expect(toTaffy({ alignItems: "baseline" })).toEqual({ align_items: "Baseline" });
    expect(toTaffy({ alignItems: "stretch" })).toEqual({ align_items: "Stretch" });
  });

  test("align-self", () => {
    expect(toTaffy({ alignSelf: "start" })).toEqual({ align_self: "Start" });
    expect(toTaffy({ alignSelf: "end" })).toEqual({ align_self: "End" });
    expect(toTaffy({ alignSelf: "flex-start" })).toEqual({ align_self: "FlexStart" });
    expect(toTaffy({ alignSelf: "flex-end" })).toEqual({ align_self: "FlexEnd" });
    expect(toTaffy({ alignSelf: "center" })).toEqual({ align_self: "Center" });
    expect(toTaffy({ alignSelf: "baseline" })).toEqual({ align_self: "Baseline" });
    expect(toTaffy({ alignSelf: "stretch" })).toEqual({ align_self: "Stretch" });
  });

  test("justify-items", () => {
    expect(toTaffy({ justifyItems: "start" })).toEqual({ justify_items: "Start" });
    expect(toTaffy({ justifyItems: "end" })).toEqual({ justify_items: "End" });
    expect(toTaffy({ justifyItems: "flex-start" })).toEqual({ justify_items: "FlexStart" });
    expect(toTaffy({ justifyItems: "flex-end" })).toEqual({ justify_items: "FlexEnd" });
    expect(toTaffy({ justifyItems: "center" })).toEqual({ justify_items: "Center" });
    expect(toTaffy({ justifyItems: "baseline" })).toEqual({ justify_items: "Baseline" });
    expect(toTaffy({ justifyItems: "stretch" })).toEqual({ justify_items: "Stretch" });
  });

  test("justify-self", () => {
    expect(toTaffy({ justifySelf: "start" })).toEqual({ justify_self: "Start" });
    expect(toTaffy({ justifySelf: "end" })).toEqual({ justify_self: "End" });
    expect(toTaffy({ justifySelf: "flex-start" })).toEqual({ justify_self: "FlexStart" });
    expect(toTaffy({ justifySelf: "flex-end" })).toEqual({ justify_self: "FlexEnd" });
    expect(toTaffy({ justifySelf: "center" })).toEqual({ justify_self: "Center" });
    expect(toTaffy({ justifySelf: "baseline" })).toEqual({ justify_self: "Baseline" });
    expect(toTaffy({ justifySelf: "stretch" })).toEqual({ justify_self: "Stretch" });
  });

  test("align-content", () => {
    expect(toTaffy({ alignContent: "start" })).toEqual({ align_content: "Start" });
    expect(toTaffy({ alignContent: "end" })).toEqual({ align_content: "End" });
    expect(toTaffy({ alignContent: "flex-start" })).toEqual({ align_content: "FlexStart" });
    expect(toTaffy({ alignContent: "flex-end" })).toEqual({ align_content: "FlexEnd" });
    expect(toTaffy({ alignContent: "center" })).toEqual({ align_content: "Center" });
    expect(toTaffy({ alignContent: "stretch" })).toEqual({ align_content: "Stretch" });
    expect(toTaffy({ alignContent: "space-between" })).toEqual({ align_content: "SpaceBetween" });
    expect(toTaffy({ alignContent: "space-evenly" })).toEqual({ align_content: "SpaceEvenly" });
    expect(toTaffy({ alignContent: "space-around" })).toEqual({ align_content: "SpaceAround" });
  });

  test("justify-content", () => {
    expect(toTaffy({ justifyContent: "start" })).toEqual({ justify_content: "Start" });
    expect(toTaffy({ justifyContent: "end" })).toEqual({ justify_content: "End" });
    expect(toTaffy({ justifyContent: "flex-start" })).toEqual({ justify_content: "FlexStart" });
    expect(toTaffy({ justifyContent: "flex-end" })).toEqual({ justify_content: "FlexEnd" });
    expect(toTaffy({ justifyContent: "center" })).toEqual({ justify_content: "Center" });
    expect(toTaffy({ justifyContent: "stretch" })).toEqual({ justify_content: "Stretch" });
    expect(toTaffy({ justifyContent: "space-between" })).toEqual({ justify_content: "SpaceBetween" });
    expect(toTaffy({ justifyContent: "space-evenly" })).toEqual({ justify_content: "SpaceEvenly" });
    expect(toTaffy({ justifyContent: "space-around" })).toEqual({ justify_content: "SpaceAround" });
  });

  test("gap", () => {
    expect(toTaffy({ gap: 10 })).toEqual({ gap: size.px(10, 10) });
    expect(toTaffy({ gap: "10px" })).toEqual({ gap: size.px(10, 10) });
    expect(toTaffy({ gap: "10%" })).toEqual({ gap: size.percent(0.1, 0.1) });
    expect(toTaffy({ gap: "1px 2px" })).toEqual({ gap: size.px(1, 2) });
  });

  test("flex-direction", () => {
    expect(toTaffy({ flexDirection: "row" })).toEqual({ flex_direction: "Row" });
    expect(toTaffy({ flexDirection: "column" })).toEqual({ flex_direction: "Column" });
    expect(toTaffy({ flexDirection: "row-reverse" })).toEqual({ flex_direction: "RowReverse" });
    expect(toTaffy({ flexDirection: "column-reverse" })).toEqual({ flex_direction: "ColumnReverse" });
  });

  test("flex-wrap", () => {
    expect(toTaffy({ flexWrap: "nowrap" })).toEqual({ flex_wrap: "NoWrap" });
    expect(toTaffy({ flexWrap: "wrap" })).toEqual({ flex_wrap: "Wrap" });
    expect(toTaffy({ flexWrap: "wrap-reverse" })).toEqual({ flex_wrap: "WrapReverse" });
  });

  test("flex-basis", () => {
    expect(toTaffy({ flexBasis: 10 })).toEqual({ flex_basis: size.px(10, 10) });
    expect(toTaffy({ flexBasis: "10px" })).toEqual({ flex_basis: size.px(10, 10) });
    expect(toTaffy({ flexBasis: "10%" })).toEqual({ flex_basis: size.percent(0.1, 0.1) });
    expect(toTaffy({ flexBasis: "auto" })).toEqual({ flex_basis: size.auto("Auto", "Auto") });
  });

  test("grid-template-rows", () => {
    expect(toTaffy({ gridTemplateRows: "1px 2px" })).toEqual({
      grid_template_rows: [
        {
          Single: {
            min: { Fixed: { Length: 1 } },
            max: { Fixed: { Length: 1 } },
          },
        },
        {
          Single: {
            min: { Fixed: { Length: 2 } },
            max: { Fixed: { Length: 2 } },
          },
        },
      ],
    });
    expect(toTaffy({ gridTemplateRows: "min-content" })).toEqual({
      grid_template_rows: [
        {
          Single: {
            min: "MinContent",
            max: "MinContent",
          },
        },
      ],
    });
    expect(toTaffy({ gridTemplateRows: "max-content" })).toEqual({
      grid_template_rows: [
        {
          Single: {
            min: "MaxContent",
            max: "MaxContent",
          },
        },
      ],
    });
    expect(toTaffy({ gridTemplateRows: "auto" })).toEqual({
      grid_template_rows: [
        {
          Single: {
            min: "Auto",
            max: "Auto",
          },
        },
      ],
    });
  });
}
