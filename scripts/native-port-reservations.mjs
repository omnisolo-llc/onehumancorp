import { createServer as createNetServer } from 'node:net';

// Keep every selected port bound until its owning service is ready to start.
// Closing each probe before selecting the next role lets the kernel return the
// same port twice. Handoff still requires the child to bind its released port.
export async function reserveServicePorts(names, { createServer = createNetServer, signal } = {}) {
  if (!Array.isArray(names) || !names.length || names.some(name => typeof name !== 'string' || !name)
    || new Set(names).size !== names.length) throw new Error('Service port roles must be unique nonempty names');
  const reservations = new Map();
  const ports = Object.create(null);
  const closeOne = reservation => {
    reservation.closing ??= new Promise((resolve, reject) => {
      reservation.socket.close(error => {
        if (error && error.code !== 'ERR_SERVER_NOT_RUNNING') reject(error);
        else resolve();
      });
    });
    return reservation.closing;
  };
  const release = async (...roles) => {
    for (const role of roles) if (!reservations.has(role)) throw new Error(`Unknown service port role: ${role}`);
    const settled = await Promise.allSettled(roles.map(role => closeOne(reservations.get(role))));
    const errors = settled.filter(result => result.status === 'rejected').map(result => result.reason);
    if (errors.length) throw new AggregateError(errors, 'Unable to release service port reservations');
  };
  const close = () => {
    signal?.removeEventListener('abort', abortReservations);
    return release(...reservations.keys());
  };
  const abortReservations = () => { void close().catch(() => {}); };
  try {
    for (const name of names) {
      signal?.throwIfAborted();
      const socket = createServer(connection => connection.destroy());
      reservations.set(name, { socket });
      await new Promise((resolve, reject) => {
        const finish = error => {
          socket.removeListener('error', failed);
          signal?.removeEventListener('abort', aborted);
          if (error) reject(error); else resolve();
        };
        const failed = error => finish(error);
        const aborted = () => finish(signal.reason ?? new Error('Service port allocation cancelled'));
        socket.once('error', failed);
        signal?.addEventListener('abort', aborted, { once: true });
        try { socket.listen({ port: 0, host: '127.0.0.1', exclusive: true }, () => finish()); }
        catch (error) { finish(error); }
      });
      signal?.throwIfAborted();
      const address = socket.address();
      if (!address || typeof address === 'string' || !Number.isInteger(address.port) || address.port <= 0) {
        throw new Error(`Service port reservation has no bound address: ${name}`);
      }
      ports[name] = address.port;
    }
    signal?.addEventListener('abort', abortReservations, { once: true });
    signal?.throwIfAborted();
    return { ports: Object.freeze(ports), release, close };
  } catch (error) {
    let cleanupError;
    try { await close(); } catch (failure) { cleanupError = failure; }
    if (cleanupError) throw new AggregateError([error, cleanupError], 'Service port allocation and cleanup failed', { cause: error });
    throw error;
  }
}
