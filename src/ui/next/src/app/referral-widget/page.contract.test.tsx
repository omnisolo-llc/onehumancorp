import React from 'react';
import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner, QUEUE_IDENTITY_EPOCH_KEY } from '@/lib/sync/queueIdentity';
import Page from './page';
const owner = { userId: 'verified-user', tenantId: 'verified-tenant&literal' };
let plan = 'Free';
beforeEach(() => {
  localStorage.clear(); localStorage.setItem('business_display_name', 'forged-store');
  act(() => invalidateQueueOwner()); plan = 'Free';
  vi.stubGlobal('fetch', vi.fn(async input => Response.json(String(input) === '/api/v1/auth/session-identity' ? { ...owner, expiresAt: Date.now() + 60000 } : { current_plan: plan })));
});
afterEach(() => { cleanup(); act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
async function ready() { await waitFor(() => expect(screen.getByRole('checkbox')).toBeEnabled()); }
it('does not invent a referral link, coupon or fulfilled reward', async () => {
  render(<Page />); await ready();
  expect(screen.getByRole('textbox', { name: 'Referral link preview' })).toHaveValue('');
  expect(screen.getByRole('button', { name: 'Link unavailable' })).toBeDisabled();
  expect(screen.getByText(/No coupon, referral link, or reward has been created/)).toBeVisible();
});
it('uses the verified tenant and encoded draft fields for authenticated preview code', async () => {
  plan = 'Business'; render(<Page />); await ready();
  fireEvent.click(screen.getByRole('checkbox'));
  expect(screen.getByRole('checkbox')).toBeChecked();
  fireEvent.change(screen.getByLabelText('Discount Amount'), { target: { value: '20&tenant=forged"' } });
  fireEvent.click(screen.getByRole('button', { name: 'Get Embed Code' }));
  const code = (screen.getByRole('textbox', { name: 'Embed Code' }) as HTMLTextAreaElement).value;
  const url = new URL(code.match(/src="([^"]+)"/)![1].replaceAll('&amp;', '&'));
  expect(url.searchParams.get('tenant')).toBe(owner.tenantId);
  expect(url.searchParams.get('give')).toBe('20&tenant=forged"%');
  expect(url.searchParams.get('hide_branding')).toBe('true');
  expect(screen.getByText(/Authenticated preview code/)).toBeVisible();
});
it.each(['omnisolo_auth_changed', 'pagehide', 'storage'])('%s retires the private preview immediately', async event => {
  render(<Page />); await ready();
  fireEvent.click(screen.getByRole('button', { name: 'Get Embed Code' }));
  expect(screen.getByRole('textbox', { name: 'Embed Code' })).toBeVisible();
  act(() => window.dispatchEvent(event === 'storage' ? new StorageEvent('storage', { key: null }) : new Event(event)));
  expect(screen.queryByRole('textbox', { name: 'Embed Code' })).not.toBeInTheDocument();
  expect(screen.getByRole('button', { name: 'Get Embed Code' })).toBeDisabled();
});
it('does not generate private code while the real plan is unavailable', async () => {
  vi.stubGlobal('fetch', vi.fn(async input => String(input) === '/api/v1/auth/session-identity' ? Response.json({ ...owner, expiresAt: Date.now() + 60000 }) : Response.json({ error: 'unavailable' }, { status: 503 })));
  render(<Page />);
  await screen.findByText(/Current plan data is unavailable/);
  expect(screen.getByRole('button', { name: 'Get Embed Code' })).toBeDisabled();
  expect(screen.queryByRole('textbox', { name: 'Embed Code' })).not.toBeInTheDocument();
});
it('rechecks the owner epoch before copying a previously visible private preview', async () => {
  const writeText = vi.fn().mockResolvedValue(undefined);
  vi.stubGlobal('navigator', { ...navigator, onLine: true, clipboard: { writeText } });
  render(<Page />); await ready();
  fireEvent.click(screen.getByRole('button', { name: 'Get Embed Code' }));
  expect(screen.getByRole('textbox', { name: 'Embed Code' })).toBeVisible();
  // Same-tab storage changes do not emit a storage event. The click must
  // synchronously reject the retired identity before dispatching a copy.
  localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY, 'new-login-epoch');
  fireEvent.click(screen.getByRole('button', { name: 'Copy Code' }));
  expect(writeText).not.toHaveBeenCalled();
  expect(screen.queryByRole('textbox', { name: 'Embed Code' })).not.toBeInTheDocument();
  await ready();
});
