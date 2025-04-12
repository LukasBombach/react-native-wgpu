import * as z from "zod";

const display = z.literal(["block", "flex", "grid", "none"]);
const boxSizing = z.literal(["border-box", "content-box"]);
const overflow = z.literal(["visible", "hidden", "clip", "scroll", "auto"]);
