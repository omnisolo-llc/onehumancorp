import '@testing-library/jest-dom'
import { vi } from 'vitest'

// Mock next/navigation
vi.mock('next/navigation', () => ({
  useRouter() {
    return {
      push: vi.fn(),
      replace: vi.fn(),
      prefetch: vi.fn(),
      back: vi.fn(),
      forward: vi.fn(),
      refresh: vi.fn(),
      pathname: '/',
      query: {},
    }
  },
  usePathname() {
    return '/'
  },
  useSearchParams() {
    return new URLSearchParams()
  },
  redirect: vi.fn(),
  notFound: vi.fn(),
}))

// Mock next/server
vi.mock('next/server', () => {
  return {
    NextResponse: {
      json: (data: any, init?: any) => {
        return new Response(JSON.stringify(data), {
          ...init,
          headers: {
            ...init?.headers,
            'Content-Type': 'application/json',
          },
        })
      },
      redirect: (url: string, status?: number) => {
        return new Response(null, {
          status: status || 307,
          headers: { Location: url },
        })
      },
      next: () => new Response(null, { status: 200 }),
    },
    NextRequest: Request,
  }
})

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
// Add fetch mock if needed
const originalFetch = global.fetch;
global.fetch = async (url: string | URL | Request, init?: RequestInit) => {
    let urlString = '';
    if (typeof url === 'string') {
        urlString = url;
    } else if (url instanceof URL) {
        urlString = url.toString();
    } else if (url instanceof Request) {
        urlString = url.url;
    }

    if (urlString.startsWith('/')) {
        url = 'http://localhost:3000' + urlString;
    }
    return originalFetch(url, init);
};
