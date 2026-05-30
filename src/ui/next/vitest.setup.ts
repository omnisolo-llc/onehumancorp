import '@testing-library/jest-dom'
import { vi } from 'vitest'

global.vi = vi
window.HTMLElement.prototype.scrollIntoView = vi.fn();
