import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import CheckoutPage from './page';

const mockUseSearchParams = vi.fn(() => new URLSearchParams(''));
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
  useSearchParams: () => mockUseSearchParams(),
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
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

    const payButton = screen.getByText('Pay with Stripe');
    fireEvent.click(payButton);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/billing/create-checkout-session', expect.objectContaining({
        method: 'POST',
        headers: expect.objectContaining({ 'Content-Type': 'application/json' }),
        body: JSON.stringify({
          tier: 'Starter'
        }),
      }));
      expect(assign).toHaveBeenCalledWith('https://checkout.stripe.com/pay/test');
    });
  });

  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the checkout page', () => {
    render(<CheckoutPage />);
    expect(screen.getByText('Checkout')).toBeDefined();
    expect(screen.getByText('Pay $45.00')).toBeDefined();
  });

  it('handles payment click', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ referral_link: 'http://test.link' })
    } as any);

    render(<CheckoutPage />);

    const payButton = screen.getByText('Pay $45.00');
    fireEvent.click(payButton);

    expect(payButton.textContent).toBe('Processing...');

    await waitFor(() => {
      expect(screen.getByText('Payment Successful!')).toBeDefined();
    });
  });

  it('includes browser coordinates when checking delivery eligibility', async () => {
    Object.defineProperty(navigator, 'geolocation', {
      configurable: true,
      value: {
        getCurrentPosition: vi.fn((success) => success({
          coords: { latitude: 37.77, longitude: -122.41 },
        })),
      },
    });
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ success: true, fee: 8.5 }),
    } as any);

    render(<CheckoutPage />);

    fireEvent.change(screen.getByPlaceholderText('Enter delivery address...'), {
      target: { value: '123 Market St' },
    });
    fireEvent.click(screen.getByText('Check'));

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/checkout/delivery-quote', expect.objectContaining({
        method: 'POST',
        headers: expect.objectContaining({ 'Content-Type': 'application/json' }),
        body: JSON.stringify({
          deliveryAddress: '123 Market St',
          coordinates: { lat: 37.77, lng: -122.41 },
        }),
      }));
    });
  });

  it('starts a MercadoPago checkout and redirects to the provider URL', async () => {
    const assign = vi.fn();
    Object.defineProperty(window, 'location', {
      configurable: true,
      value: { assign },
    });
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        checkout_url: 'https://www.mercadopago.com/checkout/v1/redirect?pref_id=real',
      }),
    } as any);

    render(<CheckoutPage />);

    fireEvent.click(screen.getByText('Pay with Mercado Pago'));

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/checkout/mercadopago', expect.objectContaining({
        method: 'POST',
        headers: expect.objectContaining({ 'Content-Type': 'application/json' }),
        body: JSON.stringify({
          tenant_id: 'fake-token',
          amount_cents: 4500,
          currency: 'MXN',
        }),
      }));
      expect(assign).toHaveBeenCalledWith('https://www.mercadopago.com/checkout/v1/redirect?pref_id=real');
    });
  });
});
