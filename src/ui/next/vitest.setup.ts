import '@testing-library/jest-dom/vitest'
import { vi } from 'vitest'

// Mock next/navigation
vi.mock('next/navigation', () => {
  return {
    useRouter: () => ({
      push: vi.fn(),
      replace: vi.fn(),
      prefetch: vi.fn(),
      back: vi.fn(),
      forward: vi.fn(),
      refresh: vi.fn(),
      pathname: '/',
      query: {},
    }),
    usePathname: () => '/',
    useSearchParams: () => new URLSearchParams(),
    useParams: () => ({ articleId: 'getting-started', article: 'getting-started' }),
    redirect: vi.fn(),
    notFound: vi.fn(),
  }
})

// Mock next/link
vi.mock('next/link', () => {
  return {
    default: ({ children, href, ...rest }: any) => {
      // @ts-ignore
      const React = require('react')
      return React.createElement('a', { href, ...rest }, children)
    }
  }
})

// Mock next/image
vi.mock('next/image', () => ({
  default: (props: any) => {
    // @ts-ignore
    const React = require('react')
    return React.createElement('img', props)
  }
}))

// Mock dompurify
vi.mock('dompurify', () => ({
  default: {
    sanitize: (html: string) => html
  }
}))

// Mock next/server
vi.mock('next/server', () => {
  return {
    NextResponse: class extends Response {
      static json(data: any, init?: any) {
        return new Response(JSON.stringify(data), {
          ...init,
          headers: {
            ...init?.headers,
            'Content-Type': 'application/json',
          },
        })
      }
      static redirect(url: string, status?: number) {
        return new Response(null, {
          status: status || 307,
          headers: { Location: url },
        })
      }
      static next() {
        return new Response(null, { status: 200 })
      }
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
}

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

if (typeof window !== 'undefined') {
  Object.defineProperty(window, 'IntersectionObserver', {
    writable: true,
    configurable: true,
    value: IntersectionObserver,
  })
}

// Add fetch mock if needed
const originalFetch2 = global.fetch;
global.fetch = vi.fn().mockImplementation(async (url: string | URL | Request, init?: RequestInit): Promise<Response> => {
    let urlString = '';
    if (typeof url === 'string') {
        urlString = url;
    } else if (url instanceof URL) {
        urlString = url.toString();
    } else if (url instanceof Request) {
        urlString = url.url;
    }

    if (urlString.startsWith('/')) {
        return Promise.resolve(new Response(JSON.stringify({
            ok: true,
            entries: [], metrics: {}, approvals: [], workflows: [], milestones: []
        }), {
            status: 200,
            headers: { 'Content-Type': 'application/json' }
        }));
    }
    return originalFetch2(url, init);
}) as any;

// Mock localStorage
const localStorageMock = (() => {
  let store: Record<string, string> = {};
  return {
    getItem: vi.fn((key: string) => store[key] || null),
    setItem: vi.fn((key: string, value: string) => {
      store[key] = value.toString();
    }),
    removeItem: vi.fn((key: string) => {
      delete store[key];
    }),
    clear: vi.fn(() => {
      store = {};
    }),
  };
})();

Object.defineProperty(window, 'localStorage', {
  value: localStorageMock,
  writable: true,
});

// Silence React act() warnings
const originalError = console.error;
console.error = (...args: any[]) => {
  if (typeof args[0] === 'string' && (args[0].includes('not configured to support act') || args[0].includes('was not wrapped in act') || args[0].includes('Sync WebSocket error'))) {
    return;
  }
  originalError(...args);
};

// Set IS_REACT_ACT_ENVIRONMENT
globalThis.IS_REACT_ACT_ENVIRONMENT = true;

// Mock Worker
class WorkerMock {
  constructor(stringUrl: string) {}
  onmessage() {}
  postMessage() {}
  terminate() {}
  addEventListener() {}
  removeEventListener() {}
  dispatchEvent() { return false; }
  onerror() {}
  onmessageerror() {}
}
globalThis.Worker = WorkerMock as any;

// Mock navigator locks
Object.defineProperty(navigator, 'locks', {
  value: {
    request: vi.fn(),
    query: vi.fn()
  }
});
