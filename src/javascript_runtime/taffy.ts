import { pipe } from "fp-ts/lib/function";
import * as R from "fp-ts/lib/Record";
import * as A from "fp-ts/lib/Array";
import * as S from "fp-ts/string";
import * as O from "fp-ts/lib/Option";
import { match, P } from "ts-pattern";
import * as z from "zod";

import type * as t from "./taffy_types";

export function cssToTaffy<T extends Record<string, unknown>>(css: T): Partial<t.Style> {
  const size: t.Size<t.Dimension> = { width: "Auto", height: "Auto" };
  const taffy: Partial<t.Style> = { size };

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
      });
  }

  return taffy;
}

/*
 * to values
 */

function toDisplay(value: string): t.Display {
  return match<string, t.Display>(value)
    .with("block", () => "Block")
    .with("flex", () => "Flex")
    .with("grid", () => "Grid")
    .with("none", () => "None")
    .otherwise(unknownProp("display", value));
}

function toBoxSizing(value: string): t.BoxSizing {
  return match<string, t.BoxSizing>(value)
    .with("border-box", () => "BorderBox")
    .with("content-box", () => "ContentBox")
    .otherwise(unknownProp("boxSizing", value));
}

function toOverflow(value: string): t.Overflow {
  return match<string, t.Overflow>(value)
    .with("visible", () => "Visible")
    .with("hidden", () => "Hidden")
    .with("clip", () => "Clip")
    .with("scroll", () => "Scroll")
    .otherwise(unknownProp("overflow", value));
}

function toPosition(value: string): t.Position {
  return match<string, t.Position>(value)
    .with("absolute", () => "Absolute")
    .with("relative", () => "Relative")
    .otherwise(unknownProp("position", value));
}

function toLengthPercentageAuto(value: string | number): t.LengthPercentageAuto {
  return match(value)
    .with(P.string.endsWith("%"), v => toPercent(parseFloat(v) / 100))
    .with(P.string.endsWith("px"), v => toLength(parseFloat(v)))
    .with(P.number, toLength)
    .with("auto", toAuto)
    .otherwise(unknownValue(value));
}

/*
 * checks
 */

function isString(value: unknown): string {
  return z.string().parse(value);
}

function isStringOrNum(value: unknown): string | number {
  return z.union([z.string(), z.number()]).parse(value);
}

/*
 * containers
 */

function toLength<T>(Length: T): t.Length<T> {
  return { Length };
}

function toPercent<T>(Percent: T): t.Percent<T> {
  return { Percent };
}

function toPoint<T>([x, y]: [T, T]): t.Point<T> {
  return { x, y };
}

function toRect<T>([left, right, top, bottom]: [T, T, T, T]): t.Rect<T> {
  return { left, right, top, bottom };
}

function toAuto(): t.Auto {
  return "Auto";
}

/*
 * multiples
 */

function map2<T, U>(fn: (value: T) => U): (values: [T, T]) => [U, U] {
  return values => values.map(fn) as [U, U];
}

function map4<T, U>(fn: (value: T) => U): (values: [T, T, T, T]) => [U, U, U, U] {
  return values => values.map(fn) as [U, U, U, U];
}

function toShorthand2(value: string): [string, string] {
  return z
    .array(z.string())
    .min(1)
    .max(2)
    .transform(([first, second]) => [first, second ?? first])
    .pipe(z.tuple([z.string(), z.string()]))
    .parse(value.split(/\s+/));
}

function toShorthand4(value: string | number): [string, string, string, string] | [number, number, number, number] {
  return match(value)
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
    .with(P.number, v => z.tuple([z.number(), z.number(), z.number(), z.number()]).parse([v, v, v, v]))
    .exhaustive();
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
