import { act, render, screen, fireEvent, waitFor } from '@testing-library/react';
import { beforeEach, afterEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner } from '@/lib/sync/queueIdentity';
import Page from './page';
let currentPlan = 'Free';
beforeEach(() => {
  localStorage.clear(); act(() => invalidateQueueOwner()); currentPlan = 'Free';
  vi.spyOn(window, 'open').mockImplementation(() => null);
  vi.stubGlobal('fetch', vi.fn(async url => Response.json(url === '/api/v1/auth/session-identity' ? { userId: 'a', tenantId: 'a', expiresAt: Date.now() + 60000 } : { current_plan: currentPlan })));
});
afterEach(() => { act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
it('displays the current verified plan without an activation claim', async () => {
  render(<Page />); expect(screen.getByRole('heading', { name: 'Plan and Trial Availability' })).toBeVisible();
  expect(await screen.findByText('Current verified plan: Free.')).toBeVisible();
  expect(screen.queryByText('Pro Access Activated')).not.toBeInTheDocument();
});
it('keeps the unsupported trial explicit without a grant request or popup', async () => {
  render(<Page />); fireEvent.click(screen.getByRole('button', { name: 'Check trial availability' }));
  expect(await screen.findByText(/Trial activation is unavailable/)).toBeVisible();
  expect(window.open).not.toHaveBeenCalled(); expect(vi.mocked(fetch).mock.calls.some(([,options]) => options?.method === 'POST')).toBe(false);
});
it('refreshes an actual changed current plan without claiming a new trial', async () => {
  render(<Page />); await screen.findByText('Current verified plan: Free.'); currentPlan = 'Pro';
  fireEvent.click(screen.getByRole('button', { name: 'Refresh current plan' }));
  expect(await screen.findByText('Current verified plan: Pro.')).toBeVisible();
  expect(screen.queryByText('Pro Access Activated')).not.toBeInTheDocument();
  expect(vi.mocked(fetch).mock.calls.some(([,options]) => options?.method === 'POST')).toBe(false);
});
it('retires the displayed current plan immediately on account change', async () => {
  currentPlan = 'Pro'; render(<Page />); await screen.findByText('Current verified plan: Pro.');
  act(() => window.dispatchEvent(new Event('omnisolo_auth_changed')));
  await waitFor(() => expect(screen.getByRole('status', { name: 'Current plan' })).toHaveTextContent(/session changed/i));
  expect(screen.queryByText('Current verified plan: Pro.')).not.toBeInTheDocument();
});
