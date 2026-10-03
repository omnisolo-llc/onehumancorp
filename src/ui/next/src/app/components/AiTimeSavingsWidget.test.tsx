import { act, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, afterEach, expect, it, vi } from 'vitest';
import Widget from './AiTimeSavingsWidget';
const plan = vi.hoisted(() => ({ claimTrial: vi.fn().mockResolvedValue(false), claimError: null, verifiedOwner: { userId: 'a', tenantId: 'a' } as { userId: string; tenantId: string } | null }));
vi.mock('./useProPlan', () => ({ useProPlan: () => plan }));
beforeEach(() => { plan.verifiedOwner = { userId: 'a', tenantId: 'a' }; vi.stubGlobal('fetch', vi.fn(async () => Response.json({ hours_saved: 3, inquiries_handled: 2, appointments_scheduled: 1 }))); });
afterEach(() => vi.unstubAllGlobals());
it('shows only the actual owner-bound savings response without a reward claim', async () => {
  render(<Widget />); expect(await screen.findByText('Recorded estimate: 3 hours saved.')).toBeVisible();
  const [,init] = vi.mocked(fetch).mock.calls[0]; const headers = new Headers(init?.headers);
  expect(headers.get('x-ohc-expected-user')).toBe('a'); expect(headers.get('x-ohc-expected-tenant')).toBe('a');
  expect(screen.getByText(/Customer inquiries handled: 2/)).toBeVisible();
  expect(screen.queryByText(/You saved 12|Trial Extended|7 Days/)).not.toBeInTheDocument();
});
it.each([{ error: 'no data' }, { hours_saved: -1 }, { hours_saved: 3, success: false }, { hours_saved: 3, inquiries_handled: -1 }])('does not substitute sample facts for invalid data: %j', async body => {
  vi.stubGlobal('fetch', vi.fn(async () => Response.json(body))); render(<Widget />);
  expect(await screen.findByText('Recorded time-savings data is unavailable.')).toBeVisible();
  expect(screen.queryByText(/12 hours|45 customer|8 appointments/)).not.toBeInTheDocument();
});
it('shows transport failure instead of invented metrics', async () => {
  vi.stubGlobal('fetch', vi.fn(async () => { throw new Error('offline'); })); render(<Widget />);
  expect(await screen.findByText('Recorded time-savings data is unavailable.')).toBeVisible();
});
it('holds private metrics while owner verification is unavailable and ignores a late body', async () => {
  let resolve!: (value: unknown) => void; const body = new Promise(yes => { resolve = yes; });
  vi.stubGlobal('fetch', vi.fn(async () => ({ status: 200, json: () => body }) as Response));
  const view = render(<Widget />); await waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
  plan.verifiedOwner = null; view.rerender(<Widget />);
  await act(async () => resolve({ hours_saved: 99 }));
  expect(screen.queryByText(/99 hours/)).not.toBeInTheDocument(); expect(screen.getByText('Verify your account to read time-savings data.')).toBeVisible();
});
