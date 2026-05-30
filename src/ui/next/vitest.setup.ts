import '@testing-library/jest-dom'
import { vi } from 'vitest'

global.vi = vi
global.fetch = vi.fn().mockImplementation(() => Promise.resolve({
  ok: true,
  json: () => Promise.resolve({})
})) as any;

// Mock scrollIntoView for jsdom
window.HTMLElement.prototype.scrollIntoView = vi.fn();
