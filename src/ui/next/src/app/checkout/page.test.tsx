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

  it('handles locale and currency changes', async () => {
    render(<CheckoutPage />);

    const localeSelect = screen.getByTestId('locale-select');
    fireEvent.change(localeSelect, { target: { value: 'ar-AE' } });

    expect(screen.getByText('الدفع')).toBeDefined();
    expect(screen.getByText('ادفع الآن')).toBeDefined();

    const currencySelect = screen.getByTestId('currency-select');
    fireEvent.change(currencySelect, { target: { value: 'AED' } });

    // Mock prompt and alert for offline test
    global.prompt = vi.fn().mockReturnValue('100');
    global.alert = vi.fn();

    // Simulate going offline
    const originalNavigator = global.navigator;
    Object.defineProperty(global, 'navigator', {
        value: { onLine: false },
        configurable: true
    });

    const tapToPayButton = screen.getByText('انقر للدفع (Stripe Terminal)');
    fireEvent.click(tapToPayButton);

    // Verify localStorage queue has AED and exchange rate 3.67
    const queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
    expect(queue.length).toBeGreaterThan(0);
    const lastEvent = queue[queue.length - 1];
    expect(lastEvent.currency).toBe('AED');
    expect(lastEvent.exchange_rate).toBe(3.67);

    // Restore navigator
    Object.defineProperty(global, 'navigator', {
        value: originalNavigator,
        configurable: true
    });
  });
});
