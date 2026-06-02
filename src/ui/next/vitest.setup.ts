import '@testing-library/jest-dom'
import { vi } from 'vitest'

// Add fetch mock if needed
if (typeof global.fetch === 'undefined') {
  global.fetch = vi.fn().mockImplementation((url: RequestInfo | URL, init?: RequestInit) => {
    // Mock for dashboard metrics and other relative URLs in tests
    return Promise.resolve(new Response(JSON.stringify({}), {
        status: 200,
        headers: { 'Content-Type': 'application/json' }
    }));
  }) as any
} else {
  // Override existing global.fetch for Vitest env if it throws Invalid URL on relative paths
  const originalFetch = global.fetch;
  global.fetch = vi.fn().mockImplementation((url: RequestInfo | URL, init?: RequestInit) => {
    if (typeof url === 'string' && url.startsWith('/')) {
        return Promise.resolve(new Response(JSON.stringify({
            ok: true,
            // Provide sensible defaults for the failed API calls in tests
            metrics: {}, approvals: [], workflows: [], milestones: []
        }), {
            status: 200,
            headers: { 'Content-Type': 'application/json' }
        }));
    }
    return originalFetch(url, init);
  }) as any
}

// Add standard window mocks
if (typeof window !== 'undefined') {
  Object.defineProperty(window, 'matchMedia', {
    writable: true,
    value: vi.fn().mockImplementation(query => ({
      matches: false,
      media: query,
      onchange: null,
      addListener: vi.fn(), // deprecated
      removeListener: vi.fn(), // deprecated
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      dispatchEvent: vi.fn(),
    })),
  })

  // IntersectionObserver mock
  class IntersectionObserver {
    root = null;
    rootMargin = '';
    thresholds = [];

    disconnect() {}
    observe() {}
    takeRecords() { return [] }
    unobserve() {}
  }
  Object.defineProperty(window, 'IntersectionObserver', {
    writable: true,
    configurable: true,
    value: IntersectionObserver,
  })
}
