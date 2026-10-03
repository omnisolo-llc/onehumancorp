import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, expect, it, vi } from 'vitest';
import type { ReactNode } from 'react';
import PaymentLedger from './page';

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: ReactNode }) => <>{children}</>,
}));
afterEach(() => vi.unstubAllGlobals());

it('shows processing and disables duplicate requests while intent creation is pending', async () => {
  let finish!: (response: Response) => void;
  vi.stubGlobal('fetch', vi.fn(async (url: string) => url.endsWith('/intent')
    ? new Promise<Response>((resolve) => { finish = resolve; })
    : Response.json({ total_revenue: 0 })));
  render(<PaymentLedger />);
  await userEvent.setup().click(screen.getByTestId('request-payment-button'));
  expect(screen.getByTestId('request-payment-button')).toHaveTextContent('Waiting for card...');
  expect(screen.getByTestId('request-payment-button')).toBeDisabled();
  finish(Response.json({ payment_id: 'pending', status: 'pending' }));
  expect(await screen.findByTestId('payment-status')).toHaveTextContent('Awaiting payment confirmation');
});

it('updates approved revenue only for a confirmed succeeded payment', async () => {
  let succeeded = false;
  vi.stubGlobal('fetch', vi.fn(async (url: string) => {
    if (url.endsWith('/intent')) {
      succeeded = true;
      return Response.json({ id: 'pi-confirmed', status: 'succeeded' });
    }
    return Response.json({ total_revenue: succeeded ? 50 : 0 });
  }));
  render(<PaymentLedger />);
  expect(await screen.findByTestId('total-revenue')).toHaveTextContent('$0.00');
  await userEvent.setup().click(screen.getByTestId('request-payment-button'));
  expect(await screen.findByTestId('payment-status')).toHaveTextContent('Approved');
  expect(await screen.findByText('$50.00')).toBeVisible();
});

it.each([
  { payment_id: 'pending-ledger-intent', status: 'pending' },
  { id: 'pi-pending', status: 'requires_payment_method', client_secret: 'synthetic-uncollected-intent' },
])('does not report an uncollected $status payment intent as approved', async (intent) => {
  vi.stubGlobal('fetch', vi.fn(async (url: string) => Response.json(
    url.endsWith('/intent')
      ? intent
      : { total_revenue: 0 },
  )));
  render(<PaymentLedger />);
  await userEvent.setup().click(screen.getByTestId('request-payment-button'));
  expect(await screen.findByTestId('payment-status')).toHaveTextContent('Awaiting payment confirmation');
  expect(screen.getByTestId('total-revenue')).toHaveTextContent('$0.00');
  expect(screen.queryByText('Approved')).toBeNull();
});

it('keeps revenue unchanged when the payment provider is unavailable', async () => {
  vi.stubGlobal('fetch', vi.fn(async (url: string) => url.endsWith('/intent')
    ? Response.json({ error: 'payment_provider_unavailable' }, { status: 503 })
    : Response.json({ total_revenue: 0 })));
  render(<PaymentLedger />);
  await userEvent.setup().click(screen.getByTestId('request-payment-button'));
  expect(await screen.findByTestId('payment-status')).toHaveTextContent('Failed to initialize');
  expect(screen.getByTestId('total-revenue')).toHaveTextContent('$0.00');
  expect(screen.getByTestId('request-payment-button')).toBeEnabled();
});
