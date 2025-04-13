import { pipe } from "fp-ts/lib/function";
import * as R from "fp-ts/lib/Record";
import * as A from "fp-ts/lib/Array";
import * as S from "fp-ts/string";
import * as O from "fp-ts/lib/Option";
import { match, P } from "ts-pattern";
import * as z from "zod";
import { snakeCase, pascalCase } from "change-case";
import { display, boxSizing, overflow } from "./css_schema";

import type { CSSProperties } from "react";
import type * as Taffy from "./taffy_types";

const overflow = z.literal(["visible", "hidden", "clip", "scroll", "auto"]);

export function cssToTaffy<T extends CSSProperties>(css: T): Partial<Taffy.Style> {
  const taffy: Partial<Taffy.Style> = {};
  const keys = Object.keys(css) as (keyof T)[];

  for (const k of keys) {
    match(k)
      .with("display", () => (taffy["display"] = display.parse(css[k])))
      .with("overflow", () => {
        pipe(isString(css[k]), O.map(S.split(/\s+/)));

        taffy["overflow"] = overflow.parse(css[k]);
      })
      .otherwise(() => console.warn(`Unknown CSS property "${k.toString()}: ${css[k]}"`));
  }

  return taffy;
}

export function isString(value: unknown): O.Option<string> {
  return match(z.string().safeParse(value))
    .with({ success: true, data: P.select() }, data => O.some(data))
    .otherwise(() => O.none);
}
