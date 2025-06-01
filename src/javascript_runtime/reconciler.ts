import ReactReconciler from "react-reconciler";
import { taffyFromCss } from "./taffy.ts";

// @ts-expect-error not typed yet
export const create_instance = Deno.core.ops.op_create_instance;
// @ts-expect-error not typed yet
export const create_text_node = Deno.core.ops.op_create_text_node;
// @ts-expect-error not typed yet
export const append_child_to_container = Deno.core.ops.op_append_child_to_container;
// @ts-expect-error not typed yet
export const append_child = Deno.core.ops.op_append_child;
// @ts-expect-error not typed yet
export const get_style_defaults = Deno.core.ops.op_get_style_defaults;
// @ts-expect-error not typed yet
export const debug = Deno.core.ops.op_debug;

import type { CSSProperties, ReactNode } from "react";

type RectId = number;
type RectProps = { style: CSSProperties };

type Type = Pick<Container | Instance | TextInstance | HostContext, "type">;
type Props = RectProps;
type Container = { type: "container" };
type Instance = { type: "div"; id: RectId };
type TextInstance = { type: "text"; id: RectId };
type SuspenseInstance = never;
type HydratableInstance = never;
type PublicInstance = { type: string };
type HostContext = { type: "context" };
type UpdatePayload = RectProps;
type ChildSet = never;
type TimeoutHandle = number;
type NoTimeout = -1;

// from react-reconciler/constants, which cannot be imported with rustyscript
const NoEventPriority = 0;
const DefaultEventPriority = 0b0000000000000000000000000010000;

let currentUpdatePriority = NoEventPriority;

export const reconciler = ReactReconciler<
  Type,
  Props,
  Container,
  Instance,
  TextInstance,
  SuspenseInstance,
  HydratableInstance,
  PublicInstance,
  HostContext,
  UpdatePayload,
  ChildSet,
  TimeoutHandle,
  NoTimeout
>({
  isPrimaryRenderer: true,
  supportsMutation: true,
  supportsHydration: false,
  supportsPersistence: false,
  noTimeout: -1,

  createInstance(_type, props, _rootContainerInstance, _hostContext, _internalInstanceHandle) {
    const taffyStyle = taffyFromCss(props.style as Record<string, unknown>);
    const { backgroundColor = "transparent", borderRadius = 0 } = props.style;
    const id = create_instance(taffyStyle, backgroundColor, borderRadius);
    return { type: "div", id };
  },

  appendChildToContainer(_container, child) {
    if (child.type === "div" || child.type === "text") {
      append_child_to_container(child.id);
    } else {
      console.warn("appendChildToContainer: Ignoring child", child);
    }
  },

  appendInitialChild(parent, child) {
    if (child.type === "div" || child.type === "text") {
      append_child(parent.id, child.id);
    } else {
      console.warn("appendInitialChild: Ignoring child", child);
    }
  },

  appendChild(parent, child) {
    if (child.type === "div" || child.type === "text") {
      append_child(parent.id, child.id);
    } else {
      console.warn("appendChild: Ignoring child", child);
    }
  },

  createTextInstance(text, _rootContainerInstance, _hostContext, _internalInstanceHandle) {
    // Default text properties - could be configurable via context or props in the future
    const textColor = "black";
    const fontSize = 32.0; // Increased from 16.0 to make text more visible

    console.debug("createTextInstance", text, {
      textColor,
      fontSize,
    });

    const id = create_text_node(text, textColor, fontSize);
    return { type: "text", id };
  },

  clearContainer: () => false,
  prepareForCommit: () => null,
  preparePortalMount: () => {},
  resetAfterCommit: () => {},
  shouldSetTextContent: () => false,
  hideInstance() {},
  unhideInstance() {},
  hideTextInstance: () => {},
  unhideTextInstance: () => {},
  beforeActiveInstanceBlur: () => {},
  afterActiveInstanceBlur: () => {},
  detachDeletedInstance: () => {},
  // @ts-expect-error badly typed by react-reconciler
  scheduleTimeout: setTimeout,
  cancelTimeout: clearTimeout,
  getInstanceFromNode: () => null,
  prepareScopeUpdate: () => {},
  getInstanceFromScope: () => null,
  finalizeInitialChildren: () => false,
  maySuspendCommit: () => false,
  prepareUpdate: (_i, _t, _o, newProps) => newProps,
  getRootHostContext: () => ({ type: "context" }),
  getChildHostContext: () => ({ type: "context" }),
  getPublicInstance: ({ type }) => ({ type }),
  getCurrentEventPriority: () => NoEventPriority,
  setCurrentUpdatePriority: newPriority => {
    currentUpdatePriority = newPriority;
  },
  getCurrentUpdatePriority: () => {
    return currentUpdatePriority;
  },
  resolveUpdatePriority: () => {
    return currentUpdatePriority || DefaultEventPriority;
  },
});
