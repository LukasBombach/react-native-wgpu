const x = {
  display: "Grid",
  item_is_table: false,
  box_sizing: "BorderBox",
  overflow: { x: "Visible", y: "Visible" },
  scrollbar_width: 0,
  position: "Relative",
  inset: {
    left: "Auto",
    right: "Auto",
    top: "Auto",
    bottom: "Auto",
  },
  size: {
    width: { Length: 800 },
    height: { Length: 600 },
  },
  min_size: { width: "Auto", height: "Auto" },
  max_size: { width: "Auto", height: "Auto" },
  aspect_ratio: null,
  margin: {
    left: { Length: 0 },
    right: { Length: 0 },
    top: { Length: 0 },
    bottom: { Length: 0 },
  },
  padding: {
    left: { Length: 0 },
    right: { Length: 0 },
    top: { Length: 0 },
    bottom: { Length: 0 },
  },
  border: {
    left: { Length: 0 },
    right: { Length: 0 },
    top: { Length: 0 },
    bottom: { Length: 0 },
  },
  align_items: null,
  align_self: null,
  justify_items: null,
  justify_self: null,
  align_content: null,
  justify_content: null,
  gap: {
    width: { Length: 0 },
    height: { Length: 0 },
  },
  text_align: "Auto",
  flex_direction: "Row",
  flex_wrap: "NoWrap",
  flex_basis: "Auto",
  flex_grow: 0,
  flex_shrink: 1,
  grid_template_rows: [
    {
      Repeat: [
        { Count: 1 },
        [
          {
            min: "Auto",
            max: { Fraction: 1 },
          },
        ],
      ],
    },
  ],
  grid_template_columns: [
    {
      Single: {
        min: {
          Fixed: { Length: 250 },
        },
        max: {
          Fixed: { Length: 250 },
        },
      },
    },
    {
      Single: {
        min: "Auto",
        max: { Fraction: 1 },
      },
    },
    {
      Single: {
        min: {
          Fixed: { Length: 250 },
        },
        max: {
          Fixed: { Length: 250 },
        },
      },
    },
  ],
  grid_auto_rows: [
    {
      min: {
        Fixed: { Length: 100 },
      },
      max: {
        Fixed: { Length: 100 },
      },
    },
  ],
  grid_auto_columns: [
    {
      min: {
        Fixed: { Length: 100 },
      },
      max: {
        Fixed: { Length: 100 },
      },
    },
  ],
  grid_auto_flow: "Row",
  grid_row: {
    start: { Line: 1 },
    end: "Auto",
  },
  grid_column: {
    start: { Span: 3 },
    end: "Auto",
  },
};
