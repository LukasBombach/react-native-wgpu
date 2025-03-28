import ReactReconciler from "npm:react-reconciler";
import { NoEventPriority } from "npm:react-reconciler/constants";
import { create_rect, append_rect_to_window } from "rn-wgpu:rect";

type RectId = number;
type RectProps = { top: number; left: number; width: number; height: number };

type Type = "rect";
type Props = RectProps;
type Container = null;
type Instance = { id: RectId };
type TextInstance = null;
type SuspenseInstance = never;
type HydratableInstance = never;
type PublicInstance = null;
type HostContext = null;
type UpdatePayload = RectProps;
type ChildSet = never;
type TimeoutHandle = number;
type NoTimeout = -1;

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
    return { id };
  },
  createTextInstance(_text, _rootContainerInstance, _hostContext, _internalInstanceHandle) {
    return null;
  },
  appendChildToContainer(_container, child) {
    if (!child) return;
    append_rect_to_window(child.id);
  },
  appendChild(_parent, child) {
    if (!child) return;
    append_rect_to_window(child.id);
  },
  appendInitialChild(_parent, child) {
    if (!child) return;
    append_rect_to_window(child.id);
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
  getRootHostContext: () => null,
  getChildHostContext: () => null,
  getPublicInstance: () => null,
  getCurrentEventPriority: () => NoEventPriority,
});
