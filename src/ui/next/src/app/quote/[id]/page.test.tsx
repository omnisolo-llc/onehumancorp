import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import InteractiveQuotePage from './page';
import CustomerProposalView from '../../proposals/customer-view/page';

const route = vi.hoisted(() => ({ id: '', pendingSearchParams: null as Promise<void> | null }));
vi.mock('next/navigation', () => ({
  useParams: () => ({ id: route.id }),
  useSearchParams: () => {
    if (route.pendingSearchParams) throw route.pendingSearchParams;
    return new URLSearchParams(route.id ? { id: route.id } : {});
  },
}));

const id = '11111111-1111-4111-8111-111111111111';
const otherId = '22222222-2222-4222-8222-222222222222';
const invoiceId = '33333333-3333-4333-8333-333333333333';
const version = '2026-10-04T01:00:00.123456+01:00';
const updatedVersion = '2026-10-04T00:00:00.123457Z';
const item = { id: 'line-1', quote_id: id, description: 'Site visit', unit_price_cents: 12000, quantity: 1, is_optional: false };
function detail(quote: Record<string, unknown> = {}, acceptance: unknown = null, lines = [item]) {
  return { quote: { id, customer_id: 'owned-customer', status: 'SENT', updated_at: version,
    total_amount_cents: 12000, required_deposit_cents: 4000, stripe_payment_link: null, ...quote },
  line_items: lines, acceptance };
}
function receipt(overrides: Record<string, unknown> = {}) {
  return { success: true, status: 'accepted', quote_id: id, invoice_id: invoiceId,
    invoice_status: 'Draft', payment_status: 'unverified', checkout_status: 'not_configured',
    stripe_payment_link: '', reason: 'checkout_not_configured', ...overrides };
}
function accepted(value = receipt()) {
  return detail({ status: 'ACCEPTED', updated_at: updatedVersion }, value);
}
function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>(r => { resolve = r; });
  return { promise, resolve };
}
const fetcher = vi.mocked(fetch);
const posts = () => fetcher.mock.calls.filter(([, options]) => options?.method === 'POST');
const acceptButton = () => screen.getByRole('button', { name: 'Accept quote' });
const successfulRead = () => Response.json(accepted());

beforeEach(() => {
  route.id = id;
  route.pendingSearchParams = null;
  fetcher.mockReset();
});
afterEach(cleanup);

describe.each([
  ['quote route', InteractiveQuotePage],
  ['proposal query route', CustomerProposalView],
] as const)('%s customer acceptance', (_name, Page) => {
  async function ready() {
    const view = render(<Page />);
    await screen.findByText(/Site visit/);
    return view;
  }
  async function submit() {
    await ready();
    fireEvent.click(acceptButton());
  }

  it('marks the real read as pending until its result is rendered', async () => {
    const pending = deferred<Response>();
    fetcher.mockReturnValueOnce(pending.promise);
    render(<Page />);
    expect(screen.getByRole('status')).toHaveTextContent('Loading quote...');
    expect(screen.getByRole('status')).toHaveAttribute('aria-busy', 'true');
    expect(screen.queryByRole('button', { name: 'Accept quote' })).toBeNull();
    await act(async () => pending.resolve(Response.json(detail())));
    await screen.findByText('Site visit x1');
    expect(document.querySelector('[aria-busy="true"]')).toBeNull();
  });

  it('explains why an invalid reference cannot be refreshed', async () => {
    route.id = 'e2e-id';
    render(<Page />);
    await screen.findByText('Quote not found.');
    expect(screen.getByText('The quote reference is invalid. Open a valid quote link.')).toBeVisible();
    expect(screen.getByRole('button', { name: 'Refresh quote' })).toBeDisabled();
    expect(fetcher).not.toHaveBeenCalled();
  });

  it('reports the completed missing result after manual refresh', async () => {
    fetcher.mockImplementation(async () => new Response(null, { status: 404 }));
    render(<Page />);
    await screen.findByText('Quote not found.');
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    await screen.findByText('Refresh complete. Quote not found.');
    expect(fetcher).toHaveBeenCalledTimes(2);
    expect(posts()).toHaveLength(0);
    const repeated = deferred<Response>();
    fetcher.mockReturnValueOnce(repeated.promise);
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    expect(screen.queryByText('Refresh complete. Quote not found.')).toBeNull();
    await act(async () => repeated.resolve(new Response(null, { status: 404 })));
    await screen.findByText('Refresh complete. Quote not found.');
    expect(fetcher).toHaveBeenCalledTimes(3);
  });

  it('reports refreshing the same saved terms without attempting acceptance', async () => {
    fetcher.mockImplementation(async () => Response.json(detail()));
    await ready();
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    await screen.findByText('Quote refreshed from saved data.');
    expect(screen.getByText('Site visit x1')).toBeVisible();
    expect(posts()).toHaveLength(0);
  });

  it('distinguishes a failed refresh from a completed saved read', async () => {
    fetcher.mockImplementation(async () => new Response(null, { status: 500 }));
    render(<Page />);
    await screen.findByText('This quote is unavailable.');
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    await screen.findByText('Refresh failed. Quote could not be loaded.');
    expect(screen.queryByText('Quote refreshed from saved data.')).toBeNull();
  });

  it('reads the real owned envelope with actual line, total and deposit amounts', async () => {
    fetcher.mockResolvedValueOnce(Response.json(detail()));
    await ready();
    expect(screen.getAllByText('$120.00')).toHaveLength(2);
    expect(screen.getByText('$40.00')).toBeVisible();
    expect(acceptButton()).toBeEnabled();
    expect(fetcher).toHaveBeenCalledWith(`/api/v1/quotes/${id}`, expect.objectContaining({ cache: 'no-store' }));
  });

  it('sends the original microsecond/offset version and verifies acceptance without checkout', async () => {
    fetcher.mockResolvedValueOnce(Response.json(detail()))
      .mockResolvedValueOnce(Response.json(receipt()))
      .mockResolvedValueOnce(successfulRead());
    await submit();
    expect(await screen.findByRole('heading', { name: 'Quote accepted' })).toBeVisible();
    expect(posts()).toHaveLength(1);
    expect(posts()[0]).toEqual([`/api/v1/quotes/${id}/accept`, expect.objectContaining({
      headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ expected_updated_at: version }),
    })]);
    expect(screen.getByText(`Invoice: ${invoiceId}`)).toBeVisible();
    expect(screen.getByText('Invoice status: Draft')).toBeVisible();
    expect(screen.getByText('Payment status: unverified')).toBeVisible();
    expect(screen.getByText(/Checkout is not configured/)).toBeVisible();
    expect(screen.getByText(/Site visit/)).toBeVisible();
    expect(screen.queryByText(/business has been notified|continue scheduling|deposit.*complete/i)).toBeNull();
    expect(screen.queryByRole('link', { name: /payment/i })).toBeNull();
  });

  it('requires the committed readback before showing success', async () => {
    const readback = deferred<Response>();
    fetcher.mockResolvedValueOnce(Response.json(detail()))
      .mockResolvedValueOnce(Response.json(receipt())).mockReturnValueOnce(readback.promise);
    await submit();
    await waitFor(() => expect(fetcher).toHaveBeenCalledTimes(3));
    expect(screen.queryByRole('heading', { name: 'Quote accepted' })).toBeNull();
    await act(async () => readback.resolve(successfulRead()));
    expect(await screen.findByRole('heading', { name: 'Quote accepted' })).toBeVisible();
  });

  it.each(['lost', 'malformed', 'http'])('reconciles a %s response using the owned committed receipt without reposting', async failure => {
    fetcher.mockResolvedValueOnce(Response.json(detail()));
    if (failure === 'lost') fetcher.mockRejectedValueOnce(new TypeError('Network lost'));
    else if (failure === 'malformed') fetcher.mockResolvedValueOnce(new Response('{', { status: 200 }));
    else fetcher.mockResolvedValueOnce(Response.json({ success: false }, { status: 503 }));
    fetcher.mockResolvedValueOnce(successfulRead());
    await submit();
    expect(await screen.findByRole('heading', { name: 'Quote accepted' })).toBeVisible();
    expect(posts()).toHaveLength(1);
  });

  it.each([409, 404, 503])('holds a %s acceptance failure until explicit refreshed review', async status => {
    fetcher.mockResolvedValueOnce(Response.json(detail()))
      .mockResolvedValueOnce(Response.json({ success: false, status: 'reconciliation' }, { status }))
      .mockResolvedValueOnce(Response.json(detail({ updated_at: updatedVersion })));
    await submit();
    expect(await screen.findByRole('alert')).toHaveTextContent(/could not be confirmed.*Refresh/i);
    expect(acceptButton()).toBeDisabled();
    fireEvent.click(acceptButton());
    expect(posts()).toHaveLength(1);
    fetcher.mockResolvedValueOnce(Response.json(detail({ updated_at: updatedVersion })))
      .mockResolvedValueOnce(Response.json(receipt())).mockResolvedValueOnce(successfulRead());
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    await waitFor(() => expect(acceptButton()).toBeEnabled());
    fireEvent.click(acceptButton());
    await screen.findByRole('heading', { name: 'Quote accepted' });
    expect(JSON.parse(posts()[1][1]!.body as string)).toEqual({ expected_updated_at: updatedVersion });
  });

  it('holds an unknown network outcome and failed readback without offering blind retry', async () => {
    fetcher.mockResolvedValueOnce(Response.json(detail()))
      .mockRejectedValueOnce(new TypeError('Connection lost')).mockRejectedValueOnce(new TypeError('Readback lost'));
    await submit();
    await screen.findByRole('alert');
    expect(acceptButton()).toBeDisabled();
    expect(posts()).toHaveLength(1);
    expect(screen.queryByRole('heading', { name: 'Quote accepted' })).toBeNull();
  });

  it.each([
    { success: false }, { status: 'pending' }, { quote_id: otherId }, { invoice_id: '' },
    { invoice_status: null }, { payment_status: null }, { checkout_status: 'invented' },
    { stripe_payment_link: 12 },
  ])('does not trust a malformed receipt %j, even in an ACCEPTED readback', async invalid => {
    fetcher.mockResolvedValueOnce(Response.json(detail()))
      .mockResolvedValueOnce(Response.json(receipt(invalid)))
      .mockResolvedValueOnce(Response.json(accepted(receipt(invalid))));
    await submit();
    await screen.findByRole('alert');
    expect(screen.queryByRole('heading', { name: 'Quote accepted' })).toBeNull();
    expect(screen.queryByRole('link', { name: /payment/i })).toBeNull();
  });

  it.each([{ total_amount_cents: 99999 }, { customer_id: 'different-customer' }])('does not confirm acceptance of different reviewed terms or customer %j', async changed => {
    fetcher.mockResolvedValueOnce(Response.json(detail()))
      .mockResolvedValueOnce(Response.json(receipt()))
      .mockResolvedValueOnce(Response.json(detail({ status: 'ACCEPTED', ...changed }, receipt())));
    await submit();
    await screen.findByRole('alert');
    expect(screen.queryByRole('heading', { name: 'Quote accepted' })).toBeNull();
  });

  it('rejects a valid POST receipt whose invoice disagrees with the owned readback', async () => {
    fetcher.mockResolvedValueOnce(Response.json(detail()))
      .mockResolvedValueOnce(Response.json(receipt({ invoice_id: otherId })))
      .mockResolvedValueOnce(successfulRead());
    await submit();
    await screen.findByRole('alert');
    expect(screen.queryByRole('heading', { name: 'Quote accepted' })).toBeNull();
  });

  it('never reports a receipt attached to a nonaccepted quote as accepted', async () => {
    fetcher.mockResolvedValueOnce(Response.json(detail({}, receipt())));
    await ready();
    expect(screen.queryByRole('heading', { name: 'Quote accepted' })).toBeNull();
    expect(screen.queryByRole('button', { name: 'Accept quote' })).toBeNull();
    expect(screen.getByText(/requires reconciliation/i)).toBeVisible();
  });

  it('restores the current accepted receipt on reload without posting', async () => {
    fetcher.mockResolvedValueOnce(successfulRead());
    await ready();
    expect(screen.getByRole('heading', { name: 'Quote accepted' })).toBeVisible();
    expect(posts()).toHaveLength(0);
  });

  it('labels historical accepted status without inventing a verified receipt', async () => {
    fetcher.mockResolvedValueOnce(Response.json(detail({ status: 'ACCEPTED' })));
    await ready();
    expect(screen.getByText(/Recorded status: ACCEPTED/)).toBeVisible();
    expect(screen.getByText(/requires reconciliation/i)).toBeVisible();
    expect(screen.queryByRole('heading', { name: 'Quote accepted' })).toBeNull();
    expect(screen.queryByRole('button', { name: 'Accept quote' })).toBeNull();
  });

  it('exposes only an explicit HTTPS continuation from the verified receipt', async () => {
    const value = receipt({ checkout_status: 'available', stripe_payment_link: 'https://checkout.example.test/session', reason: null });
    fetcher.mockResolvedValueOnce(Response.json(detail()))
      .mockResolvedValueOnce(Response.json(value)).mockResolvedValueOnce(Response.json(accepted(value)));
    const oldLocation = window.location.href;
    await submit();
    expect(await screen.findByRole('link', { name: 'Continue to payment' })).toHaveAttribute('href', value.stripe_payment_link);
    expect(screen.getByRole('link', { name: 'Continue to payment' })).toHaveAttribute('rel', 'noopener noreferrer');
    expect(window.location.href).toBe(oldLocation);
    expect(screen.queryByRole('button', { name: 'Approve & Pay Invoice' })).toBeNull();
  });

  it.each(['javascript:alert(1)', 'http://checkout.example.test/session', 'https://user:pass@checkout.example.test', '//checkout.example.test', 'not-a-url'])('does not expose unsafe checkout %s', async url => {
    fetcher.mockResolvedValueOnce(Response.json(accepted(receipt({ checkout_status: 'available', stripe_payment_link: url }))));
    await ready();
    expect(screen.queryByRole('link', { name: /payment/i })).toBeNull();
    expect(screen.getByText(/payment continuation is unavailable/i)).toBeVisible();
  });

  it.each([
    { payment_status: 'paid' }, { invoice_status: 'Paid' }, { payment_status: 'processing' },
    { payment_status: 'refunded' }, { invoice_status: 'Void' }, { payment_status: 'unknown' },
    { checkout_status: 'pending' }, { checkout_status: 'reconciliation' },
  ])('shows actual receipt states without another payment action: %j', async states => {
    fetcher.mockResolvedValueOnce(Response.json(accepted(receipt({ checkout_status: 'available',
      stripe_payment_link: 'https://checkout.example.test/session', ...states }))));
    await ready();
    expect(screen.getByRole('heading', { name: 'Quote accepted' })).toBeVisible();
    expect(screen.queryByRole('link', { name: /payment/i })).toBeNull();
    expect(screen.queryByRole('button', { name: 'Accept quote' })).toBeNull();
  });

  it.each([null, undefined])('keeps a record without version %s readable but does not offer acceptance', async updated_at => {
    fetcher.mockResolvedValueOnce(Response.json(detail({ updated_at })));
    await ready();
    expect(screen.getByText(/version is unavailable/i)).toBeVisible();
    expect(screen.queryByRole('button', { name: 'Accept quote' })).toBeNull();
  });

  it.each(['PENDING', 'DRAFTING', 'REJECTED', 'EXPIRED'])('does not accept ineligible %s terms', async status => {
    fetcher.mockResolvedValueOnce(Response.json(detail({ status })));
    await ready();
    expect(screen.queryByRole('button', { name: 'Accept quote' })).toBeNull();
  });

  it('does not display pending placeholder amounts as a zero-dollar quote', async () => {
    fetcher.mockResolvedValueOnce(Response.json(detail({ status: 'DRAFTING', total_amount_cents: 0, required_deposit_cents: 0 })));
    await ready();
    expect(screen.getAllByText('Not available')).toHaveLength(2);
    expect(screen.queryByText('$0.00')).toBeNull();
    expect(screen.queryByRole('button', { name: 'Accept quote' })).toBeNull();
  });

  it('keeps null total and deposit unavailable instead of zero', async () => {
    fetcher.mockResolvedValueOnce(Response.json(detail({ total_amount_cents: null, required_deposit_cents: null })));
    await ready();
    expect(screen.getAllByText('Not available')).toHaveLength(2);
    expect(screen.queryByText('$0.00')).toBeNull();
    expect(screen.queryByRole('button', { name: 'Accept quote' })).toBeNull();
  });

  it.each([-2147483649, 2147483648])('holds first acceptance outside the backend invoice range: %s cents', async cents => {
    fetcher.mockResolvedValueOnce(Response.json(detail({ total_amount_cents: cents })));
    await ready();
    expect(screen.getByText(/outside the supported invoice range/)).toBeVisible();
    expect(screen.queryByRole('button', { name: 'Accept quote' })).toBeNull();
    expect(screen.getByText(cents < 0 ? '-$21,474,836.49' : '$21,474,836.48')).toBeVisible();
  });

  it.each([-2147483648, 2147483647])('keeps supported signed invoice boundary eligible: %s cents', async cents => {
    fetcher.mockResolvedValueOnce(Response.json(detail({ total_amount_cents: cents })));
    await ready();
    expect(acceptButton()).toBeEnabled();
  });

  it('renders exact cents at the safe-integer boundary without dollar rounding', async () => {
    fetcher.mockResolvedValueOnce(Response.json(detail({ total_amount_cents: Number.MAX_SAFE_INTEGER,
      required_deposit_cents: -Number.MAX_SAFE_INTEGER }, null,
    [{ ...item, unit_price_cents: Number.MAX_SAFE_INTEGER }])));
    await ready();
    expect(screen.getAllByText('$90,071,992,547,409.91')).toHaveLength(2);
    expect(screen.getByText('-$90,071,992,547,409.91')).toBeVisible();
  });

  it('preserves signed amounts and optional lines without recomputing terms', async () => {
    fetcher.mockResolvedValueOnce(Response.json(detail({ total_amount_cents: -500, required_deposit_cents: -200 }, null,
      [{ ...item, unit_price_cents: -300, quantity: 2, is_optional: true }])));
    await ready();
    expect(screen.getByText('-$5.00')).toBeVisible();
    expect(screen.getByText('-$2.00')).toBeVisible();
    expect(screen.getByText('-$6.00')).toBeVisible();
    expect(screen.getByText('Optional')).toBeVisible();
  });

  it.each([
    detail({ id: otherId }), detail({}, null, [{ ...item, quote_id: otherId }]),
    detail({ total_amount_cents: '12000' }), detail({}, null, [item, item]), { quote: null },
  ])('fails closed on a mismatched or malformed quote envelope', async value => {
    fetcher.mockResolvedValueOnce(Response.json(value));
    render(<Page />);
    expect(await screen.findByRole('alert')).toHaveTextContent(/unavailable/i);
    expect(screen.queryByRole('button', { name: 'Accept quote' })).toBeNull();
  });

  it('shows a foreign 404 without details or acceptance', async () => {
    fetcher.mockResolvedValueOnce(new Response('', { status: 404 }));
    render(<Page />);
    expect(await screen.findByRole('alert')).toHaveTextContent(/not found/i);
    expect(screen.queryByText(/Site visit/)).toBeNull();
    expect(posts()).toHaveLength(0);
  });

  it.each(['', 'e2e-id', 'quote-7', '../other'])('rejects unsupported ID %s without an API call', async value => {
    route.id = value;
    render(<Page />);
    expect(await screen.findByRole('alert')).toHaveTextContent(/not found/i);
    expect(fetcher).not.toHaveBeenCalled();
  });

  it('guards duplicate same-tick clicks until readback resolves', async () => {
    const response = deferred<Response>();
    fetcher.mockResolvedValueOnce(Response.json(detail())).mockReturnValueOnce(response.promise)
      .mockResolvedValueOnce(successfulRead());
    await ready();
    const button = acceptButton();
    fireEvent.click(button);
    fireEvent.click(button);
    expect(posts()).toHaveLength(1);
    await act(async () => response.resolve(Response.json(receipt())));
    await screen.findByRole('heading', { name: 'Quote accepted' });
    expect(posts()).toHaveLength(1);
  });

  it('ignores the old GET after navigation, including transports that ignore abort', async () => {
    const old = deferred<Response>();
    fetcher.mockReturnValueOnce(old.promise).mockResolvedValueOnce(Response.json(detail({ id: otherId }, null,
      [{ ...item, quote_id: otherId, description: 'New quote service' }])));
    const view = render(<Page />);
    route.id = otherId;
    view.rerender(<Page />);
    await screen.findByText(/New quote service/);
    await act(async () => old.resolve(Response.json(detail())));
    expect(screen.queryByText(/Site visit/)).toBeNull();
    expect(acceptButton()).toBeEnabled();
  });

  it('ignores an old acceptance completion and does not request its readback after navigation', async () => {
    const old = deferred<Response>();
    fetcher.mockResolvedValueOnce(Response.json(detail())).mockReturnValueOnce(old.promise)
      .mockResolvedValueOnce(Response.json(detail({ id: otherId }, null,
        [{ ...item, quote_id: otherId, description: 'New quote service' }])));
    const view = await ready();
    fireEvent.click(acceptButton());
    route.id = otherId;
    view.rerender(<Page />);
    await screen.findByText(/New quote service/);
    await act(async () => old.resolve(Response.json(receipt())));
    expect(fetcher).toHaveBeenCalledTimes(3);
    expect(screen.queryByRole('heading', { name: 'Quote accepted' })).toBeNull();
    expect(acceptButton()).toBeEnabled();
  });

  it('ignores old receipt readback after navigation', async () => {
    const old = deferred<Response>();
    fetcher.mockResolvedValueOnce(Response.json(detail())).mockResolvedValueOnce(Response.json(receipt()))
      .mockReturnValueOnce(old.promise).mockResolvedValueOnce(Response.json(detail({ id: otherId }, null,
        [{ ...item, quote_id: otherId, description: 'New quote service' }])));
    const view = await ready();
    fireEvent.click(acceptButton());
    await waitFor(() => expect(fetcher).toHaveBeenCalledTimes(3));
    route.id = otherId;
    view.rerender(<Page />);
    await screen.findByText(/New quote service/);
    await act(async () => old.resolve(successfulRead()));
    expect(screen.queryByRole('heading', { name: 'Quote accepted' })).toBeNull();
    expect(screen.queryByText(/Site visit/)).toBeNull();
  });
});

it('marks the proposal query suspense fallback as pending before a quote read exists', async () => {
  const pending = deferred<void>();
  route.pendingSearchParams = pending.promise;
  fetcher.mockResolvedValueOnce(Response.json(detail()));
  render(<CustomerProposalView />);
  expect(screen.getByRole('status')).toHaveAttribute('aria-busy', 'true');
  expect(fetcher).not.toHaveBeenCalled();
  await act(async () => {
    route.pendingSearchParams = null;
    pending.resolve();
  });
  await screen.findByText('Site visit x1');
  expect(document.querySelector('[aria-busy="true"]')).toBeNull();
});
