import { pipe } from "fp-ts/function";
import * as R from "fp-ts/Record";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";
import { match, P } from "ts-pattern";
import { pascalCase } from "change-case";

interface Point<T> {
  x: T;
  y: T;
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

type Percentage = {
  Percentage: number;
};

type Auto = "Auto";
type LPA = Length | Percentage | Auto;

type Prop<Value> = readonly [key: string, value: Value];

function toTaffy(css: Record<string, string | number>) {
  return pipe(
    css,
    R.toEntries,
    A.map(([key, value]): Prop<string | Point<string> | Rect<LPA>> => {
      return match([key, value])
        .with(["display", P.string], pair => stringValue(pair))
        .with(["boxSizing", P.string], stringValue)
        .with(["overflow", P.string], pair => pipe(pair, shorthand2, point))
        .with(["position", P.string], stringValue)
        .with(["inset", P.union(P.string, P.number)], pair => pipe(pair, shorthand4, rect))
        .run();
    }),
    A.map((a): [key: string, value: string | Point<string> | Rect<LPA>] => [...a]), // make readonly mutable
    R.fromEntries
  );
}

function stringValue([key, value]: Prop<string>): Prop<string> {
  return [pascalCase(key), pascalCase(value)];
}

function point([key, [x, y]]: Prop<[string, string]>): Prop<Point<string>> {
  return [pascalCase(key), { x: pascalCase(x), y: pascalCase(y) }];
}

function lpa(value: string): LPA {
  return match(value)
    .with("auto", (): Auto => "Auto")
    .with(P.string.endsWith("%"), s => ({ Percentage: parseFloat(s) / 100 }))
    .with(P.string.endsWith("px"), s => ({ Length: parseFloat(s) }))
    .with(P.string.regex(/^\d+$/), s => ({ Length: parseFloat(s) }))
    .with(P.number, n => ({ Length: parseFloat(n) }))
    .otherwise(() => {
      throw new Error(`Invalid value for length or percentage: ${value}`);
    });
}

function rect([key, [top, right, bottom, left]]: Prop<[string, string, string, string]>): Prop<Rect<LPA>> {
  return [
    pascalCase(key),
    {
      top: lpa(top),
      right: lpa(right),
      bottom: lpa(bottom),
      left: lpa(left),
    },
  ];
}

function shorthand2([key, value]: Prop<string>): Prop<[string, string]> {
  const [a, b] = value.split(" ");
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
      top: { Percentage: top },
      right: { Percentage: right },
      bottom: { Percentage: bottom },
      left: { Percentage: left },
    }),
    auto: (top: Auto, right: Auto, bottom: Auto, left: Auto) => ({
      top,
      right,
      bottom,
      left,
    }),
  };

  test("display", () => {
    expect(toTaffy({ display: "block" })).toEqual({ Display: "Block" });
    expect(toTaffy({ display: "flex" })).toEqual({ Display: "Flex" });
    expect(toTaffy({ display: "grid" })).toEqual({ Display: "Grid" });
    expect(toTaffy({ display: "none" })).toEqual({ Display: "None" });
  });

  test("position", () => {
    expect(toTaffy({ position: "relative" })).toEqual({ Position: "Relative" });
    expect(toTaffy({ position: "absolute" })).toEqual({ Position: "Absolute" });
  });

  test("box-sizing", () => {
    expect(toTaffy({ boxSizing: "border-box" })).toEqual({ BoxSizing: "BorderBox" });
    expect(toTaffy({ boxSizing: "content-box" })).toEqual({ BoxSizing: "ContentBox" });
  });

  test("overflow", () => {
    expect(toTaffy({ overflow: "visible" })).toEqual({ Overflow: { x: "Visible", y: "Visible" } });
    expect(toTaffy({ overflow: "clip" })).toEqual({ Overflow: { x: "Clip", y: "Clip" } });
    expect(toTaffy({ overflow: "hidden" })).toEqual({ Overflow: { x: "Hidden", y: "Hidden" } });
    expect(toTaffy({ overflow: "scroll" })).toEqual({ Overflow: { x: "Scroll", y: "Scroll" } });

    expect(toTaffy({ overflow: "visible scroll" })).toEqual({ Overflow: { x: "Visible", y: "Scroll" } });
    expect(toTaffy({ overflow: "clip hidden" })).toEqual({ Overflow: { x: "Clip", y: "Hidden" } });
  });

  test("inset", () => {
    expect(toTaffy({ inset: 10 })).toEqual({ Inset: rect.px(10, 10, 10, 10) });
    expect(toTaffy({ inset: "10px" })).toEqual({ Inset: rect.px(10, 10, 10, 10) });
    expect(toTaffy({ inset: "10%" })).toEqual({ Inset: rect.percent(0.1, 0.1, 0.1, 0.1) });
    expect(toTaffy({ inset: "auto" })).toEqual({ Inset: rect.auto("Auto", "Auto", "Auto", "Auto") });

    expect(toTaffy({ inset: "1px 2px" })).toEqual({ Inset: rect.px(1, 2, 1, 2) });
    expect(toTaffy({ inset: "1px 2px 3px" })).toEqual({ Inset: rect.px(1, 2, 3, 2) });
    expect(toTaffy({ inset: "1px 2px 3px 4px" })).toEqual({ Inset: rect.px(1, 2, 3, 4) });
  });
}
