import '@testing-library/jest-dom';
import { vi, beforeAll, afterAll } from 'vitest';

const originalConsoleError = console.error;
beforeAll(() => {
  console.error = vi.fn();
});
afterAll(() => {
  console.error = originalConsoleError;
});
