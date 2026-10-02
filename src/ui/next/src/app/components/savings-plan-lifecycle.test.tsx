import { act, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, afterEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner } from '@/lib/sync/queueIdentity';
import { useProPlan } from './useProPlan';
import Savings from './AiTimeSavingsWidget';
function Plan() { const { hasPro } = useProPlan(); return <output aria-label="Actual plan">{String(hasPro)}</output>; }
beforeEach(() => { localStorage.clear(); act(() => invalidateQueueOwner()); });
afterEach(() => { act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); });
it.each(['session_identity_changed','queued owner does not match the current session','business_metrics_conflict'])('handles the actual savings409 lifecycle for %s', async error => {
  let resolve!: (response: Response) => void; const response = new Promise<Response>(yes => { resolve = yes; });
  vi.stubGlobal('fetch', vi.fn(async url => url === '/api/v1/auth/session-identity' ? Response.json({ userId: 'a', tenantId: 'a', expiresAt: Date.now() + 60000 }) : url === '/api/v1/billing/my-plan' ? Response.json({ current_plan: 'Pro' }) : response));
  render(<><Plan /><Savings /></>);
  await waitFor(() => expect(screen.getByLabelText('Actual plan')).toHaveTextContent('true'));
  await waitFor(() => expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/growth/time-savings')).toBe(true));
  await act(async () => resolve(Response.json({ error }, { status: 409 })));
  expect(screen.getByLabelText('Actual plan')).toHaveTextContent(error === 'business_metrics_conflict' ? 'true' : 'false');
  expect(screen.queryByText(/Recorded estimate:/)).not.toBeInTheDocument();
});
