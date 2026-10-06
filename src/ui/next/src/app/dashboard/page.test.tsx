import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, waitFor, fireEvent, act, within } from '@testing-library/react';
import Dashboard from './page';
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
  if (url === '/api/v1/walkthrough/dashboard') {
    return Promise.resolve(Response.json([
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
      ], { status: 200 }));
  }
  return Promise.resolve(Response.json({}, { status: 200 }));
});

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
    if (url === '/api/v1/walkthrough/dashboard') {
      return Promise.resolve(Response.json([
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
        ], { status: 200 }));
    }
    return Promise.resolve(Response.json({}, { status: 200 }));
  });
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

}, 30000);

test('does not request dashboard APIs that have no server contract', async () => {
  const requestedUrls: string[] = [];
  global.fetch = vi.fn((url: string) => {
    requestedUrls.push(url);
    return Promise.resolve(Response.json({}, { status: 200 }));
  });

  const { act } = await import('@testing-library/react');
  await act(async () => {
    render(<TooltipProvider><Dashboard /></TooltipProvider>);
  });

  await waitFor(() => {
    expect(screen.getByText('Action Required')).toBeDefined();
  });

  expect(requestedUrls).not.toContain('/api/v1/ledger/accounts');
  expect(requestedUrls).not.toContain('/api/v1/user/usage');
  expect(requestedUrls).not.toContain('/api/v1/mesh/v2/collective?action=getNearby');
}, 30000);

test('financial summary never invents an available balance', async () => {
  global.fetch = vi.fn(async () => Response.json({}));
  await act(async () => { render(<TooltipProvider><Dashboard /></TooltipProvider>); });
  const card = within(screen.getByTestId('dashboard-financials-card'));
  expect(card.queryByText('$1,500.00 USD')).toBeNull();
  expect(card.getByText('Balance unavailable')).toBeVisible();
  expect(card.getByRole('link', { name: 'Recent Activity' })).toHaveAttribute('href', '/dashboard/ledger');
}, 30000);


test('reports unavailable migration instead of inventing a completed import', async () => {
  const requests: string[] = [];
  global.fetch = vi.fn((url: string) => {
    requests.push(url);
    return Promise.resolve(Response.json({}, { status: 200 }));
  });
  vi.useFakeTimers();
  try {
    await act(async () => { render(<TooltipProvider><Dashboard /></TooltipProvider>); });
    fireEvent.click(screen.getByRole('button', { name: 'Migrate Existing Store' }));
    const start = screen.queryByRole('button', { name: 'Start Migration' });
    if (start) fireEvent.click(start);
    await act(async () => { await vi.advanceTimersByTimeAsync(1000); });
    expect(screen.queryByText('Migration Complete!')).toBeNull();
    expect(screen.getByText(/Automatic store migration is not available yet/)).toBeVisible();
    expect(screen.queryByRole('button', { name: 'Start Migration' })).toBeNull();
    expect(screen.getByRole('link', { name: 'Open product catalog' })).toHaveAttribute('href', '/products');
    expect(requests.filter(url => /migration|myshopify/i.test(url))).toEqual([]);
  } finally {
    vi.clearAllTimers();
    vi.useRealTimers();
  }
}, 30000);

test.each([
  [undefined, 'Unavailable'],
  [{ error: 'supply_unavailable', success: false }, 'Unavailable'],
  [{ vendors: null, raw_materials: [], bom_items: [] }, 'Unavailable'],
  [{ vendors: [], raw_materials: [], bom_items: [] }, '0'],
  [{ vendors: [{ id: 'owned-vendor', name: 'Recorded vendor' }], raw_materials: [], bom_items: [] }, '1'],
])('vendor summary distinguishes unavailable supply from valid empty data: %j', async (supply, expected) => {
  global.fetch = vi.fn(async (url: string) => Response.json(url.startsWith('/api/v1/ui/dashboard/unified-feed')
    ? { supply, orders: [{ id: 'owned-order', status: 'pending', total_amount: 12, customer_name: 'Recorded customer' }] }
    : {}));
  await act(async () => { render(<TooltipProvider><Dashboard /></TooltipProvider>); });
  const card = screen.getByText('Vendors', { exact: true }).parentElement!;
  expect(within(card).getByText(expected, { exact: true })).toBeVisible();
  expect(screen.getByText('Operations Map')).toBeVisible();
  expect(screen.getByText('Recorded customer')).toBeVisible();
}, 30000);

test('vendor summary remains loading until supply is observed', async () => {
  let finish!: (response: Response) => void;
  global.fetch = vi.fn(async (url: string) => url.startsWith('/api/v1/ui/dashboard/unified-feed')
    ? new Promise<Response>(resolve => { finish = resolve; }) : Response.json({}));
  render(<TooltipProvider><Dashboard /></TooltipProvider>);
  const card = screen.getByText('Vendors', { exact: true }).parentElement!;
  expect(within(card).getByText('Loading…')).toBeVisible();
  await act(async () => { finish(Response.json({ supply: { vendors: [], raw_materials: [], bom_items: [] } })); });
  await waitFor(() => expect(within(card).getByText('0', { exact: true })).toBeVisible());
}, 30000);

test('mobile supply without thresholds never invents a healthy zero stock warning count', async () => {
  global.fetch = vi.fn(async (url: string) => Response.json(url.startsWith('/api/v1/ui/dashboard/unified-feed')
    ? { supply: { vendors: [{ id: 'vendor', name: 'Vendor' }], raw_materials: [{ id: 'flour', name: 'Flour', current_quantity: 2 }], bom_items: [] } }
    : {}));
  await act(async () => { render(<TooltipProvider><Dashboard /></TooltipProvider>); });
  expect(within(screen.getByText('Low Stock', { exact: true }).parentElement!).getByText('Unavailable')).toBeVisible();
  expect(within(screen.getByText('Vendors', { exact: true }).parentElement!).getByText('1', { exact: true })).toBeVisible();
}, 30000);
