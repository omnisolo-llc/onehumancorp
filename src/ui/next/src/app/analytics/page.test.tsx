import React from 'react';
import '@testing-library/jest-dom';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import AnalyticsPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn(), refresh: vi.fn() }),
  usePathname: () => '/analytics',
  useSearchParams: () => new URLSearchParams(),
}));

describe('AnalyticsPage', () => {
  beforeEach(() => vi.clearAllMocks());

  it('renders only metrics returned by the dashboard API', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ total_sales: 1250, active_customers: 14, pending_orders: 3, total_campaigns_sent: 8 }),
    });
    render(<TooltipProvider><AnalyticsPage /></TooltipProvider>);

    expect(await screen.findByText('Recorded Revenue')).toBeInTheDocument();
    expect(screen.getByText('$1,250.00')).toBeInTheDocument();
    expect(screen.getByText('Active Customers')).toBeInTheDocument();
    expect(screen.getByText('Pending Orders')).toBeInTheDocument();
    expect(screen.getByText('Campaigns Sent')).toBeInTheDocument();
    expect(screen.getByText(/Predictive analytics are unavailable/)).toBeInTheDocument();
  });

  it('shows an unavailable state when metrics fail', async () => {
    global.fetch = vi.fn().mockResolvedValue({ ok: false });
    render(<TooltipProvider><AnalyticsPage /></TooltipProvider>);
    expect(await screen.findByRole('status')).toHaveTextContent('Analytics data is unavailable.');
    expect(screen.queryByText('Recorded Revenue')).toBeNull();
  });
});
