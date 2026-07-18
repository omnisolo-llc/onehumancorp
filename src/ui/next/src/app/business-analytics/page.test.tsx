import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import BusinessAnalytics from './page';
import { expect, test, vi, beforeEach } from 'vitest';
import { TooltipProvider } from '../../components/TooltipRegistry';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn(), refresh: vi.fn() }),
  usePathname: () => '/business-analytics',
  useSearchParams: () => new URLSearchParams(),
}));

beforeEach(() => vi.clearAllMocks());

test('renders recorded business metrics and marks predictions unavailable', async () => {
  global.fetch = vi.fn().mockResolvedValue({
    ok: true,
    json: async () => ({ total_sales: 100, pending_orders: 10, active_customers: 5, total_campaigns_sent: 4 }),
  });
  render(<TooltipProvider><BusinessAnalytics /></TooltipProvider>);

  expect(screen.getByRole('heading', { name: /Business Analytics/i })).toBeInTheDocument();
  expect(await screen.findByText('Recorded Revenue')).toBeInTheDocument();
  expect(screen.getByText('$100.00')).toBeInTheDocument();
  expect(screen.getByText('Active Customers')).toBeInTheDocument();
  expect(screen.getByText(/Forecasts and cohort analytics are unavailable/)).toBeInTheDocument();
  expect(screen.getByText('Back to Dashboard').closest('a')).toHaveAttribute('href', '/dashboard');
});

test('does not display metric cards for invalid backend data', async () => {
  global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ total_sales: 100 }) });
  render(<TooltipProvider><BusinessAnalytics /></TooltipProvider>);
  expect(await screen.findByRole('status')).toHaveTextContent('Business metrics are unavailable.');
  expect(screen.queryByText('Recorded Revenue')).toBeNull();
});
