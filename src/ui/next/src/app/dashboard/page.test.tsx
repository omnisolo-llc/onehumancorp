import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, act } from '@testing-library/react';
import Dashboard from './page';
import { expect, test, vi } from 'vitest';

test('renders dashboard with actionable feed', async () => {

  // Mock fetch just for this test
  global.fetch = vi.fn().mockImplementation((url, options) => {
    if (typeof url === 'string' && url.includes('/api/v1/dashboard/metrics')) {
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({
          totalSales: 1200,
          uniqueVisitors: 450,
          ordersCount: 12,
          conversionRate: 2.5,
          totalSalesTrend: 10,
          visitorsTrend: 5,
          ordersTrend: -2,
          conversionTrend: 1,
          orders: [],
          actions: [],
          health: { activeAgents: 2, systemStatus: 'ok' }
        })
      });
    }
    return Promise.resolve({
      ok: true,
      json: () => Promise.resolve([])
    });
  });

  await act(async () => {
    render(<TooltipProvider><Dashboard /></TooltipProvider>);
  });

  expect(screen.getByText("Business Analytics")).toBeDefined();
  expect(screen.getByText(/Action Required/)).toBeDefined();
  expect(screen.getByText("Complete Stripe Setup")).toBeDefined();
  expect(screen.getByText("Weekly Insights")).toBeDefined();
  expect(screen.getByText("AI Business Advisory")).toBeDefined();
});
