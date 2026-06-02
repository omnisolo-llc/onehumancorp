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
const originalFetch = global.fetch;
global.fetch = vi.fn().mockImplementation((url, options) => {
  if (typeof url === 'string') {
    if (url.includes('/api/v1/dashboard/metrics')) {
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({
          total_sales: 1200,
          active_customers: 450,
          pending_orders: 12
        }),
        text: () => Promise.resolve('')
      });
    }
    return Promise.resolve({
      ok: true,
      json: () => Promise.resolve([]),
      text: () => Promise.resolve('')
    });
  }
  if (url instanceof URL || url instanceof Request) {
     if (url.toString().includes('/api/v1/dashboard/metrics')) {
       return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({
          total_sales: 1200,
          active_customers: 450,
          pending_orders: 12
        }),
        text: () => Promise.resolve('')
       });
     }
     return Promise.resolve({
      ok: true,
      json: () => Promise.resolve([]),
      text: () => Promise.resolve('')
    });
  }
  return Promise.resolve({
      ok: true,
      json: () => Promise.resolve([]),
      text: () => Promise.resolve('')
  });
});


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
