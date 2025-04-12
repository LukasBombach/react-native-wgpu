import * as z from "zod";
import { snakeCase, pascalCase } from "change-case";

import type { Point, Display, BoxSizing, Overflow } from "./taffy_types";

export const display = z.literal(["block", "flex", "grid", "none"]).transform(v => pascalCase(v) as Display);

export const boxSizing = z.literal(["border-box", "content-box"]).transform(v => pascalCase(v) as BoxSizing);

export const overflow = z
  .literal(["visible", "hidden", "clip", "scroll", "auto"])
  .transform(v => pascalCase(v) as Point<Overflow>);
