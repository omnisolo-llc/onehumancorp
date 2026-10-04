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
const otherId = '22222222-2222-4222-8222-222222222222';
const item = { id: 'line-1', quote_id: id, description: 'Reviewed service', unit_price_cents: 10000, quantity: 1, is_optional: false, service_item_id: 'service-1' };
function detail(quote: Record<string, unknown> = {}, lineItems = [item]) {
  return { quote: { id, customer_id: 'owned-customer', status: 'DRAFT', total_amount_cents: 10000, required_deposit_cents: 2500, stripe_payment_link: null, ...quote }, line_items: lineItems, acceptance: null };
}
function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>(r => { resolve = r; });
  return { promise, resolve };
}
const fetcher = vi.fn<typeof fetch>();
const back = vi.fn();

beforeEach(() => {
  vi.clearAllMocks();
  vi.mocked(useParams).mockReturnValue({ id });
  vi.mocked(useRouter).mockReturnValue({ back } as unknown as ReturnType<typeof useRouter>);
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
  it.each(['e2e-id', 'visual-audit-id', '../other'])('rejects unsupported ID %s without requesting or inventing a quote', async invalid => {
    vi.mocked(useParams).mockReturnValue({ id: invalid });
    render(<QuoteReviewPage />);
    await screen.findByText('Quote not found');
    expect(fetcher).not.toHaveBeenCalled();
    expect(screen.queryByText(/Sink Repair/)).not.toBeInTheDocument();
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

  it('keeps loading distinct from missing and offers no approval before data arrives', async () => {
    const pending = deferred<Response>();
    fetcher.mockReturnValue(pending.promise);
    render(<QuoteReviewPage />);
    expect(screen.getByText('Loading...')).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /Approve/ })).not.toBeInTheDocument();
    await act(async () => pending.resolve(Response.json(detail())));
    await screen.findByText('Reviewed service (x1)');
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
    fetcher.mockReturnValueOnce(approval.promise).mockResolvedValueOnce(Response.json(detail({ status: 'SENT' })));
    const button = screen.getByRole('button', { name: /^Approve quote$/ });
    fireEvent.click(button);
    fireEvent.click(button);
    expect(fetcher).toHaveBeenCalledTimes(2);
    expect(fetcher).toHaveBeenLastCalledWith(`/api/v1/quotes/${id}/approve`, expect.objectContaining({ method: 'PATCH' }));
    await act(async () => approval.resolve(Response.json({ quote: detail({ status: 'SENT' }).quote })));
    await screen.findByText('SENT');
    expect(screen.getByText(/Message delivery to the customer is not confirmed/)).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /^Approve quote$/ })).not.toBeInTheDocument();
  });

  it('does not confirm approval when readback contains unreviewed changed terms', async () => {
    await ready();
    fetcher.mockResolvedValueOnce(Response.json({ quote: detail({ status: 'SENT' }).quote }))
      .mockResolvedValueOnce(Response.json(detail({ status: 'SENT', total_amount_cents: 50000 })));
    fireEvent.click(screen.getByRole('button', { name: /^Approve quote$/ }));
    await screen.findByText(/Approval could not be confirmed/);
    expect(screen.queryByText('Quote approval saved. Message delivery to the customer is not confirmed.')).not.toBeInTheDocument();
  });

  it.each(['http', 'network', 'recheck'])('does not invent approval after a %s failure and requires reconciliation before retry', async failure => {
    await ready();
    if (failure === 'http') fetcher.mockResolvedValueOnce(new Response(null, { status: 500 }));
    else if (failure === 'network') fetcher.mockRejectedValueOnce(new Error('offline'));
    else fetcher.mockResolvedValueOnce(Response.json({ quote: detail({ status: 'SENT' }).quote })).mockRejectedValueOnce(new Error('read failed'));
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
    fetcher.mockResolvedValueOnce(Response.json({ success: true })).mockResolvedValueOnce(Response.json(detail({ total_amount_cents: 11500 }, [{ ...optionalItem, id: 'saved-discount' }, { ...item, id: 'saved-line', unit_price_cents: 12500 }])));
    fireEvent.click(screen.getByRole('button', { name: 'Save Changes' }));
    await screen.findByText('Quote changes saved.');
    const [, options] = fetcher.mock.calls.find(([, options]) => options?.method === 'PUT')!;
    expect(JSON.parse(options!.body as string)).toMatchObject({ total_amount_cents: 11500, required_deposit_cents: 2500, line_items: [expect.objectContaining({ unit_price_cents: 12500, service_item_id: 'service-1', is_optional: false }), expect.objectContaining({ unit_price_cents: -1000, is_optional: true })] });
    expect(screen.getByText('$115.00')).toBeInTheDocument();
  });

  it('keeps edits unconfirmed when readback differs from the submitted terms', async () => {
    await ready();
    fireEvent.click(screen.getByRole('button', { name: 'Edit quote' }));
    fireEvent.change(screen.getByLabelText('Total amount'), { target: { value: '120' } });
    fetcher.mockResolvedValueOnce(Response.json({ success: true })).mockResolvedValueOnce(Response.json(detail()));
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
    await act(async () => mutation.resolve(Response.json({ quote: detail({ status: 'SENT' }).quote })));
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
    expect(back).toHaveBeenCalledOnce();
  });
});
