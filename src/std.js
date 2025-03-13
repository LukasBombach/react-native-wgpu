export const console = {
  log: (...args) => Deno.core.print(args.join(" ") + "\n"),
};

export function setTimeout(callback, delay) {
  Deno.core.queueUserTimer(Deno.core.getTimerDepth() + 1, false, delay, callback);
}

export function setInterval(callback, delay) {
  Deno.core.queueUserTimer(Deno.core.getTimerDepth() + 1, true, delay, callback);
}
