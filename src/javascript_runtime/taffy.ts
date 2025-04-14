import { pipe } from "fp-ts/lib/function";
import * as R from "fp-ts/lib/Record";
import * as A from "fp-ts/lib/Array";
import * as S from "fp-ts/string";
import * as O from "fp-ts/lib/Option";
import { match, P } from "ts-pattern";
import * as z from "zod";

import type * as t from "./taffy_types";

export function cssToTaffy<T extends Record<string, unknown>>(css: T): Partial<t.Style> {
  const taffy: Partial<t.Style> = {};

  for (const [key, value] of Object.entries(css)) {
    const wtf = thrw(key, value);

    match(key)
      .with("display", () => {
        taffy["display"] = match<unknown, t.Display | undefined>(value)
          .with("block", () => "Block")
          .with("flex", () => "Flex")
          .with("grid", () => "Grid")
          .with("none", () => "None")
          .otherwise(wtf);
      })
      .with("boxSizing", () => {
        taffy["box_sizing"] = match<unknown, t.BoxSizing | undefined>(value)
          .with("border-box", () => "BorderBox")
          .with("content-box", () => "ContentBox")
          .otherwise(wtf);
      })
      .with("overflow", () => {
        taffy["overflow"] = pipe(value, isString, toShortHand, toOverflow, toPoint);
      });
  }

  return taffy;
}

function isString(value: unknown): string {
  return z.string().parse(value);
}

function toShortHand(value: string): [string, string] {
  return z
    .array(z.string())
    .min(1)
    .max(2)
    .transform(([first, second]) => [first, second ?? first])
    .pipe(z.tuple([z.string(), z.string()]))
    .parse(value.split(/\s+/));
}

function toOverflow(values: [string, string]): [t.Overflow, t.Overflow] {
  return values.map(value =>
    match<string, t.Overflow>(value)
      .with("visible", () => "Visible")
      .with("hidden", () => "Hidden")
      .with("clip", () => "Clip")
      .with("scroll", () => "Scroll")
      .otherwise(thrw("overflow", value))
  ) as [t.Overflow, t.Overflow];
}

function toPoint<T>([x, y]: [T, T]): t.Point<T> {
  return { x, y };
}

function thrw(key: string, value: unknown): () => never {
  return () => {
    throw new Error(`Unknown CSS property "${key}: ${value}"`);
  };
}
