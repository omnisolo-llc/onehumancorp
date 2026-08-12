import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import InteractiveQuotePage from './page';

const navigation = vi.hoisted(() => ({ id: 'visual-audit-id' }));

vi.mock('next/navigation', () => ({
  useParams: () => navigation,
  useRouter: () => ({ push: vi.fn(), replace: vi.fn() }),
}));

describe('InteractiveQuotePage', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('rejects malformed quote IDs locally without a failing browser request', async () => {
    const fetchMock = vi.fn().mockResolvedValue({ ok: false, status: 400 });
    vi.stubGlobal('fetch', fetchMock);
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => undefined);

    render(<InteractiveQuotePage />);

    await waitFor(() => expect(screen.getByText('Quote not found.')).toBeInTheDocument());
    expect(fetchMock).not.toHaveBeenCalled();
    expect(consoleError).not.toHaveBeenCalled();
  });

  it('accepts a valid quote through the authenticated v1 action route', async () => {
    navigation.id = '11111111-1111-4111-8111-111111111111';
    const fetchMock = vi.fn()
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          quote: {
            service_name: 'Sink repair',
            total_amount_cents: 10000,
            required_deposit_cents: 2500,
          },
          line_items: [],
        }),
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ stripe_payment_link: 'https://checkout.example.test/session' }),
      });
    vi.stubGlobal('fetch', fetchMock);

    render(<InteractiveQuotePage />);
    await screen.findByText('Sink repair');
    fireEvent.change(screen.getByTestId('quote-date-selector'), {
      target: { value: '2026-08-10T10:00' },
    });
    fireEvent.click(screen.getByTestId('pay-deposit-button'));

    await waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(2));
    expect(fetchMock).toHaveBeenNthCalledWith(
      2,
      '/api/v1/quotes/11111111-1111-4111-8111-111111111111/accept',
      { method: 'POST' },
    );
  });
});
