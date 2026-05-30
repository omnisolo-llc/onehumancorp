import '@testing-library/jest-dom/vitest'
import { vi } from 'vitest'

global.vi = vi
window.HTMLElement.prototype.scrollIntoView = vi.fn();
