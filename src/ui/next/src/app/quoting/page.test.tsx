import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
const mock = vi.hoisted(() => ({ id: '11111111-1111-4111-8111-111111111111', enqueue: vi.fn() }));
vi.mock('next/navigation', () => ({ useSearchParams: () => ({ get: () => mock.id }) }));
vi.mock('../../lib/sync/SyncManager', () => ({ SyncManager: { getInstance: () => ({ enqueue: mock.enqueue }) } }));
import Page from './page';
const version = '2026-10-04T00:00:00.123456+00:00';
const committedVersion = '2026-10-04T00:00:00.123457Z';
const quote = (id = mock.id, status = 'DRAFT') => ({ quote: { id, status, updated_at: version, total_amount_cents: 2500, required_deposit_cents: 500 }, line_items: [{ id: 'line-a', description: 'Reviewed work', quantity: 1, unit_price_cents: 2500, is_optional: false }] });
const deferred = <T,>() => { let resolve!: (value: T) => void; const promise = new Promise<T>(r => { resolve = r; }); return { promise, resolve }; };
let transport: ReturnType<typeof vi.fn>;
function setup(mutate: (url: string, init: RequestInit) => Promise<Response> = async (url) => Response.json(url.endsWith('/approve') ? { quote: { id: mock.id, status: 'SENT', updated_at: '2026-10-04T00:00:00.123458Z' } } : { success: true, updated_at: committedVersion })) {
  transport = vi.fn(async (url: string, init: RequestInit = {}) => init.method ? mutate(url, init) : Response.json(quote()));
  vi.stubGlobal('fetch', transport);
}
beforeEach(() => { mock.id = '11111111-1111-4111-8111-111111111111'; mock.enqueue.mockReset().mockResolvedValue(undefined); vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true); vi.stubGlobal('alert', vi.fn()); });
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
const clickApprove = async () => { const button = await screen.findByTestId('quote-approve-btn'); fireEvent.click(button); return button; };
describe('quote owner approval acknowledgement', () => {
  it('associates each editable quantity and price with its visible line-item label', async () => {
    setup();
    const detail = quote();
    detail.line_items.push({ ...detail.line_items[0], id: 'line-b', description: 'Additional work' });
    transport.mockResolvedValueOnce(Response.json(detail));
    render(<Page />);
    await screen.findByText('Additional work');
    const controls = ['Reviewed work', 'Additional work'].flatMap(description => [
      screen.getByRole('spinbutton', { name: `Qty for ${description}` }),
      screen.getByRole('spinbutton', { name: `Price ($) for ${description}` }),
    ]);
    expect(new Set(controls.map(control => control.id)).size).toBe(4);
    for (const control of controls) {
      expect(control).toBeEnabled();
      expect((control as HTMLInputElement).labels).toHaveLength(1);
      expect((control as HTMLInputElement).labels![0].htmlFor).toBe(control.id);
    }
  });
  it('does not claim customer acceptance while either write is pending and prevents duplicate clicks', async () => {
    const pending = deferred<Response>(); setup(async () => pending.promise); render(<Page />);
    const button = await clickApprove(); fireEvent.click(button);
    expect(screen.queryByText('Proposal Accepted')).toBeNull();
    expect(button).toBeDisabled();
    expect(screen.getByTestId('quote-item-quantity-line-a')).toBeDisabled();
    expect(transport.mock.calls.filter(([, init]) => init?.method)).toHaveLength(1);
    await act(async () => pending.resolve(Response.json({ success: false }, { status: 500 })));
  });
  it.each([500, 200])('never invents offline persistence when an online update fails (%s)', async status => {
    setup(async () => Response.json({ success: false }, { status })); render(<Page />); await clickApprove();
    await waitFor(() => expect(screen.getByRole('alert')).toHaveTextContent(/not confirm/i));
    expect(screen.queryByText('Proposal Accepted')).toBeNull(); expect(mock.enqueue).not.toHaveBeenCalled(); expect(alert).not.toHaveBeenCalled();
    expect(transport.mock.calls.filter(([, init]) => init?.method)).toHaveLength(1);
  });
  it('reports the actual SENT result without claiming customer acceptance or delivery', async () => {
    setup(); render(<Page />); await clickApprove();
    expect(await screen.findByRole('status')).toHaveTextContent(/approval saved/i);
    expect(screen.getByRole('status')).toHaveTextContent(/delivery.*not confirmed/i);
    expect(screen.queryByText('Proposal Accepted')).toBeNull(); expect(screen.getByText('SENT', { exact: true })).toBeVisible();
  });
  it('sends exact reviewed and committed versions through the two-step approval', async () => {
    setup(); render(<Page />); await clickApprove();
    await screen.findByRole('status');
    const update = transport.mock.calls.find(([, init]) => init?.method === 'POST');
    const approval = transport.mock.calls.find(([, init]) => init?.method === 'PATCH');
    expect(JSON.parse(update![1].body)).toMatchObject({ expected_updated_at: version });
    expect(JSON.parse(approval![1].body)).toEqual({ expected_updated_at: committedVersion });
  });
  it('holds approval when the update receipt omits the final committed version', async () => {
    setup(async () => Response.json({ success: true })); render(<Page />); await clickApprove();
    expect(await screen.findByRole('alert')).toHaveTextContent(/approval.*not.*confirm/i);
    expect(transport.mock.calls.some(([url]) => url.endsWith('/approve'))).toBe(false);
  });
  it('cannot submit a versionless quote even through a dispatched click', async () => {
    setup(); transport.mockResolvedValueOnce(Response.json({ ...quote(), quote: { ...quote().quote, updated_at: null } }));
    render(<Page />);
    const button = await screen.findByTestId('quote-approve-btn');
    expect(button).toBeDisabled(); fireEvent.click(button);
    expect(transport.mock.calls.filter(([, init]) => init?.method)).toHaveLength(0);
  });
  it.each(['DRAFTING', 'PENDING', 'DECLINED'])('cannot write prices before rejecting an ineligible %s approval', async status => {
    setup(); transport.mockResolvedValueOnce(Response.json(quote(mock.id, status))); render(<Page />);
    const button = await screen.findByTestId('quote-approve-btn');
    expect(button).toBeDisabled(); fireEvent.click(button);
    expect(transport.mock.calls.filter(([, init]) => init?.method)).toHaveLength(0);
  });
  it.each(['total', 'deposit', 'lines'])('cannot approve incomplete persisted %s terms', async missing => {
    setup(); const incomplete = quote();
    if (missing === 'total') incomplete.quote.total_amount_cents = null as unknown as number;
    else if (missing === 'deposit') incomplete.quote.required_deposit_cents = null as unknown as number;
    else incomplete.line_items = [];
    transport.mockResolvedValueOnce(Response.json(incomplete)); render(<Page />);
    const button = await screen.findByTestId('quote-approve-btn');
    expect(button).toBeDisabled(); fireEvent.click(button);
    expect(transport.mock.calls.filter(([, init]) => init?.method)).toHaveLength(0);
  });
  it('keeps an unknown second write separate from the acknowledged changes', async () => {
    setup(async url => Response.json(url.endsWith('/approve') ? { error: 'unknown' } : { success: true, updated_at: committedVersion }, { status: url.endsWith('/approve') ? 503 : 200 }));
    render(<Page />); await clickApprove();
    expect(await screen.findByRole('alert')).toHaveTextContent(/changes.*saved/i);
    expect(screen.getByRole('alert')).toHaveTextContent(/approval.*not.*confirm/i);
    expect(screen.queryByText('Proposal Accepted')).toBeNull(); expect(mock.enqueue).not.toHaveBeenCalled();
  });
  it('retains offline edits and processes optimistic queuing', async () => {
    const connection = vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    setup(); render(<Page />);

    const price = await screen.findByTestId('quote-item-price-line-a');
    fireEvent.change(price, { target: { value: '31.25' } });

    act(() => {
      Object.defineProperty(navigator, 'onLine', { configurable: true, get: () => false });
      window.dispatchEvent(new Event('offline'));
    });

    const button = await screen.findByTestId('quote-approve-btn');
    fireEvent.click(button);
    await waitFor(() => expect(mock.enqueue).toHaveBeenCalledTimes(2));
    expect(await screen.findByText('Proposal Accepted')).toBeVisible();

    // Changing network status to true will not re-trigger the API call as it's handled via background queue
    connection.mockReturnValue(true);
    act(() => {
      window.dispatchEvent(new Event('online'));
    });
    expect(transport.mock.calls.filter(([, init]) => init?.method)).toHaveLength(0);
  });
  it('does not approve a different quote after navigation during a save', async () => {
    const pending = deferred<Response>(); setup(async () => pending.promise); const view = render(<Page />); await clickApprove();
    mock.id = '22222222-2222-4222-8222-222222222222'; view.rerender(<Page />); await waitFor(() => expect(transport.mock.calls.some(([url]) => url.includes('22222222-2222-4222-8222-222222222222'))).toBe(true));
    await act(async () => pending.resolve(Response.json({ success: true, updated_at: committedVersion })));
    expect(transport.mock.calls.filter(([url]) => url.endsWith('/approve'))).toHaveLength(0);
    expect(screen.queryByText('Proposal Accepted')).toBeNull();
  });
});
