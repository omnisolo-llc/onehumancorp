import '@testing-library/jest-dom'
import { vi } from 'vitest'

global.vi = vi
window.HTMLElement.prototype.scrollIntoView = vi.fn();


// START PATCHED FETCH MOCK
global.fetch = vi.fn().mockImplementation((url) => {
  if (typeof url === 'string') {
    if (url.includes('/api/health')) {
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ status: 'ok' })
      });
    }
    if (url.includes('/api/auth/session')) {
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ user: { name: 'Test User' } })
      });
    }
  }
  return Promise.resolve({
    ok: true,
    json: () => Promise.resolve({})
  });
}) as any;
// END PATCHED FETCH MOCK

window.HTMLElement.prototype.scrollIntoView = vi.fn();
