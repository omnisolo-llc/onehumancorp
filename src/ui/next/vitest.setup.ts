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
vi.mock('next/link', async () => {
  const { createElement } = await vi.importActual<typeof import('react')>('react');
  return {
    default: ({ children, href, ...rest }: import('react').AnchorHTMLAttributes<HTMLAnchorElement>) =>
      createElement('a', { href, ...rest }, children),
  };
})

// Mock next/image
vi.mock('next/image', async () => {
  const { createElement } = await vi.importActual<typeof import('react')>('react');
  return { default: (props: import('react').ImgHTMLAttributes<HTMLImageElement>) =>
    createElement('img', props) };
})

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
      static json(data: unknown, init?: ResponseInit) {
        const headers = new Headers(init?.headers);
        headers.set('Content-Type', 'application/json');
        return new Response(JSON.stringify(data), { ...init, headers });
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

// Each component test owns its explicit API-boundary fixtures. An unspecified
// request is neither invented success nor permission to contact a live service.
// Direct assignment keeps vi.unstubAllGlobals() in individual tests from restoring
// native network access; their own stubs restore this blocked baseline instead.
global.fetch = vi.fn<typeof fetch>(async () => {
  throw new Error('Browser unit tests must explicitly mock their transport boundary');
});

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

if (typeof window !== 'undefined') {
  Object.defineProperty(window, 'localStorage', {
    value: localStorageMock,
    writable: true,
  });
}

// Set IS_REACT_ACT_ENVIRONMENT
Object.defineProperty(globalThis, 'IS_REACT_ACT_ENVIRONMENT', {
  value: true, writable: true, configurable: true,
});

// Mock Worker
class TestWorker extends EventTarget {
  onmessage: ((event: MessageEvent) => void) | null = null;
  onmessageerror: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  postMessage = vi.fn<(message: unknown, transfer?: Transferable[]) => void>();
  terminate = vi.fn<() => void>();
}
Object.defineProperty(globalThis, 'Worker', {
  value: TestWorker, writable: true, configurable: true,
});

// Mock navigator.locks
if (typeof navigator !== 'undefined') {
  Object.defineProperty(navigator, 'locks', {
    value: {
      request: vi.fn(),
      query: vi.fn()
    },
    writable: true
  });
}
if (typeof navigator !== 'undefined' && navigator.locks) {
  navigator.locks.request = vi.fn().mockImplementation(async (name, cb) => {
    if (typeof cb === 'function') {
      return cb();
    }
  });
}
