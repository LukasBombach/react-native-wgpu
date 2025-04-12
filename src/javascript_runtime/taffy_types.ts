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

type Display = "Block" | "Flex" | "Grid" | "None";

type BoxSizing = "BorderBox" | "ContentBox";

interface Point<T> {
  x: T;
  y: T;
}

type Overflow = "Visible" | "Clip" | "Hidden" | "Scroll";

type Position = "Relative" | "Absolute";

interface Rect<T> {
  left: T;
  right: T;
  top: T;
  bottom: T;
}

type LengthPercentageAuto = Length<number> | Percent<number> | Auto;

interface Length<T> {
  Length: T;
}

interface Percent<T> {
  Percent: T;
}

type Auto = "Auto";

interface Size<T> {
  width: T;
  height: T;
}

type Dimension = Length<number> | Percent<number> | Auto;

type Option<T> = T | undefined;

type LengthPercentage = Length<number> | Percent<number>;

type AlignItems = "Start" | "End" | "FlexStart" | "FlexEnd" | "Center" | "Baseline" | "Stretch";

type AlignSelf = AlignItems;

type AlignContent =
  | "Start"
  | "End"
  | "FlexStart"
  | "FlexEnd"
  | "Center"
  | "Stretch"
  | "SpaceBetween"
  | "SpaceEvenly"
  | "SpaceAround";

type JustifyContent = AlignContent;

type TextAlign = "Auto" | "LegacyLeft" | "LegacyRight" | "LegacyCenter";

type FlexDirection = "Row" | "Column" | "RowReverse" | "ColumnReverse";

type FlexWrap = "NoWrap" | "Wrap" | "WrapReverse";

type GridTrackVec<T> = Array<T>;

type TrackSizingFunction =
  | Single<NonRepeatedTrackSizingFunction>
  | Repeat<GridTrackRepetition, GridTrackVec<NonRepeatedTrackSizingFunction>>;

interface Single<T> {
  Single: T;
}

type Repeat<R, T> = [R, T];

type NonRepeatedTrackSizingFunction = MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>;

interface MinMax<Min, Max> {
  min: Min;
  max: Max;
}

type MinTrackSizingFunction = Fixed<LengthPercentage> | MinContent | MaxContent | Auto;

type MaxTrackSizingFunction =
  | Fixed<LengthPercentage>
  | MinContent
  | MaxContent
  | FitContent<LengthPercentage>
  | Auto
  | Fraction<number>;

interface Fixed<T> {
  Fixed: T;
}

interface FitContent<T> {
  FitContent: T;
}

interface Fraction<T> {
  Fraction: T;
}

type MinContent = "MinContent";
type MaxContent = "MaxContent";

type GridTrackRepetition = AutoFill | AutoFit | Count<number>;

type AutoFill = "AutoFill";

type AutoFit = "AutoFit";

interface Count<T> {
  Count: T;
}

type GridAutoFlow = "Row" | "Column" | "RowDense" | "ColumnDense";

interface Line<T> {
  start: T;
  end: T;
}

type GridPlacement = GenericGridPlacement<GridLine>;

type GenericGridPlacement<LineType> = Auto | Line<LineType> | Span<number>;

interface GridLine<T = number> {
  GridLine: T;
}

interface Span<T = number> {
  Span: T;
}
