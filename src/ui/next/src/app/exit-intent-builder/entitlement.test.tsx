import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner, QUEUE_IDENTITY_EPOCH_KEY } from '@/lib/sync/queueIdentity';
import Page from './page';
const owner = { userId: 'exit-owner', tenantId: 'exit-tenant' };
let plan: string | null;
const code = () => screen.getByText((_, element) => element?.tagName === 'CODE').textContent ?? '';
beforeEach(() => {
  localStorage.clear(); act(() => invalidateQueueOwner()); plan = 'Free';
  vi.stubGlobal('fetch', vi.fn(async url => {
    if (url === '/api/v1/auth/session-identity') return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    if (url === '/api/v1/billing/my-plan') return plan === null ? Response.json({ error: 'unavailable' }, { status: 503 }) : Response.json({ current_plan: plan });
    throw new Error(`Unexpected fixture request: ${url}`);
  }));
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: vi.fn().mockResolvedValue(undefined) } });
});
afterEach(() => { cleanup(); act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); vi.unstubAllEnvs(); vi.restoreAllMocks(); });
async function ready() { await waitFor(() => expect(screen.getByRole('switch')).toBeEnabled()); }
it.each(['true', ''])('Free upgrade uses real plan review under VITEST=%j', async mode => {
  vi.stubEnv('VITEST', mode); render(<Page />); await ready();
  fireEvent.click(screen.getByRole('switch'));
  expect(screen.getByRole('link', { name: 'Review plans' })).toHaveAttribute('href', '/pricing');
  expect(screen.queryByRole('button', { name: 'Upgrade to Pro' })).not.toBeInTheDocument();
  fireEvent.click(screen.getByRole('button', { name: 'Cancel' }));
  expect(screen.getByRole('switch')).toHaveAttribute('aria-checked', 'false');
  expect(code()).toContain('Powered by OmniSolo');
  expect(localStorage.getItem('has_pro')).toBeNull();
});
it.each(['Pro', 'Business'])('preserves current %s branding selection and retires it on owner change', async tier => {
  plan = tier; render(<Page />); await ready();
  fireEvent.click(screen.getByRole('switch'));
  expect(screen.getByRole('switch')).toHaveAttribute('aria-checked', 'true');
  expect(code()).not.toContain('Powered by OmniSolo');
  act(() => window.dispatchEvent(new Event('omnisolo_auth_changed')));
  expect(screen.getByRole('switch')).toBeDisabled();
  expect(screen.getByRole('switch')).toHaveAttribute('aria-checked', 'false');
  expect(code()).toContain('Powered by OmniSolo');
});
it('does not present unknown plan data as a verified Free or Pro account', async () => {
  plan = null; render(<Page />);
  await screen.findByText(/Current plan data is unavailable/);
  expect(screen.getByRole('switch')).toBeDisabled();
  expect(code()).toContain('Powered by OmniSolo');
});
it('rechecks a silent owner epoch change before copying paid unbranded code', async () => {
  plan = 'Pro'; render(<Page />); await ready();
  fireEvent.click(screen.getByRole('switch'));
  localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY, 'later-owner');
  fireEvent.click(screen.getByRole('button', { name: 'Copy to Clipboard' }));
  expect(navigator.clipboard.writeText).not.toHaveBeenCalled();
  await waitFor(() => expect(screen.getByRole('switch')).toHaveAttribute('aria-checked', 'false'));
  expect(code()).toContain('Powered by OmniSolo');
});
