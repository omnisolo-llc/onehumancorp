// Share one manager across isolated modules/documents to model origin-level exclusion.
export function createBuilderLocks() {
  const active = new Set<string>();
  const queues = new Map<string, Array<() => void>>();
  return { request<T>(name: string, options: LockOptions, callback: (lock: Lock | null) => T | PromiseLike<T>): Promise<T> {
    if (options.ifAvailable && active.has(name)) return Promise.resolve().then(() => callback(null));
    return new Promise<T>((resolve, reject) => {
      const run = () => {
        active.add(name);
        void Promise.resolve().then(() => callback({ name, mode: 'exclusive' } as Lock)).then(resolve, reject).finally(() => {
          active.delete(name);
          const next = queues.get(name)?.shift();
          if (next) next();
        });
      };
      if (active.has(name)) { const queue = queues.get(name) ?? []; queue.push(run); queues.set(name, queue); }
      else run();
    });
  } };
}
export function installBuilderLocks(locks = createBuilderLocks()) { Object.defineProperty(navigator, 'locks', { value: locks }); }
