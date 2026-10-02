import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner } from '@/lib/sync/queueIdentity';
import { SoftPaywallWidget } from './SoftPaywallWidget';
import { ViralTrialExtensionWidget } from './ViralTrialExtensionWidget';
import WinBack from '../win-back/page';
import Trial from '../trial-extension/page';
import Demo from '../interactive-demo/page';
import Referral from '../referral-widget/page';
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
const owner = { userId: 'grant-view-owner', tenantId: 'grant-view-tenant' };
let currentPlan = 'Free';
beforeEach(() => {
  localStorage.clear(); act(() => invalidateQueueOwner()); currentPlan = 'Free';
  vi.spyOn(window, 'open').mockImplementation(() => null); vi.spyOn(window, 'alert').mockImplementation(() => {});
  vi.stubGlobal('fetch', vi.fn(async (url, options) => {
    if (url === '/api/v1/auth/session-identity') return Response.json({ ...owner, expiresAt: Date.now() + 60000 });
    if (url === '/api/v1/billing/my-plan') return Response.json({ current_plan: currentPlan });
    if (url === '/api/v1/growth/campaign/generate-win-back') return Response.json({ subject: 'Actual supplied subject', body: 'Actual supplied campaign body' });
    return Response.json({ error: 'grant_not_available' }, { status: options?.method === 'POST' ? 503 : 200 });
  }));
});
afterEach(() => { act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
const cases = [
  { name: 'soft paywall', Page: SoftPaywallWidget, open: () => fireEvent.click(screen.getByRole('button', { name: /^(Enable|Review automation setup)$/ })) },
  { name: 'viral trial widget', Page: ViralTrialExtensionWidget, open: () => {} },
  { name: 'win-back', Page: WinBack, open: () => { fireEvent.change(screen.getByLabelText('Product to Feature (Optional)'), { target: { value: 'Owner product' } }); fireEvent.change(screen.getByLabelText('Discount Offer (%)'), { target: { value: '15' } }); fireEvent.click(screen.getByRole('button', { name: 'Generate AI Campaign' })); } },
  { name: 'trial page', Page: Trial, open: () => {} },
  { name: 'interactive demo', Page: Demo, open: () => fireEvent.click(screen.getByRole('checkbox')) },
  { name: 'referral builder', Page: Referral, open: () => fireEvent.click(screen.getByRole('checkbox')) },
];
it.each(cases)('$name does not send an unverified grant or claim success from a failed trial', async ({ Page, open }) => {
  await act(async () => { render(<Page />); }); open();
  const button = screen.getByRole('button', { name: /Share on X to|Share to Unlock|Check trial availability/ });
  await act(async () => fireEvent.click(button));
  expect(vi.mocked(fetch).mock.calls.some(([, init]) => init?.method === 'POST')).toBe(false);
  expect(window.open).not.toHaveBeenCalled(); expect(window.alert).not.toHaveBeenCalled();
  expect(screen.queryAllByText(/Pro Access Activated|Pro access activated|Unlocked!|Trial Extended/)).toHaveLength(0);
  expect(localStorage.getItem('has_pro')).toBeNull();
  expect(screen.getByText(/Trial activation is unavailable/)).toBeVisible();
});
it('an unowned saved Pro flag cannot enable automation or be overwritten', async () => {
  localStorage.setItem('has_pro', 'true'); await act(async () => { render(<SoftPaywallWidget />); });
  fireEvent.click(screen.getByRole('button', { name: /^(Enable|Review automation setup)$/ }));
  expect(screen.queryByText(/✅ Enabled/)).not.toBeInTheDocument();
  expect(screen.getByRole('link', { name: 'Open Agents' })).toHaveAttribute('href', '/agents');
  expect(localStorage.getItem('has_pro')).toBe('true');
});
it('an unowned saved Pro flag cannot remove referral branding', async () => {
  localStorage.setItem('has_pro', 'true'); await act(async () => { render(<Referral />); });
  fireEvent.click(screen.getByRole('checkbox'));
  expect(screen.getByRole('button', { name: 'Check trial availability' })).toBeVisible();
  expect(screen.getByRole('checkbox')).not.toBeChecked();
  expect(localStorage.getItem('has_pro')).toBe('true');
});
it('a currently verified Pro owner can still explicitly request the real win-back generator', async () => {
  currentPlan = 'Pro'; await act(async () => { render(<WinBack />); });
  fireEvent.change(screen.getByLabelText('Product to Feature (Optional)'), { target: { value: 'Owner product' } }); fireEvent.change(screen.getByLabelText('Discount Offer (%)'), { target: { value: '15' } });
  const button = screen.getByRole('button', { name: 'Generate AI Campaign' }); await waitFor(() => expect(button).toBeEnabled());
  fireEvent.click(button);
  expect(await screen.findByText(/Actual supplied subject/)).toBeVisible();
  expect(vi.mocked(fetch).mock.calls.filter(([url]) => url === '/api/v1/growth/campaign/generate-win-back')).toHaveLength(1);
  expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/growth/trial-extension/claim')).toBe(false);
});
