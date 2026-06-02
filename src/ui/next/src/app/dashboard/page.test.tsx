import { TooltipProvider } from '../../components/TooltipRegistry';
import { vi, expect, test } from 'vitest';
import { render, screen, act } from '@testing-library/react';
import Dashboard from './page';

global.fetch = vi.fn().mockResolvedValue({
  ok: true,
  json: async () => ({
    total_sales: 100,
    active_customers: 10,
    pending_orders: 5,
    metrics: { team_invites_sent: 2, active_referrals: 3, revenue: 50, pending_rewards: 10 }
  })
});

test('renders dashboard with actionable feed', async () => {
  await act(async () => {
    render(<TooltipProvider><Dashboard /></TooltipProvider>);
  });
  expect(screen.getByText("Business Analytics")).toBeDefined();
  expect(screen.getByText(/Action Required/)).toBeDefined();
  expect(screen.getByText("Complete Stripe Setup")).toBeDefined();
  expect(screen.getByText("Weekly Insights")).toBeDefined();
  expect(screen.getByText("AI Business Advisory")).toBeDefined();
});
