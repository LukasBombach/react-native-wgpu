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
type LengthPercentageAuto = Length | Percentage | Auto;
type PropTouple<V = string> = [key: string, value: V];

function toTaffy(css: Record<string, string>) {
  return pipe(
    css,
    R.toEntries,
    A.map(([key, value]): [string, string | Point<string> | Rect<LengthPercentageAuto>] => {
      return match([key, value])
        .with(["display", P.string], toEnum)
        .with(["boxSizing", P.string], toEnum)
        .with(["position", P.string], toEnum)
        .with(["overflow", P.string], toOverflow)
        .run();
    }),
    R.fromEntries
  );
}

function toEnum([key, value]: PropTouple): PropTouple {
  return [pascalCase(key), pascalCase(value)];
}

function toPoint([key, value]: PropTouple): PropTouple<Point<string>> {
  return [pascalCase(key), { x: pascalCase(value), y: pascalCase(value) }];
}

function toOverflow([key, value]: PropTouple<string>): PropTouple<Point<string>> {
  const [x, y] = value.split(" ");
  return [pascalCase(key), { x: pascalCase(x), y: pascalCase(y || x) }];
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
}
