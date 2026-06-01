import '@testing-library/jest-dom'
import { vi } from 'vitest'

global.vi = vi
if (typeof window !== 'undefined' && window.HTMLElement) {
  window.HTMLElement.prototype.scrollIntoView = vi.fn();
}
