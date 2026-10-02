// Shared instances model separate documents contending for an origin Web Lock.
export function createOriginLockManager() {
  const tails = new Map<string, Promise<unknown>>();
  return { request<T>(name: string, _options: unknown, callback: () => Promise<T>): Promise<T> {
    const next = (tails.get(name) ?? Promise.resolve()).then(callback);
    tails.set(name, next.catch(() => {}));
    return next;
  } };
}
export function installOnboardingLocks(locks = createOriginLockManager()) {
  Object.defineProperty(navigator, 'locks', { value: locks });
}
