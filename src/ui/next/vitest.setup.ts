import '@testing-library/jest-dom'
import { vi } from 'vitest'
import React from 'react'

global.vi = vi
window.HTMLElement.prototype.scrollIntoView = vi.fn();

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn() }),
  usePathname: () => '/',
  useParams: () => ({ article: 'getting-started' })
}));

vi.mock('next/link', () => {
  return {
    default: ({ children, href }: any) => React.createElement('a', { href }, children)
  }
});

vi.mock('next/server', () => ({
  NextResponse: {
    json: (body: any) => ({
      json: async () => body,
      status: 200,
      ok: true,
      headers: new Headers()
    }),
    next: () => ({}),
    redirect: () => ({})
  }
}));

vi.mock('node:child_process', () => ({
  spawn: vi.fn()
}));

vi.mock('node:crypto', () => ({
  randomUUID: () => '1234-5678-9012-3456'
}));

vi.mock('swagger-ui-react/swagger-ui.css', () => ({}));
