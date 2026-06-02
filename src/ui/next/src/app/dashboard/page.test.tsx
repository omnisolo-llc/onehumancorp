import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, waitFor, act } from '@testing-library/react';
import Dashboard from './page';
import { expect, test, vi } from 'vitest';

global.fetch = vi.fn(() =>
  Promise.resolve({
    json: () => Promise.resolve({
      total_sales: 0,
      active_customers: 0,
      pending_orders: 0,
      metrics: {
        active_referrals: 0,
        revenue: 0,
        pending_rewards: 0
      },
      pending_approvals: []
    }),
    ok: true,
  })
) as any;

test('renders dashboard with actionable feed', async () => {
  await act(async () => {
    render(<TooltipProvider><Dashboard /></TooltipProvider>);
  });

  await waitFor(() => {
    expect(screen.getByText("Business Analytics")).toBeDefined();
  });

  expect(screen.getByText(/Action Required/)).toBeDefined();
  expect(screen.getByText("Complete Stripe Setup")).toBeDefined();
  expect(screen.getByText("Weekly Insights")).toBeDefined();
  expect(screen.getByText("AI Business Advisory")).toBeDefined();
});
