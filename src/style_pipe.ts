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
type PropTouple<V = string> = [key: string, value: V];

function toTaffy(css: Record<string, string>) {
  return pipe(
    css,
    R.toEntries,
    A.map(([key, value]): PropTouple<string> | PropTouple<Point<string>> => {
      return match([key, value])
        .with(["display", P.string], toEnum)
        .with(["boxSizing", P.string], toEnum)
        .with(["overflow", P.string], t => pipe(t, shorthand2, point))
        .with(["position", P.string], toEnum)
        .with(["inset", P.string], t => pipe(t, shorthand4, rect))
        .run();
    }),
    R.fromEntries
  );
}

function toEnum([key, value]: PropTouple): PropTouple {
  return [pascalCase(key), pascalCase(value)];
}

function point([key, [x, y]]: PropTouple): PropTouple<Point<string>> {
  return [pascalCase(key), { x: pascalCase(x), y: pascalCase(y) }];
}

function lpa(value: string): LPA {
  return match(value)
    .with(P.number, n => ({ Length: n }))
    .with(P.string, s => {
      if (s.endsWith("%")) {
        return { Percentage: parseFloat(s) };
      }
      if (s === "auto") {
        return "Auto";
      }
      return { Length: parseFloat(s) };
    })
    .run();
}

function rect([key, [top, right, bottom, left]]: PropTouple): PropTouple<Rect<LPA>> {
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

function shorthand2([key, value]: PropTouple): PropTouple<string[]> {
  const [a, b] = value.split(" ");
  return [key, [a, b || a]];
}

function shorthand4([key, value]: PropTouple): PropTouple<[string, string, string, string]> {
  const values = value.split(" ");
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
    expect(toTaffy({ inset: "0" })).toEqual({
      Inset: { top: { Length: 0 }, right: { Length: 0 }, bottom: { Length: 0 }, left: { Length: 0 } },
    });
    expect(toTaffy({ inset: "10px" })).toEqual({
      Inset: { top: { Length: 10 }, right: { Length: 10 }, bottom: { Length: 10 }, left: { Length: 10 } },
    });
    expect(toTaffy({ inset: "10px 20px" })).toEqual({
      Inset: { top: { Length: 10 }, right: { Length: 20 }, bottom: { Length: 10 }, left: { Length: 20 } },
    });
    expect(toTaffy({ inset: "10px 20px 30px" })).toEqual({
      Inset: {
        top: { Length: 10 },
        right: { Length: 20 },
        bottom: { Length: 30 },
        left: { Length: 20 },
      },
    });
    expect(toTaffy({ inset: "10px 20px 30px 40px" })).toEqual({
      Inset: {
        top: { Length: 10 },
        right: { Length: 20 },
        bottom: { Length: 30 },
        left: { Length: 40 },
      },
    });
  });
}
