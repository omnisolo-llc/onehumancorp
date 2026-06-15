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
global.fetch = vi.fn(() => Promise.resolve({
  ok: true,
  json: () => Promise.resolve({
    total_revenue_cents: 150000,
    active_customers: 25,
    pending_orders: 5
  })
})) as any;

test('renders dashboard with actionable feed', async () => {
  const { act } = await import('@testing-library/react');
  await act(async () => {
    render(<TooltipProvider><Dashboard /></TooltipProvider>);
  });

  await waitFor(() => {
    expect(screen.getByText("Dashboard")).toBeDefined();
    expect(screen.getByText("Total Revenue")).toBeDefined();
    expect(screen.getByText("Active Customers")).toBeDefined();
    expect(screen.getByText("Pending Orders")).toBeDefined();
    expect(screen.getByText("$1500.00")).toBeDefined();
    expect(screen.getByText("25")).toBeDefined();
    expect(screen.getByText("5")).toBeDefined();
  });
});
