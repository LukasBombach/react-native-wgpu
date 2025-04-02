import ReactReconciler from "npm:react-reconciler";
import { create_instance, append_child_to_container, append_child } from "rn-wgpu:rect";
import { taffyFromCss } from "./style.ts";
import type { CSSProperties, ReactNode } from "react";

type RectId = number;
type RectProps = { style: CSSProperties };

type Type = Pick<Container | Instance | TextInstance | HostContext, "type">;
type Props = RectProps;
type Container = { type: "container" };
type Instance = { type: "rectangle"; id: RectId };
type TextInstance = { type: "text" };
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
const ConcurrentRoot = 1;

let currentUpdatePriority = NoEventPriority;

const reconciler = ReactReconciler<
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
    const taffyStyle = taffyFromCss(props.style);
    const id = create_instance(taffyStyle);
    return { type: "rectangle", id };
  },

  appendChildToContainer(_container, child) {
    if (child.type === "rectangle") {
      append_child_to_container(child.id);
    } else {
      console.warn("appendChildToContainer: Ignoring child", child);
    }
  },

  appendInitialChild(parent, child) {
    if (child.type === "rectangle") {
      append_child(parent.id, child.id);
    } else {
      console.warn("appendInitialChild: Ignoring child", child);
    }
  },

  appendChild(parent, child) {
    if (child.type === "rectangle") {
      append_child(parent.id, child.id);
    } else {
      console.warn("appendChild: Ignoring child", child);
    }
  },

  createTextInstance(_text, _rootContainerInstance, _hostContext, _internalInstanceHandle) {
    return { type: "text" };
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

export const ReactWGPU = {
  render(rootInstance: ReactNode) {
    const container = reconciler.createContainer(
      { type: "container" },
      ConcurrentRoot,
      null,
      true,
      null,
      "",
      error => console.error("Recoverable error:", error),
      null
    );
    reconciler.updateContainer(rootInstance, container, null, null);
  },
};
