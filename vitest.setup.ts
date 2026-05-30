import '@testing-library/jest-dom';
import { vi } from 'vitest';

global.fetch = vi.fn();
window.HTMLElement.prototype.scrollIntoView = vi.fn();
