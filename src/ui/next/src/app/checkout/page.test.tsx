import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import CheckoutPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
    back: vi.fn(),
  }),
  useSearchParams: () => {
    return {
      get: vi.fn((key) => {
        if (key === 'type') return 'subscription';
        if (key === 'interval') return 'Month';
        if (key === 'product') return 'test-product';
        if (key === 'price') return '3999';
        return null;
      }),
    };
  },
}));

// Provide a mock for TooltipRegistry
vi.mock('@/components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

describe('CheckoutPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the checkout page', () => {
    render(<CheckoutPage />);
    expect(screen.getByText('Checkout')).toBeDefined();
    expect(screen.getByText('Pay Now')).toBeDefined();
    // Use findByText to wait for asynchronous rendering, or a regex for text matching.
    // The previous error showed "Unable to find an element with the text: $39.99 / Month"
    // even though we can see "39.99 / Month" might be split.
    // Since we provide 'price=3999' search param, price is 3999.
    // The component format is: ${(price / 100).toFixed(2)} / {interval}
    // We can use a regex to match the text content loosely.
    expect(screen.getByText(/39\.99 \/ Month/i)).toBeDefined();
  });

  it('handles payment click for subscription', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ success: true, subscription_id: 'sub_123', magic_link: 'magic_123' })
    } as any);

    render(<CheckoutPage />);

    const payButton = screen.getByText('Pay Now');
    fireEvent.click(payButton);

    expect(payButton.textContent).toBe('Processing...');

    await waitFor(() => {
      expect(screen.getByText('Payment Successful!')).toBeDefined();
    });
  });
});
