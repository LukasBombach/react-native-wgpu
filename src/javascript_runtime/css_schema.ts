import * as z from "zod";
import { snakeCase, pascalCase } from "change-case";

import type { Point, Display, BoxSizing, Overflow } from "./taffy_types";

export const display = z.literal(["block", "flex", "grid", "none"]).transform(v => pascalCase(v) as Display);

export const boxSizing = z.literal(["border-box", "content-box"]).transform(v => pascalCase(v) as BoxSizing);

export const overflow = z
  .string()
  .pipe(z.transform(v => v.split(/\W/)))
  .pipe(
    z
      .array(z.literal(["visible", "hidden", "clip", "scroll", "auto"]))
      .min(1)
      .max(2)
  )
  .pipe(z.transform(vs => vs.map(v => pascalCase(v))))
  .pipe(
    z.tuple([
      z.literal(["Visible", "Hidden", "Clip", "Scroll", "Auto"]),
      z.literal(["Visible", "Hidden", "Clip", "Scroll", "Auto"]),
    ])
  );
