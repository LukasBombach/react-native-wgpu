import type { CSSProperties } from "react";

export function cssToTaffy(css: CSSProperties) {
  return Object.fromEntries(
    Object.entries(css).map(([prop, value]) => {
      if (typeof value === "string" && value.endsWith("%")) {
        const num = parseFloat(value.slice(0, -1)) / 100;
        return [prop, { Percent: num }];
      }

      return [prop, value];
    })
  );
}
