import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, waitFor } from '@testing-library/react';
import Dashboard from './page';
import { FloatingActionButton } from './components/FAB';
import { expect, test, vi, beforeEach } from 'vitest';

vi.mock('./components/FAB', () => ({
  FloatingActionButton: () => <div data-testid="mock-fab">Mock FAB</div>
}));

vi.mock('next/navigation', () => ({
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
}));

beforeEach(() => {
  global.EventSource = class MockEventSource {
    onmessage: any = null;
    onerror: any = null;
    close = vi.fn();
  } as any;

  global.fetch = vi.fn(() => Promise.resolve({
    ok: true,
    json: () => Promise.resolve({})
  })) as any;
});

test('renders dashboard with actionable feed', async () => {
  const { act } = await import('@testing-library/react');
  await act(async () => {
    render(<TooltipProvider><Dashboard /></TooltipProvider>);
  });

  await waitFor(() => {
    expect(screen.getAllByText("Business Analytics").length).toBeGreaterThan(0);
  });

  expect(screen.getByText("Operations Map")).toBeDefined();
  expect(screen.getByText(/Action Required/)).toBeDefined();
  expect(screen.getByText("Recent Orders")).toBeDefined();
  expect(screen.queryByText(/\/api\/ui\/dashboard\/unified-feed/)).toBeNull();
  expect(screen.getByText("Inbox Activity")).toBeDefined();
  expect(screen.getByRole("link", { name: /Campaign Orchestration/i })).toHaveAttribute("href", "/feed");
  expect(screen.getByText("Pro Plan ROI Calculator")).toBeDefined();
  expect(screen.getByText("Assistant Tasks")).toBeDefined();
});
