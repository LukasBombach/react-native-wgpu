import { pipe } from "fp-ts/lib/function";
import * as R from "fp-ts/lib/Record";
import * as A from "fp-ts/lib/Array";
import { match, P } from "ts-pattern";
import * as z from "zod";
import { snakeCase, pascalCase } from "change-case";
import { display, boxSizing, overflow } from "./css_schema";

import type { CSSProperties } from "react";
import type * as Taffy from "./taffy_types";

export function cssToTaffy<T extends CSSProperties>(css: T): Partial<Taffy.Style> {
  const taffy: Partial<Taffy.Style> = {};
  const keys = Object.keys(css) as (keyof T)[];

  for (const k of keys) {
    match(k)
      .with("display", () => (taffy["display"] = display.parse(css[k])))
      // .with("boxSizing", () => (taffy["box_sizing"] = boxSizing.parse(css[k])))
      .with("overflow", () => (taffy["overflow"] = overflow.parse(css[k])))
      .otherwise(() => console.warn(`Unknown CSS property "${k.toString()}: ${css[k]}"`));
  }

  return taffy;
}
