import '@testing-library/jest-dom/vitest';
import { vi, beforeAll, afterAll } from 'vitest';

if (typeof window !== "undefined") { window.HTMLElement.prototype.scrollIntoView = function () {}; }

const originalConsoleError = console.error;
beforeAll(() => {
  console.error = vi.fn();
});
afterAll(() => {
  console.error = originalConsoleError;
});
