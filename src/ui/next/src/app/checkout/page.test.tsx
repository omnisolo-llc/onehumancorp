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

  it('renders the checkout page and handles local delivery', async () => {
    // Mock localStorage to enable DoorDash
    vi.spyOn(Storage.prototype, 'getItem').mockImplementation((key) => {
      if (key === 'ohc_settings_preferences') {
        return JSON.stringify({ doordash_enabled: true });
      }
      return null;
    });

    render(<CheckoutPage />);
    expect(screen.getByText('Checkout')).toBeDefined();
    expect(screen.getByText('Pay Now')).toBeDefined();
    expect(screen.getByText('Delivery Information')).toBeDefined();

    // Enter address
    const addressInput = screen.getByPlaceholderText('e.g. 123 Main St, San Francisco, CA');
    fireEvent.change(addressInput, { target: { value: '123 Delivery St' } });

    // Check for quote button
    const checkQuoteBtn = screen.getByText('Check Local Delivery Quote');
    expect(checkQuoteBtn).toBeDefined();

    // Click quote button and simulate API wait
    fireEvent.click(checkQuoteBtn);
    expect(screen.getByText('Checking availability...')).toBeDefined();

    await waitFor(() => {
        expect(screen.getByText('Address is within delivery radius. DoorDash delivery fee: $7.50')).toBeDefined();
    }, { timeout: 1500 });

    // Check order summary includes delivery fee
    expect(screen.getByText('Local Delivery (DoorDash)')).toBeDefined();
    expect(screen.getByText('$7.50')).toBeDefined();
    expect(screen.getByText('$52.50')).toBeDefined(); // 45.00 + 7.50
  });

  it('handles payment click', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ referral_link: 'http://test.link' })
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
