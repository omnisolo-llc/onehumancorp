import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import CheckoutPage from './page';
import * as React from 'react';

const mockPush = vi.fn();
const mockUseSearchParams = vi.fn();

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
  useSearchParams: () => mockUseSearchParams(),
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

vi.mock('../components/OneTapReferral', () => ({
  OneTapReferral: () => <div data-testid="one-tap-referral" />,
}));

describe('CheckoutPage', () => {
  afterEach(() => {
    mockUseSearchParams.mockImplementation(() => new URLSearchParams(''));
  });
beforeEach(() => {
  vi.clearAllMocks();
  mockUseSearchParams.mockReturnValue(new URLSearchParams(''));
  Object.defineProperty(window, 'localStorage', {
    value: {
      getItem: vi.fn(() => 'fake-token'),
      setItem: vi.fn(),
    },
    writable: true
  });
});

  it('displays subscription UI when tier is provided and handles Stripe checkout', async () => {
    mockUseSearchParams.mockImplementation(() => new URLSearchParams('?tier=Starter'));
    const assign = vi.fn();
    Object.defineProperty(window, 'location', {
      configurable: true,
      value: { assign },
    });
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        checkout_url: 'https://checkout.stripe.com/pay/test',
      }),
    } as any);

    render(<CheckoutPage />);

    expect(screen.getByText('Plan Upgrade')).toBeDefined();
    expect(screen.getByText('OHC Starter Plan')).toBeDefined();

    const payButton = screen.getByText('Upgrade via Stripe');
    fireEvent.click(payButton);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/billing/create-checkout-session', expect.objectContaining({
        method: 'POST',
        headers: expect.objectContaining({
            'Authorization': 'Bearer fake-token'
        })
      }));
      expect(assign).toHaveBeenCalledWith('https://checkout.stripe.com/pay/test');
    });
  });

  it('handles regular checkout deposit flow correctly', async () => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/v1/payments/terminal/reserve') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ success: true, lock_id: 'test-lock' }),
        });
      }
      if (url === '/api/v1/payments/terminal/commit') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ success: true }),
        });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
    });

    render(<CheckoutPage />);

    expect(screen.getByText('Secure Checkout')).toBeDefined();
    expect(screen.getByText('Service Deposit')).toBeDefined();
    expect(screen.getByText('Subscribe & Save 10%')).toBeDefined();
    expect(screen.getAllByText('$45.00')[0]).toBeDefined();

    const payButton = screen.getByText('Pay');
    fireEvent.click(payButton);

    await waitFor(() => {
      expect(screen.getByText('Payment Successful!')).toBeDefined();
      expect(screen.getByTestId('one-tap-referral')).toBeDefined();
      expect(screen.getByText(/Your order is confirmed/)).toBeDefined();
    }, { timeout: 2000 });
  });

  it('handles delivery quote flow correctly', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        success: true,
        fee: 10.50,
      }),
    } as any);

    render(<CheckoutPage />);

    const addressInput = screen.getByPlaceholderText('Enter address for delivery quote');
    fireEvent.change(addressInput, { target: { value: '123 Main St' } });

    const checkButton = screen.getByText('Check');
    fireEvent.click(checkButton);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/checkout/delivery-quote', expect.objectContaining({
        method: 'POST'
      }));
      expect(screen.getByText('Delivery available: +$10.50')).toBeDefined();
      expect(screen.getByText('Total with Delivery')).toBeDefined();
      expect(screen.getByText('$55.50')).toBeDefined();
    });
  });

  it('handles Mercado Pago checkout correctly', async () => {
    const assign = vi.fn();
    Object.defineProperty(window, 'location', {
      configurable: true,
      value: { assign },
    });

    // We must mock the /api/v1/payments/terminal/reserve call to succeed before it calls mercadopago
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/v1/payments/terminal/reserve') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ success: true, lock_id: 'test-lock' }),
        });
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({
          checkout_url: 'https://mercadopago.com/checkout/test',
        }),
      });
    });

    render(<CheckoutPage />);

    const payButton = screen.getByText('Pay with Mercado Pago');
    fireEvent.click(payButton);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/checkout/mercadopago', expect.objectContaining({
        method: 'POST',
        body: expect.stringContaining('"amount_cents":4500')
      }));
      expect(assign).toHaveBeenCalledWith('https://mercadopago.com/checkout/test');
    });
  });

  it('handles Mercado Pago checkout correctly for subscription tier', async () => {
    mockUseSearchParams.mockImplementation(() => new URLSearchParams('?tier=Pro'));
    const assign = vi.fn();
    Object.defineProperty(window, 'location', {
      configurable: true,
      value: { assign },
    });
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        checkout_url: 'https://mercadopago.com/checkout/test_sub',
      }),
    } as any);

    render(<CheckoutPage />);

    const payButton = screen.getByText('Upgrade via Mercado Pago');
    fireEvent.click(payButton);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/checkout/mercadopago', expect.objectContaining({
        method: 'POST',
        body: expect.stringContaining('"amount_cents":7900')
      }));
      expect(assign).toHaveBeenCalledWith('https://mercadopago.com/checkout/test_sub');
    });
  });
});
