import ReactReconciler from "npm:react-reconciler";
// import { NoEventPriority, DefaultEventPriority } from "npm:react-reconciler/constants";
import { create_rect, append_rect_to_window } from "rn-wgpu:rect";

const NoEventPriority = 0;
const DefaultEventPriority = 0b0000000000000000000000000010000;

type RectId = number;
type RectProps = { top: number; left: number; width: number; height: number };

type Type = Pick<Container | Instance | TextInstance | HostContext, "type">;
type Props = RectProps;
type Container = { type: "container" };
type Instance = { type: "rect"; id: RectId };
type TextInstance = { type: "text" };
type SuspenseInstance = never;
type HydratableInstance = never;
type PublicInstance = { type: string };
type HostContext = { type: "context" };
type UpdatePayload = RectProps;
type ChildSet = never;
type TimeoutHandle = number;
type NoTimeout = -1;

let currentUpdatePriority: number = NoEventPriority;

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
    const { top, left, width, height } = props;
    const id = create_rect(top, left, width, height);
    return { type: "rect", id };
  },
  createTextInstance(_text, _rootContainerInstance, _hostContext, _internalInstanceHandle) {
    return { type: "text" };
  },
  appendChildToContainer(_container, child) {
    if (child.type === "rect") {
      append_rect_to_window(child.id);
    } else {
      console.warn("appendChildToContainer: Ignoring child", child);
    }
  },
  appendChild(_parent, child) {
    if (child.type === "rect") {
      append_rect_to_window(child.id);
    } else {
      console.warn("appendChild: Ignoring child", child);
    }
  },
  appendInitialChild(_parent, child) {
    if (child.type === "rect") {
      append_rect_to_window(child.id);
    } else {
      console.warn("appendInitialChild: Ignoring child", child);
    }
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
  prepareUpdate: (_i, _t, _o, newProps) => newProps,
  getRootHostContext: () => ({ type: "context" }),
  getChildHostContext: () => ({ type: "context" }),
  getPublicInstance: ({ type }) => ({ type }),
  getCurrentEventPriority: () => NoEventPriority,

  setCurrentUpdatePriority: (newPriority: number) => {
    currentUpdatePriority = newPriority;
  },
  getCurrentUpdatePriority: () => {
    return currentUpdatePriority;
  },
  resolveUpdatePriority: () => {
    return currentUpdatePriority || DefaultEventPriority;
  },

  maySuspendCommit: () => false,
});
