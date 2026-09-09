import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import POSTerminalMobile from './page';

const { useSearchParamsMock } = vi.hoisted(() => ({
  useSearchParamsMock: vi.fn(),
}));

vi.mock('next/navigation', () => ({
  useSearchParams: useSearchParamsMock,
}));

vi.mock('../terminal/StripeTerminalClient', () => ({
  default: () => <div data-testid="stripe-terminal-client" />,
}));

describe('POSTerminalMobile', () => {
  beforeEach(() => {
    useSearchParamsMock.mockReturnValue(new URLSearchParams('tenantId=e2e-tenant'));
    localStorage.clear();
    vi.mocked(global.fetch).mockResolvedValueOnce(new Response(JSON.stringify([
      {
        id: 'e2e-product-cake',
        title: 'Vegan Celebration Cake',
        price_cents: 3999,
        image_url: '/dashboard_with_charts.png',
      },
    ]), {
      status: 200,
      headers: { 'Content-Type': 'application/json' },
    }));
  });

  it('normalizes the backend catalog contract before rendering prices', async () => {
    Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
    render(<POSTerminalMobile />);

    await waitFor(() => expect(screen.getByText('Vegan Celebration Cake')).toBeInTheDocument());
    expect(screen.getByText('$39.99')).toBeInTheDocument();
    expect(global.fetch).toHaveBeenCalledWith('/api/v1/catalog/products');
    expect(screen.getByRole('heading', { name: 'mPOS' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Quick Charge' })).toBeDisabled();
  });

  it('shows a loading fallback while the query string is being resolved', () => {
    const pendingSearchParams = new Promise<never>(() => {});
    useSearchParamsMock.mockImplementation(() => {
      throw pendingSearchParams;
    });

    render(<POSTerminalMobile />);

    expect(screen.getByText('Loading mPOS...')).toBeInTheDocument();
  });
});
