import Reconciler from "npm:react-reconciler";
import { create_rect, append_rect_to_window } from "rn-wgpu:rect";

import type { HostConfig } from "npm:react-reconciler";

const ConcurrentRoot = 1;
const ContinuousEventPriority = 8;
const DefaultEventPriority = 32;
const DiscreteEventPriority = 2;
const IdleEventPriority = 268435456;
const LegacyRoot = 0;
const NoEventPriority = 0;

let currentUpdatePriority = NoEventPriority;

type RectId = number;
type RectProps = { top: number; left: number; width: number; height: number };

type Type = "rect";
type Props = RectProps;
type Container = null;
type Instance = { type: "rect"; id: RectId };
type TextInstance = { type: "text" };
type SuspenseInstance = any;
type HydratableInstance = any;
type PublicInstance = any;
type HostContext = any;
type UpdatePayload = any;
type ChildSet = any;
type TimeoutHandle = any;
type NoTimeout = any;

const hostConfig: HostConfig<
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
> = {
  supportsMutation: true,
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
      console.warn("appendChildToContainer: child is not a rect");
    }
  },
  appendChild(_parent, child) {
    if (child.type === "rect") {
      append_rect_to_window(child.id);
    } else {
      console.warn("appendChild: child is not a rect");
    }
  },
  appendInitialChild(_parent, child) {
    if (child.type === "rect") {
      append_rect_to_window(child.id);
    } else {
      console.warn("appendInitialChild: child is not a rect");
    }
  },

  /* prepareUpdate(instance, type, oldProps, newProps, rootContainerInstance, currentHostContext) {
    // Logic for preparing an update
  },
  commitUpdate(instance, updatePayload, type, oldProps, newProps, finishedWork) {
    // Logic for committing an update
  },
  finalizeInitialChildren() {
    // Logic for finalizing initial children
  },
  getChildHostContext() {
    // Logic for getting child host context
  },
  getPublicInstance() {
    // Logic for getting public instance
  },
  getRootHostContext() {
    // Logic for getting root host context
  },
  prepareForCommit() {
    // Logic before committing changes
  },
  resetAfterCommit() {
    // Logic after committing changes
  },
  shouldSetTextContent() {
    return false;
  }, */

  setCurrentUpdatePriority: newPriority => {
    currentUpdatePriority = newPriority;
  },
  getCurrentUpdatePriority: () => {
    return currentUpdatePriority;
  },
  resolveUpdatePriority: () => {
    return currentUpdatePriority || DefaultEventPriority;
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
};
