import "react-reconciler";

declare module "react-reconciler" {
  interface HostConfig<
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
  > {
    setCurrentUpdatePriority?: (newPriority: number) => void;
    getCurrentUpdatePriority?: () => number;
    resolveUpdatePriority?: () => number;
    maySuspendCommit?: () => boolean;
  }
}
