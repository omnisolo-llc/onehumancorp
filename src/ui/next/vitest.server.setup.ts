import { beforeEach, vi } from 'vitest';

// Server tests declare their own transport responses. Never turn an unspecified
// request into fake success or send it to a live account during unit validation.
function forbidUnspecifiedTransport() {
  vi.stubGlobal('fetch', vi.fn<typeof fetch>(async () => {
    throw new Error('Server unit tests must explicitly mock their transport boundary');
  }));
}
forbidUnspecifiedTransport();
beforeEach(forbidUnspecifiedTransport);
