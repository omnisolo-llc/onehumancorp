import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import QuoteReviewPage from './page';
import { useParams, useRouter } from 'next/navigation';

jest.mock('next/navigation', () => ({
  useParams: jest.fn(),
  useRouter: jest.fn(),
}));

describe('QuoteReviewPage', () => {
  const mockRouter = { back: jest.fn(), push: jest.fn() };

  beforeEach(() => {
    (useParams as jest.Mock).mockReturnValue({ id: '123' });
    (useRouter as jest.Mock).mockReturnValue(mockRouter);
    global.fetch = jest.fn() as jest.Mock;
  });

  it('renders quote details and allows approval', async () => {
    (global.fetch as jest.Mock).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        id: '123',
        status: 'DRAFT',
        total_amount_cents: 10000,
        required_deposit_cents: 3333,
        line_items: [{ id: 'li1', description: 'Item 1', unit_price_cents: 10000, quantity: 1 }]
      }),
    });

    render(<QuoteReviewPage />);

    await waitFor(() => expect(screen.getByText('Item 1 (x1)')).toBeInTheDocument());
    expect(screen.getByText('00.00')).toBeInTheDocument();

    const approveBtn = screen.getByText('Approve & Send Quote');

    (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => ({ status: 'ACCEPTED', stripe_payment_link: 'http://stripe.com' })
    });

    fireEvent.click(approveBtn);
    // Note: window.alert might need mocking if it fails in jsdom
    await waitFor(() => expect(screen.getByText('ACCEPTED')).toBeInTheDocument());
  });
});
