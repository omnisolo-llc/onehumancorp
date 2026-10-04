import { act, cleanup, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import RootLayout from '../layout';
import ShareCardPage from './page';
import { invalidateQueueOwner } from '../../lib/sync/queueIdentity';
import { classifyRequest } from '../../lib/auth/publicRoutes';

const navigation = vi.hoisted(() => ({ pathname: '/share-card' }));
vi.mock('next/navigation', async () => ({
  ...await vi.importActual<typeof import('next/navigation')>('next/navigation'),
  usePathname: () => navigation.pathname,
  useRouter: () => ({ push: vi.fn(), replace: vi.fn() }),
}));

const originalLocation = window.location;
let requests: string[];

beforeEach(() => {
  navigation.pathname = '/share-card';
  requests = [];
  invalidateQueueOwner();
  Object.defineProperty(window, 'location', { configurable: true, writable: true,
    value: { ...originalLocation, replace: vi.fn() } });
  vi.stubGlobal('fetch', vi.fn<typeof fetch>(async input => {
    const path = String(input);
    requests.push(path);
    if (path === '/api/v1/help' || path === '/api/v1/videos') return Response.json([]);
    if (path === '/api/v1/tooltips') return Response.json({});
    // No verified identity or queue execution is fabricated by this render test.
    if (path === '/api/v1/auth/session-identity') return Response.json({ error: 'unauthorized' }, { status: 401 });
    throw new Error(`Unexpected startup request: ${path}`);
  }));
});

afterEach(() => {
  cleanup();
  invalidateQueueOwner();
  vi.unstubAllGlobals();
  Object.defineProperty(window, 'location', { configurable: true, writable: true, value: originalLocation });
});

// Render the actual root provider/widget tree without nesting html/body in jsdom.
const application = (children: React.ReactNode) => RootLayout({ children }).props.children.props.children;

it('does not start application reads in the share-card document that is being replaced', async () => {
  const page = await ShareCardPage({ searchParams: Promise.resolve({ url: '/onboarding?ref=owner&mode=manual' }) });
  await act(async () => { render(application(page)); });
  expect(screen.getByRole('link', { name: '/onboarding?ref=owner&mode=manual' })).toHaveAttribute('href', '/onboarding?ref=owner&mode=manual');
  expect(window.location.replace).toHaveBeenCalledExactlyOnceWith('/onboarding?ref=owner&mode=manual');
  expect(requests).toEqual([]);
  expect(screen.queryByRole('navigation')).not.toBeInTheDocument();
  expect(screen.queryByTestId('offline-queue-readiness')).not.toBeInTheDocument();
});

it.each(['/onboarding', '/dashboard', '/share-cards'])('starts the actual application widgets on destination/app route %s', async pathname => {
  navigation.pathname = pathname;
  await act(async () => { render(application(<main>Destination content</main>)); });
  expect(screen.getByText('Destination content')).toBeInTheDocument();
  expect(requests.sort()).toEqual([
    '/api/v1/auth/session-identity', '/api/v1/auth/session-identity',
    '/api/v1/help', '/api/v1/tooltips', '/api/v1/videos',
  ]);
  expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state', 'held');
  await act(async () => { window.dispatchEvent(new CustomEvent('omnisolo_event_received', { detail: { message: 'Destination notification' } })); });
  expect(screen.getByTestId('notification-toast')).toHaveTextContent('Destination notification');
});

it.each(['page', 'rsc', 'prefetch'] as const)('keeps the share-card %s request protected independently of its presentation frame', invocation => {
  expect(classifyRequest({ method: 'GET', pathname: '/share-card', invocation }).access).toBe('protected');
});
