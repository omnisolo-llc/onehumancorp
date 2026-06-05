import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import CheckoutPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

vi.mock('../../components/TooltipRegistry', () => ({
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
  });

  it('handles payment click', async () => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/v1/growth/upsell/generate') {
        return Promise.resolve({
          json: () => Promise.resolve({
            success: true,
            product_name: 'Premium Matches',
            discounted_price: 10.2,
            original_price: 12.0,
            discount_percentage: 15,
            description: 'Perfectly pairs with your cart items!'
          })
        });
      }
      return Promise.resolve({
        json: () => Promise.resolve({ referral_link: 'http://test.link' })
      });
    });

    render(<CheckoutPage />);

    const payButton = screen.getByText('Pay Now');
    fireEvent.click(payButton);

    expect(payButton.textContent).toBe('Processing...');

    await waitFor(() => {
      expect(screen.getByText('Payment Successful!')).toBeDefined();
    });
  });
});
