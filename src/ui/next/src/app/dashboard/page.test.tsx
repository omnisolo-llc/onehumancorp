import { TooltipProvider } from '../../components/TooltipRegistry';
import { render, screen, act } from '@testing-library/react';
import Dashboard from './page';
import { expect, test, vi, beforeEach, afterEach } from 'vitest';

beforeEach(() => {
  global.fetch = vi.fn((url: string | URL | Request) => {
    const urlStr = url.toString();
    if (urlStr.includes('/api/v1/growth/milestones/check')) {
       return Promise.resolve({
         json: () => Promise.resolve({}),
         ok: true,
         status: 200,
       });
    }
    if (urlStr.includes('/api/agents/approvals')) {
       return Promise.resolve({
         json: () => Promise.resolve({
           pending_approvals: []
         }),
         ok: true,
         status: 200,
       });
    }
    if (urlStr.includes('/api/v1/dashboard/metrics')) {
       return Promise.resolve({
         json: () => Promise.resolve({
           total_sales: 100,
           active_customers: 5,
           pending_orders: 2,
         }),
         ok: true,
         status: 200,
       });
    }
    return Promise.resolve({
      json: () => Promise.resolve({}),
      ok: true,
      status: 200,
    });
  }) as any;
});

afterEach(() => {
  vi.restoreAllMocks();
});

test('renders dashboard with actionable feed', async () => {
  await act(async () => {
    render(<TooltipProvider><Dashboard /></TooltipProvider>);
  });
  expect(screen.getByText("Today's Pulse")).toBeDefined();
  expect(screen.getByText("Action Required")).toBeDefined();
  expect(screen.getByText(/2 Custom Cake Orders to Review/)).toBeDefined();
  expect(screen.getByText(/Approve Instagram post/)).toBeDefined();
  expect(screen.getByText(/Weekly Insights Available/)).toBeDefined();
});
