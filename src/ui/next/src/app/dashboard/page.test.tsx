import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, waitFor } from '@testing-library/react';
import Dashboard from './page';
import { FloatingActionButton } from './components/FAB';
vi.mock('./components/FAB', () => ({
  FloatingActionButton: () => <div data-testid="mock-fab">Mock FAB</div>
}));
import { expect, test, vi } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
    replace: vi.fn(),
    prefetch: vi.fn(),
  }),
  usePathname: () => '',
  useSearchParams: () => new URLSearchParams(),
}));

// Mock fetch to prevent valid Undici errors regarding absolute URLs or missing globals
global.fetch = vi.fn((url: string) => {
  if (url === '/api/walkthrough/dashboard') {
    return Promise.resolve({
      ok: true,
      json: () => Promise.resolve([
        {
          targetId: "sales-card-target",
          title: "Business Analytics",
          content: "This panel shows your current sales and customer counts.",
          position: "bottom"
        },
        {
          targetId: "operations-map-target",
          title: "Operations Map",
          content: "Use this area to see the live state of your orders, messages, and inventory.",
          position: "bottom"
        }
      ])
    });
  }
  return Promise.resolve({
    ok: true,
    json: () => Promise.resolve({})
  });
}) as any;

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

test('renders dashboard with actionable feed', async () => {
  global.fetch = vi.fn((url: string) => {
    if (url === '/api/walkthrough/dashboard') {
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve([
          {
            targetId: "dashboard-title",
            title: "Welcome",
            content: "Welcome to your dashboard! This is your control center.",
            position: "bottom"
          },
          {
            targetId: "wrapped-summary",
            title: "AI Savings",
            content: "Here you can see the time and effort your agents have saved you.",
            position: "bottom"
          }
        ])
      });
    }
    return Promise.resolve({
      ok: true,
      json: () => Promise.resolve({})
    });
  }) as any;
  const { act } = await import('@testing-library/react');
  await act(async () => {
    render(<TooltipProvider><Dashboard /></TooltipProvider>);
  });

  await waitFor(() => {
    expect(screen.queryAllByText("Business Analytics").length).toBeGreaterThan(0);
  });

  expect(screen.getByText("Operations Map")).toBeDefined();
  expect(screen.getByText("Action Required")).toBeDefined();
  expect(screen.getByText("Recent Orders")).toBeDefined();
  expect(screen.queryByText(/\/api\/ui\/dashboard\/unified-feed/)).toBeNull();
  expect(screen.getByText("Inbox Activity")).toBeDefined();
  expect(screen.getByRole("link", { name: /Campaign Orchestration/i })).toHaveAttribute("href", "/feed");
  expect(screen.getByText("Pro Plan ROI Calculator")).toBeDefined();
  expect(screen.getByText("Assistant Tasks")).toBeDefined();
  expect(screen.getByText("My Plan")).toBeDefined();

});
