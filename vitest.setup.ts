import '@testing-library/jest-dom';

class ResizeObserver {
  disconnect() {}
  observe() {}
  unobserve() {}
}

if (typeof window !== 'undefined') {
  Object.defineProperty(window, 'ResizeObserver', {
    writable: true,
    configurable: true,
    value: ResizeObserver,
  })
}
