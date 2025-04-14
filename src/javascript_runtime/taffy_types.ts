export interface Style {
  display: Display;
  item_is_table: boolean;
  box_sizing: BoxSizing;
  overflow: Point<Overflow>;
  scrollbar_width: number;
  position: Position;
  inset: Rect<LengthPercentageAuto>;
  size: Size<Dimension>;
  min_size: Size<Dimension>;
  max_size: Size<Dimension>;
  aspect_ratio: Option<number>;
  margin: Rect<LengthPercentageAuto>;
  padding: Rect<LengthPercentage>;
  border: Rect<LengthPercentage>;
  align_items: Option<AlignItems>;
  align_self: Option<AlignSelf>;
  justify_items: Option<AlignItems>;
  justify_self: Option<AlignSelf>;
  align_content: Option<AlignContent>;
  justify_content: Option<JustifyContent>;
  gap: Size<LengthPercentage>;
  text_align: TextAlign;
  flex_direction: FlexDirection;
  flex_wrap: FlexWrap;
  flex_basis: Dimension;
  flex_grow: number;
  flex_shrink: number;
  grid_template_rows: GridTrackVec<TrackSizingFunction>;
  grid_template_columns: GridTrackVec<TrackSizingFunction>;
  grid_auto_rows: GridTrackVec<NonRepeatedTrackSizingFunction>;
  grid_auto_columns: GridTrackVec<NonRepeatedTrackSizingFunction>;
  grid_auto_flow: GridAutoFlow;
  grid_row: Line<GridPlacement>;
  grid_column: Line<GridPlacement>;
}

export type Display = "Block" | "Flex" | "Grid" | "None";

export type BoxSizing = "BorderBox" | "ContentBox";

export interface Point<T> {
  x: T;
  y: T;
}

export type Overflow = "Visible" | "Clip" | "Hidden" | "Scroll";

export type Position = "Relative" | "Absolute";

export interface Rect<T> {
  left: T;
  right: T;
  top: T;
  bottom: T;
}

export type LengthPercentageAuto = Length<number> | Percent<number> | Auto;

export interface Length<T> {
  Length: T;
}

export interface Percent<T> {
  Percent: T;
}

export type Auto = "Auto";

export interface Size<Width, Height = Width> {
  width: Width;
  height: Height;
}

export type Dimension = Length<number> | Percent<number> | Auto;

export type Option<T> = T | undefined;

export type LengthPercentage = Length<number> | Percent<number>;

export type AlignItems = "Start" | "End" | "FlexStart" | "FlexEnd" | "Center" | "Baseline" | "Stretch";

export type AlignSelf = AlignItems;

export type AlignContent =
  | "Start"
  | "End"
  | "FlexStart"
  | "FlexEnd"
  | "Center"
  | "Stretch"
  | "SpaceBetween"
  | "SpaceEvenly"
  | "SpaceAround";

export type JustifyContent = AlignContent;

export type TextAlign = "Auto" | "LegacyLeft" | "LegacyRight" | "LegacyCenter";

export type FlexDirection = "Row" | "Column" | "RowReverse" | "ColumnReverse";

export type FlexWrap = "NoWrap" | "Wrap" | "WrapReverse";

export type GridTrackVec<T> = Array<T>;

export type TrackSizingFunction =
  | Single<NonRepeatedTrackSizingFunction>
  | Repeat<GridTrackRepetition, GridTrackVec<NonRepeatedTrackSizingFunction>>;

export interface Single<T> {
  Single: T;
}

export type Repeat<R, T> = [R, T];

export type NonRepeatedTrackSizingFunction = MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>;

export interface MinMax<Min, Max> {
  min: Min;
  max: Max;
}

export type MinTrackSizingFunction = Fixed<LengthPercentage> | MinContent | MaxContent | Auto;

export type MaxTrackSizingFunction =
  | Fixed<LengthPercentage>
  | MinContent
  | MaxContent
  | FitContent<LengthPercentage>
  | Auto
  | Fraction<number>;

export interface Fixed<T> {
  Fixed: T;
}

export interface FitContent<T> {
  FitContent: T;
}

export interface Fraction<T> {
  Fraction: T;
}

export type MinContent = "MinContent";
export type MaxContent = "MaxContent";

export type GridTrackRepetition = AutoFill | AutoFit | Count<number>;

export type AutoFill = "AutoFill";

export type AutoFit = "AutoFit";

export interface Count<T> {
  Count: T;
}

export type GridAutoFlow = "Row" | "Column" | "RowDense" | "ColumnDense";

export interface Line<T> {
  start: T;
  end: T;
}

export type GridPlacement = GenericGridPlacement<GridLine>;

export type GenericGridPlacement<LineType> = Auto | Line<LineType> | Span<number>;

export interface GridLine<T = number> {
  GridLine: T;
}

export interface Span<T = number> {
  Span: T;
}
