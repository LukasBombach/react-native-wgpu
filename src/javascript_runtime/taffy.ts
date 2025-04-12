import { pipe } from "fp-ts/lib/function";
import * as R from "fp-ts/lib/Record";
import * as A from "fp-ts/lib/Array";
import { match, P } from "ts-pattern";
import * as toCase from "change-case";

import type { CSSProperties } from "react";
import type * as Taffy from "./taffy_types";

export function cssToTaffy(css: CSSProperties): Partial<Taffy.Style> {
  const taffy: Partial<Taffy.Style> = {};

  for (let k in css) {
    console.log(k, css[k]); // This works
  }

  return taffy;
}
