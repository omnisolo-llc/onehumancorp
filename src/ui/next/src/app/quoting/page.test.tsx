import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
const mock = vi.hoisted(() => ({ id: '11111111-1111-4111-8111-111111111111', enqueue: vi.fn() }));
vi.mock('next/navigation', () => ({ useSearchParams: () => ({ get: () => mock.id }) }));
vi.mock('../../lib/sync/SyncManager', () => ({ SyncManager: { getInstance: () => ({ enqueue: mock.enqueue }) } }));
import Page from './page';
const quote = (id = mock.id, status = 'DRAFT') => ({ quote: { id, status, total_amount_cents: 2500 }, line_items: [{ id: 'line-a', description: 'Reviewed work', quantity: 1, unit_price_cents: 2500, is_optional: false }] });
const deferred = <T,>() => { let resolve!: (value: T) => void; const promise = new Promise<T>(r => { resolve = r; }); return { promise, resolve }; };
let transport: ReturnType<typeof vi.fn>;
function setup(mutate: (url: string, init: RequestInit) => Promise<Response> = async (url) => Response.json(url.endsWith('/approve') ? { quote: { id: mock.id, status: 'SENT' } } : { success: true })) {
  transport = vi.fn(async (url: string, init: RequestInit = {}) => init.method ? mutate(url, init) : Response.json(quote()));
  vi.stubGlobal('fetch', transport);
}
beforeEach(() => { mock.id = '11111111-1111-4111-8111-111111111111'; mock.enqueue.mockReset().mockResolvedValue(undefined); vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true); vi.stubGlobal('alert', vi.fn()); });
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
const clickApprove = async () => { const button = await screen.findByTestId('quote-approve-btn'); fireEvent.click(button); return button; };
describe('quote owner approval acknowledgement', () => {
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
  it('keeps an unknown second write separate from the acknowledged changes', async () => {
    setup(async url => Response.json(url.endsWith('/approve') ? { error: 'unknown' } : { success: true }, { status: url.endsWith('/approve') ? 503 : 200 }));
    render(<Page />); await clickApprove();
    expect(await screen.findByRole('alert')).toHaveTextContent(/changes.*saved/i);
    expect(screen.getByRole('alert')).toHaveTextContent(/approval.*not.*confirm/i);
    expect(screen.queryByText('Proposal Accepted')).toBeNull(); expect(mock.enqueue).not.toHaveBeenCalled();
  });
  it('holds a two-step approval offline without queuing either independent operation', async () => {
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    setup(); render(<Page />); await clickApprove();
    expect(await screen.findByRole('alert')).toHaveTextContent(/connection.*approval/i);
    expect(screen.getByRole('alert')).toHaveTextContent(/no changes.*saved or queued/i);
    expect(mock.enqueue).not.toHaveBeenCalled();
    expect(transport.mock.calls.filter(([, init]) => init?.method)).toHaveLength(0);
    expect(screen.getByTestId('quote-item-price-line-a')).not.toBeDisabled();
  });
  it('retains offline edits and requires a deliberate online retry before sending', async () => {
    const connection = vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    setup(); render(<Page />);
    const price = await screen.findByTestId('quote-item-price-line-a');
    fireEvent.change(price, { target: { value: '31.25' } }); await clickApprove();
    expect(await screen.findByRole('alert')).toHaveTextContent(/no changes.*saved or queued/i);
    expect(price).toHaveValue(31.25); expect(mock.enqueue).not.toHaveBeenCalled();
    connection.mockReturnValue(true); fireEvent(window, new Event('online'));
    expect(transport.mock.calls.filter(([, init]) => init?.method)).toHaveLength(0);
    await clickApprove();
    expect(await screen.findByRole('status')).toHaveTextContent(/approval saved/i);
    const update = transport.mock.calls.find(([, init]) => init?.method === 'POST');
    expect(JSON.parse(String(update?.[1]?.body)).line_items[0].unit_price_cents).toBe(3125);
  });
  it('does not approve a different quote after navigation during a save', async () => {
    const pending = deferred<Response>(); setup(async () => pending.promise); const view = render(<Page />); await clickApprove();
    mock.id = '22222222-2222-4222-8222-222222222222'; view.rerender(<Page />); await waitFor(() => expect(transport.mock.calls.some(([url]) => url.includes('22222222-2222-4222-8222-222222222222'))).toBe(true));
    await act(async () => pending.resolve(Response.json({ success: true })));
    expect(transport.mock.calls.filter(([url]) => url.endsWith('/approve'))).toHaveLength(0);
    expect(screen.queryByText('Proposal Accepted')).toBeNull();
  });
});
