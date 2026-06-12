import { render, screen, waitFor } from '@testing-library/react';
import PricingPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';
import { vi, describe, test, expect, afterEach } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({
    push: vi.fn()
  }))
}));

describe('PricingPage', () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  test('shows loading state initially', () => {
    global.fetch = vi.fn(() => new Promise(() => {})) as any;
    render(<TooltipProvider><PricingPage /></TooltipProvider>);
    expect(screen.getByTestId('pricing-loading')).toBeDefined();
  });

  test('renders plans correctly after loading', async () => {
    const mockPlans = [
      { id: 'free', name: 'Free', price_cents: 0, ai_action_limit: 100, storage_limit_mb: 500, agent_limit: 1, product_limit: 10 },
      { id: 'starter', name: 'Starter', price_cents: 2900, ai_action_limit: 1000, storage_limit_mb: 5120, agent_limit: 3, product_limit: 100 },
      { id: 'pro', name: 'Pro', price_cents: 7900, ai_action_limit: null, storage_limit_mb: 51200, agent_limit: 10, product_limit: null },
      { id: 'business', name: 'Business', price_cents: 29900, ai_action_limit: null, storage_limit_mb: 512000, agent_limit: null, product_limit: null },
    ];

    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes('pricing-plans')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ plans: mockPlans })
        });
      } else if (url.includes('my-plan')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ current_plan: 'Free' })
        });
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    render(<TooltipProvider><PricingPage /></TooltipProvider>);

    await waitFor(() => {
      expect(screen.queryByTestId('pricing-loading')).toBeNull();
    });

    expect(screen.getByText('Free')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('Pro')).toBeDefined();
    expect(screen.getByText('Business')).toBeDefined();

    expect(screen.getByText('100 AI actions / month')).toBeDefined();
    expect(screen.getByText('100 Products Limit')).toBeDefined();
  });
});
