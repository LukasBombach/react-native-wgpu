import { pipe } from "fp-ts/lib/function";
import * as R from "fp-ts/lib/Record";
import * as A from "fp-ts/lib/Array";
import * as S from "fp-ts/string";
import * as O from "fp-ts/lib/Option";
import { match, P } from "ts-pattern";

import type * as Taffy from "./taffy_types";

export function cssToTaffy<T extends Record<string, unknown>>(css: T): Partial<Taffy.Style> {
  const taffy: Partial<Taffy.Style> = {};

  for (const [key, value] of Object.entries(css)) {
    match(key)
      .with("display", () => {
        taffy["display"] = match(value)
          .with("block", () => "Block")
          .with("flex", () => "Flex")
          .with("grid", () => "Grid")
          .with("none", () => "None")
          .otherwise(() => {
            console.warn(`Unknown CSS property "${key.toString()}: ${value}"`);
            return undefined;
          });
      })
      .with("boxSizing", () => {
        taffy["boxSizing"] = match(value)
          .with("border-box", () => "BorderBox")
          .with("content-box", () => "ContentBox")
          .otherwise(() => {
            console.warn(`Unknown CSS property "${key.toString()}: ${value}"`);
            return undefined;
          });
      });
  }

  return taffy;
}
