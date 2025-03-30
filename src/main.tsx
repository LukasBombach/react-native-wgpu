import React from "npm:react";
import { create_instance2 } from "rn-wgpu:rect";
import { ReactWGPU } from "./renderer.ts";

create_instance2({
  display: "Flex",
  flex_direction: "Column",
  size: {
    width: {
      Length: 100,
    },
    height: {
      Length: 100,
    },
  },
});

ReactWGPU.render(
  <>
    <rectangle top={100} left={100} width={600} height={600} />
  </>
);
