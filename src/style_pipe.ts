import { pipe } from "fp-ts/function";
import * as R from "fp-ts/Record";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";
import { pascalCase } from "change-case";

interface Point<T> {
  x: T;
  y: T;
}

const toPascalCase = (str: string) => pascalCase(str);

const toTaffy = (css: Record<string, string>) =>
  pipe(
    css,
    R.toEntries,
    A.map(([key, value]): [string, string | Point<string>] => {
      if (key === "overflow") {
        return [
          toPascalCase(key),
          {
            x: toPascalCase(value),
            y: toPascalCase(value),
          },
        ];
      }

      return [toPascalCase(key), toPascalCase(value)];
    }),
    R.fromEntries
  );

if (import.meta.vitest) {
  const { describe, test, expect } = import.meta.vitest;

  test.each`
    value                   | expected
    ${{ display: "block" }} | ${{ Display: "Block" }}
    ${{ display: "flex" }}  | ${{ Display: "Flex" }}
    ${{ display: "grid" }}  | ${{ Display: "Grid" }}
    ${{ display: "none" }}  | ${{ Display: "None" }}
  `("$value", ({ value, expected }) => {
    expect(toTaffy(value)).toEqual(expected);
  });

  test.each`
    value                           | expected
    ${{ boxSizing: "border-box" }}  | ${{ BoxSizing: "BorderBox" }}
    ${{ boxSizing: "content-box" }} | ${{ BoxSizing: "ContentBox" }}
  `("$value", ({ value, expected }) => {
    expect(toTaffy(value)).toEqual(expected);
  });

  test.each`
    value                      | expected
    ${{ overflow: "visible" }} | ${{ Overflow: { x: "Visible", y: "Visible" } }}
    ${{ overflow: "clip" }}    | ${{ Overflow: { x: "Clip", y: "Clip" } }}
    ${{ overflow: "hidden" }}  | ${{ Overflow: { x: "Hidden", y: "Hidden" } }}
    ${{ overflow: "scroll" }}  | ${{ Overflow: { x: "Scroll", y: "Scroll" } }}
  `("$value", ({ value, expected }) => {
    expect(toTaffy(value)).toEqual(expected);
  });
}
