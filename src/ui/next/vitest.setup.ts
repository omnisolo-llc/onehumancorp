import '@testing-library/jest-dom'
import { vi } from 'vitest'

global.vi = vi
window.HTMLElement.prototype.scrollIntoView = vi.fn();

global.fetch = vi.fn().mockImplementation(() => Promise.resolve({
  ok: true,
  json: () => Promise.resolve({})
})) as any;

window.HTMLElement.prototype.scrollIntoView = vi.fn();
