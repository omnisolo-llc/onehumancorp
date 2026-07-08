import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import QuoteReviewPage from './page';
import { useParams, useRouter } from 'next/navigation';
import { vi, describe, it, expect, beforeEach } from 'vitest';
import { TooltipProvider } from '../../../components/TooltipRegistry';

vi.mock('next/navigation', () => ({
  useParams: vi.fn(),
  useRouter: vi.fn(),
  usePathname: vi.fn(() => '/quotes/123'),
}));

describe('QuoteReviewPage', () => {
  const mockRouter = { back: vi.fn(), push: vi.fn() };

  beforeEach(() => {
    vi.clearAllMocks();
    (useParams as any).mockReturnValue({ id: '123' });
    (useRouter as any).mockReturnValue(mockRouter);
    global.fetch = vi.fn((url) => {
      if (url === '/api/tooltips') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({}),
        });
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({}),
      });
    }) as any;
    // Mock window.alert
    global.alert = vi.fn();
  });

  it('renders quote details and allows approval', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        id: '123',
        status: 'DRAFT',
        total_amount_cents: 10000,
        required_deposit_cents: 3333,
        line_items: [{ id: 'li1', description: 'Item 1', unit_price_cents: 10000, quantity: 1 }]
      }),
    });

    render(
      <TooltipProvider>
        <QuoteReviewPage />
      </TooltipProvider>
    );

    await waitFor(() => expect(screen.getByText('Item 1 (x1)')).toBeInTheDocument());
    expect(screen.getAllByText('$100.00').length).toBeGreaterThan(0);

    const approveBtn = screen.getByText('Approve & Send Quote');

    (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => ({ status: 'ACCEPTED', stripe_payment_link: 'http://stripe.com' })
    });

    fireEvent.click(approveBtn);
    await waitFor(() => expect(screen.getByText('ACCEPTED')).toBeInTheDocument());
    expect(global.alert).toHaveBeenCalled();
  });
});
