import React from 'react';
import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useParams, useRouter } from 'next/navigation';
import QuoteReviewPage from './page';

vi.mock('next/navigation', () => ({ useParams: vi.fn(), useRouter: vi.fn() }));
vi.mock('../../components/AppShell', () => ({
  AppShell: ({ title, children }: { title: string; children: React.ReactNode }) => <main><h1>{title}</h1>{children}</main>,
}));

const id = '11111111-1111-4111-8111-111111111111';
const version = '2026-10-04T00:00:00.123456Z';
const savedVersion = '2026-10-04T00:00:00.123457Z';
const otherId = '22222222-2222-4222-8222-222222222222';
const item = { id: 'line-1', quote_id: id, description: 'Reviewed service', unit_price_cents: 10000, quantity: 1, is_optional: false, service_item_id: 'service-1' };
function detail(quote: Record<string, unknown> = {}, lineItems = [item]) {
  return { quote: { id, customer_id: 'owned-customer', status: 'DRAFT', total_amount_cents: 10000, required_deposit_cents: 2500, stripe_payment_link: null, updated_at: version, ...quote }, line_items: lineItems, acceptance: null };
}
function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>(r => { resolve = r; });
  return { promise, resolve };
}
function pendingErrorResponse(status: number) {
  let controller!: ReadableStreamDefaultController<Uint8Array>;
  const stream = new ReadableStream<Uint8Array>({ start(value) { controller = value; } });
  const response = new Response(stream, { status, headers: { 'content-type': 'application/json' } });
  controller.enqueue(new TextEncoder().encode('{"error":'));
  return { response, complete() { controller.enqueue(new TextEncoder().encode('"review required"}')); controller.close(); } };
}
const fetcher = vi.fn<typeof fetch>();
const back = vi.fn();
const push = vi.fn();

beforeEach(() => {
  vi.clearAllMocks();
  vi.mocked(useParams).mockReturnValue({ id });
  vi.mocked(useRouter).mockReturnValue({ back, push } as unknown as ReturnType<typeof useRouter>);
  vi.stubGlobal('fetch', fetcher);
  fetcher.mockReset();
  fetcher.mockImplementation(async () => Response.json(detail()));
});

async function ready() {
  const view = render(<QuoteReviewPage />);
  await screen.findByText('Reviewed service (x1)');
  return view;
}

describe('Quote detail truthfulness', () => {
  it.each([404, 500])('finishes the HTTP %s read body before exposing its error state', async status => {
    const pending = pendingErrorResponse(status);
    fetcher.mockResolvedValueOnce(pending.response);
    render(<QuoteReviewPage />);
    await act(async () => { await Promise.resolve(); });
    expect(pending.response.bodyUsed).toBe(true);
    expect(screen.getByText('Loading...')).toBeVisible();
    await act(async () => pending.complete());
    await screen.findByText(status === 404 ? 'Quote not found' : /Unable to load quote/);
  });

  it('finishes a rejected approval body before reporting unconfirmed approval', async () => {
    await ready();
    const pending = pendingErrorResponse(409);
    fetcher.mockResolvedValueOnce(pending.response);
    fireEvent.click(screen.getByRole('button', { name: /^Approve quote$/ }));
    await act(async () => { await Promise.resolve(); });
    expect(pending.response.bodyUsed).toBe(true);
    expect(screen.queryByText(/Approval could not be confirmed/)).toBeNull();
    await act(async () => pending.complete());
    await screen.findByText(/Approval could not be confirmed/);
    expect(screen.getByText('DRAFT')).toBeVisible();
  });

  it.each(['e2e-id', 'visual-audit-id', '../other'])('rejects unsupported ID %s without requesting or inventing a quote', async invalid => {
    vi.mocked(useParams).mockReturnValue({ id: invalid });
    render(<QuoteReviewPage />);
    await screen.findByText('Quote not found');
    expect(fetcher).not.toHaveBeenCalled();
    expect(screen.queryByText(/Sink Repair/)).not.toBeInTheDocument();
  });

  it('explains why an invalid reference cannot be refreshed', async () => {
    vi.mocked(useParams).mockReturnValue({ id: 'e2e-id' });
    render(<QuoteReviewPage />);
    await screen.findByText('Quote not found');
    expect(screen.getByText('The quote reference is invalid. Open a valid quote link.')).toBeVisible();
    expect(screen.getByRole('button', { name: 'Refresh quote' })).toBeDisabled();
    expect(fetcher).not.toHaveBeenCalled();
  });

  it('reports a completed refresh when an owned lookup still returns missing', async () => {
    fetcher.mockImplementation(async () => new Response(null, { status: 404 }));
    render(<QuoteReviewPage />);
    await screen.findByText('Quote not found');
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    await screen.findByText('Refresh complete. Quote not found.');
    expect(fetcher).toHaveBeenCalledTimes(2);
    expect(screen.queryByRole('button', { name: /Approve/ })).not.toBeInTheDocument();
    const repeated = deferred<Response>();
    fetcher.mockReturnValueOnce(repeated.promise);
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    expect(screen.queryByText('Refresh complete. Quote not found.')).not.toBeInTheDocument();
    await act(async () => repeated.resolve(new Response(null, { status: 404 })));
    await screen.findByText('Refresh complete. Quote not found.');
    expect(fetcher).toHaveBeenCalledTimes(3);
  });

  it('reports a refreshed unchanged real quote without claiming a mutation', async () => {
    await ready();
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    await screen.findByText('Quote refreshed from saved data.');
    expect(screen.getByText('Reviewed service (x1)')).toBeVisible();
    expect(fetcher.mock.calls.every(([, request]) => !request?.method)).toBe(true);
  });

  it('keeps delivery unconfirmed after refreshing a recorded SENT status', async () => {
    fetcher.mockImplementation(async () => Response.json(detail({ status: 'SENT' })));
    await ready();
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    await screen.findByText('Quote refreshed from saved data.');
    expect(screen.getByText(/Message delivery to the customer is not confirmed/)).toBeVisible();
  });

  it('reports a failed refresh without inventing a completed read', async () => {
    fetcher.mockImplementation(async () => new Response(null, { status: 500 }));
    render(<QuoteReviewPage />);
    await screen.findByText(/Unable to load quote/);
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    await screen.findByText('Refresh failed. Quote could not be loaded.');
    expect(screen.queryByText('Quote refreshed from saved data.')).not.toBeInTheDocument();
  });

  it('reads the persisted envelope without changing its amounts', async () => {
    await ready();
    expect(screen.getAllByText('$100.00')).toHaveLength(2);
    expect(screen.getByText('$25.00')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /^Approve quote$/ })).toBeEnabled();
  });

  it('accepts persisted version-seven UUIDs supported by the backend', async () => {
    const uuid = '01982bbb-1234-7123-8123-123456789abc';
    vi.mocked(useParams).mockReturnValue({ id: uuid });
    fetcher.mockResolvedValue(Response.json(detail({ id: uuid }, [{ ...item, quote_id: uuid }])));
    render(<QuoteReviewPage />);
    await screen.findByText('Reviewed service (x1)');
  });

  it('keeps a versionless record readable but holds owner mutations', async () => {
    fetcher.mockResolvedValue(Response.json(detail({ updated_at: null })));
    await ready();
    expect(screen.getByText(/Quote version is unavailable/)).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /Approve|Edit/i })).not.toBeInTheDocument();
  });

  it('uses the post-save committed version for the next approval', async () => {
    await ready();
    fireEvent.click(screen.getByRole('button', { name: 'Edit quote' }));
    fetcher.mockResolvedValueOnce(Response.json({ success: true, updated_at: savedVersion }))
      .mockResolvedValueOnce(Response.json(detail({ updated_at: savedVersion })));
    fireEvent.click(screen.getByRole('button', { name: 'Save Changes' }));
    await screen.findByText('Quote changes saved.');
    const approved = detail({ status: 'SENT', updated_at: '2026-10-04T00:00:00.123458Z' });
    fetcher.mockResolvedValueOnce(Response.json({ quote: approved.quote })).mockResolvedValueOnce(Response.json(approved));
    fireEvent.click(screen.getByRole('button', { name: /^Approve quote$/ }));
    await screen.findByText('SENT');
    const [, request] = fetcher.mock.calls.find(([, request]) => request?.method === 'PATCH')!;
    expect(JSON.parse(request!.body as string)).toEqual({ expected_updated_at: savedVersion });
  });

  it('keeps loading distinct from missing and offers no approval before data arrives', async () => {
    const pending = deferred<Response>();
    fetcher.mockReturnValue(pending.promise);
    render(<QuoteReviewPage />);
    expect(screen.getByRole('status')).toHaveTextContent('Loading...');
    expect(screen.getByRole('status')).toHaveAttribute('aria-busy', 'true');
    expect(screen.queryByRole('button', { name: /Approve/ })).not.toBeInTheDocument();
    await act(async () => pending.resolve(Response.json(detail())));
    await screen.findByText('Reviewed service (x1)');
    expect(document.querySelector('[aria-busy="true"]')).toBeNull();
  });

  it.each([404, 500, 403])('never fabricates fixture data after HTTP %s', async status => {
    vi.mocked(useParams).mockReturnValue({ id: '823e4567-e89b-12d3-a456-426614174000' });
    fetcher.mockResolvedValue(new Response(null, { status }));
    render(<QuoteReviewPage />);
    await screen.findByText(status === 404 ? 'Quote not found' : /Unable to load quote/);
    expect(screen.queryByText(/Sink Repair|\$350.00|cust-e2e/)).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /Approve/ })).not.toBeInTheDocument();
  });

  it('allows a failed read to be retried', async () => {
    fetcher.mockRejectedValueOnce(new Error('offline'));
    render(<QuoteReviewPage />);
    await screen.findByText(/Unable to load quote/);
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    await screen.findByText('Reviewed service (x1)');
  });

  it.each([{}, detail({ id: otherId }), detail({ total_amount_cents: Number.MAX_SAFE_INTEGER + 1 })])('rejects malformed or mismatched successful responses', async body => {
    fetcher.mockResolvedValue(Response.json(body));
    render(<QuoteReviewPage />);
    await screen.findByText(/Unable to load quote/);
    expect(screen.queryByRole('button', { name: /Approve/ })).not.toBeInTheDocument();
  });

  it('shows a drafting record as pending rather than a zero-dollar ready quote', async () => {
    fetcher.mockResolvedValue(Response.json(detail({ status: 'DRAFTING', total_amount_cents: 0, required_deposit_cents: 0 }, [])));
    render(<QuoteReviewPage />);
    await screen.findByText(/Quote preparation is pending/);
    expect(screen.queryByText('$0.00')).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /Approve|Edit/i })).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    expect(fetcher).toHaveBeenCalledTimes(2);
  });

  it.each([detail({ total_amount_cents: null }), detail({}, [])])('keeps incomplete persisted terms unavailable for approval', async body => {
    fetcher.mockResolvedValue(Response.json(body));
    render(<QuoteReviewPage />);
    await screen.findByText(/Quote terms are incomplete/);
    expect(screen.queryByRole('button', { name: /Approve/ })).not.toBeInTheDocument();
  });

  it('ignores a stale response after navigation', async () => {
    const first = deferred<Response>();
    fetcher.mockReturnValueOnce(first.promise).mockResolvedValueOnce(new Response(null, { status: 404 }));
    const view = render(<QuoteReviewPage />);
    vi.mocked(useParams).mockReturnValue({ id: otherId });
    view.rerender(<QuoteReviewPage />);
    await screen.findByText('Quote not found');
    await act(async () => first.resolve(Response.json(detail())));
    expect(screen.getByText('Quote not found')).toBeInTheDocument();
    expect(screen.queryByText('Reviewed service (x1)')).not.toBeInTheDocument();
  });

  it('approves once through the real approval route then reads the saved status without claiming delivery', async () => {
    await ready();
    const approval = deferred<Response>();
    fetcher.mockReturnValueOnce(approval.promise).mockResolvedValueOnce(Response.json(detail({ status: 'SENT', updated_at: savedVersion })));
    const button = screen.getByRole('button', { name: /^Approve quote$/ });
    fireEvent.click(button);
    fireEvent.click(button);
    expect(fetcher).toHaveBeenCalledTimes(2);
    expect(fetcher).toHaveBeenLastCalledWith(`/api/v1/quotes/${id}/approve`, expect.objectContaining({ method: 'PATCH', body: JSON.stringify({ expected_updated_at: version }) }));
    await act(async () => approval.resolve(Response.json({ quote: detail({ status: 'SENT', updated_at: savedVersion }).quote })));
    await screen.findByText('SENT');
    expect(screen.getByText(/Message delivery to the customer is not confirmed/)).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /^Approve quote$/ })).not.toBeInTheDocument();
  });

  it('does not confirm approval when readback contains unreviewed changed terms', async () => {
    await ready();
    fetcher.mockResolvedValueOnce(Response.json({ quote: detail({ status: 'SENT', updated_at: savedVersion }).quote }))
      .mockResolvedValueOnce(Response.json(detail({ status: 'SENT', total_amount_cents: 50000 })));
    fireEvent.click(screen.getByRole('button', { name: /^Approve quote$/ }));
    await screen.findByText(/Approval could not be confirmed/);
    expect(screen.queryByText('Quote approval saved. Message delivery to the customer is not confirmed.')).not.toBeInTheDocument();
  });

  it.each(['http', 'network', 'recheck'])('does not invent approval after a %s failure and requires reconciliation before retry', async failure => {
    await ready();
    if (failure === 'http') fetcher.mockResolvedValueOnce(new Response(null, { status: 500 }));
    else if (failure === 'network') fetcher.mockRejectedValueOnce(new Error('offline'));
    else fetcher.mockResolvedValueOnce(Response.json({ quote: detail({ status: 'SENT', updated_at: savedVersion }).quote })).mockRejectedValueOnce(new Error('read failed'));
    fireEvent.click(screen.getByRole('button', { name: /^Approve quote$/ }));
    await screen.findByText(/Approval could not be confirmed/);
    expect(screen.getByText('DRAFT')).toBeInTheDocument();
    expect(screen.queryByText('SENT')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: /^Approve quote$/ })).toBeDisabled();
    fireEvent.click(screen.getByRole('button', { name: 'Refresh quote' }));
    await waitFor(() => expect(screen.getByRole('button', { name: /^Approve quote$/ })).toBeEnabled());
  });

  it('edits explicit terms without inventing a deposit or losing optional/catalog attributes and confirms saved state', async () => {
    const optionalItem = { ...item, description: 'Optional adjustment', is_optional: true, unit_price_cents: -1000 };
    fetcher.mockResolvedValueOnce(Response.json(detail({}, [item, { ...optionalItem, id: 'discount' }])));
    await ready();
    fireEvent.click(screen.getByRole('button', { name: 'Edit quote' }));
    fireEvent.change(screen.getByLabelText('Unit price for Reviewed service', { selector: '#quote-price-0' }), { target: { value: '125.00' } });
    expect(screen.getByLabelText('Required deposit')).toHaveValue(25);
    expect(screen.getByLabelText('Total amount')).toHaveValue(100);
    fireEvent.change(screen.getByLabelText('Total amount'), { target: { value: '115.00' } });
    fetcher.mockResolvedValueOnce(Response.json({ success: true, updated_at: savedVersion })).mockResolvedValueOnce(Response.json(detail({ total_amount_cents: 11500, updated_at: savedVersion }, [{ ...optionalItem, id: 'saved-discount' }, { ...item, id: 'saved-line', unit_price_cents: 12500 }])));
    fireEvent.click(screen.getByRole('button', { name: 'Save Changes' }));
    await screen.findByText('Quote changes saved.');
    const [, options] = fetcher.mock.calls.find(([, options]) => options?.method === 'PUT')!;
    expect(JSON.parse(options!.body as string)).toMatchObject({ total_amount_cents: 11500, required_deposit_cents: 2500, expected_updated_at: version, line_items: [expect.objectContaining({ unit_price_cents: 12500, service_item_id: 'service-1', is_optional: false }), expect.objectContaining({ unit_price_cents: -1000, is_optional: true })] });
    expect(screen.getByText('$115.00')).toBeInTheDocument();
  });

  it('keeps edits unconfirmed when readback differs from the submitted terms', async () => {
    await ready();
    fireEvent.click(screen.getByRole('button', { name: 'Edit quote' }));
    fireEvent.change(screen.getByLabelText('Total amount'), { target: { value: '120' } });
    fetcher.mockResolvedValueOnce(Response.json({ success: true, updated_at: savedVersion })).mockResolvedValueOnce(Response.json(detail()));
    fireEvent.click(screen.getByRole('button', { name: 'Save Changes' }));
    await screen.findByText(/Changes could not be confirmed/);
    expect(screen.queryByText('Quote changes saved.')).not.toBeInTheDocument();
    expect(screen.getByLabelText('Total amount')).toHaveValue(120);
    expect(screen.getByRole('button', { name: 'Save Changes' })).toBeDisabled();
  });

  it('ignores an approval receipt arriving after navigation and does not reread the old quote', async () => {
    const view = await ready();
    const mutation = deferred<Response>();
    fetcher.mockReturnValueOnce(mutation.promise).mockResolvedValueOnce(new Response(null, { status: 404 }));
    fireEvent.click(screen.getByRole('button', { name: /^Approve quote$/ }));
    vi.mocked(useParams).mockReturnValue({ id: otherId });
    view.rerender(<QuoteReviewPage />);
    await screen.findByText('Quote not found');
    await act(async () => mutation.resolve(Response.json({ quote: detail({ status: 'SENT', updated_at: savedVersion }).quote })));
    expect(fetcher).toHaveBeenCalledTimes(3);
    expect(screen.getByText('Quote not found')).toBeInTheDocument();
    expect(screen.queryByText('SENT')).not.toBeInTheDocument();
  });

  it('does not publish unsaved edits as persisted data after a rejected save', async () => {
    await ready();
    fireEvent.click(screen.getByRole('button', { name: 'Edit quote' }));
    fireEvent.change(screen.getByLabelText('Total amount'), { target: { value: '120' } });
    fetcher.mockResolvedValueOnce(new Response(null, { status: 409 }));
    fireEvent.click(screen.getByRole('button', { name: 'Save Changes' }));
    await screen.findByText(/Changes could not be confirmed/);
    expect(screen.getByRole('button', { name: 'Save Changes' })).toBeDisabled();
    expect(screen.getByText(/Unsaved changes/)).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Cancel edits' }));
    expect(screen.getAllByText('$100.00')).toHaveLength(2);
    expect(screen.queryByText('$120.00')).not.toBeInTheDocument();
  });

  it('rejects empty or non-cent amounts without sending', async () => {
    await ready();
    fireEvent.click(screen.getByRole('button', { name: 'Edit quote' }));
    fireEvent.change(screen.getByLabelText('Total amount'), { target: { value: '' } });
    fireEvent.click(screen.getByRole('button', { name: 'Save Changes' }));
    await screen.findByText(/Enter valid amounts/);
    expect(fetcher).toHaveBeenCalledTimes(1);
  });

  it('does not offer edit or approval for an accepted quote or turn an unsafe link into a payment action', async () => {
    fetcher.mockResolvedValue(Response.json(detail({ status: 'ACCEPTED', stripe_payment_link: 'javascript:alert(1)' })));
    render(<QuoteReviewPage />);
    await screen.findByText('ACCEPTED');
    expect(screen.queryByRole('button', { name: /Approve|Edit/i })).not.toBeInTheDocument();
    expect(screen.queryByRole('link')).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Back to Feed' }));
    expect(push).toHaveBeenCalledExactlyOnceWith('/feed');
    expect(back).not.toHaveBeenCalled();
  });
});

it('Back to Feed reaches its named destination from a direct quote link without browser history', async () => {
  await ready();
  fireEvent.click(screen.getByRole('button', { name: 'Back to Feed' }));
  expect(push).toHaveBeenCalledExactlyOnceWith('/feed');
  expect(back).not.toHaveBeenCalled();
});
