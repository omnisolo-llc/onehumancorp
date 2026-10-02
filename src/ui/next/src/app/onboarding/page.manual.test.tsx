import { act, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { installOnboardingLocks } from './testLocks';
import { initializeOnboardingDraft, markOnboardingDraftFromServer, useOnboardingStore } from './store';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import OnboardingWizard from './page';

vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }), usePathname: () => '/onboarding' }));
vi.mock('@/lib/sync/queueIdentity', async original => ({ ...await original<typeof import('@/lib/sync/queueIdentity')>(), readQueueOwner: vi.fn(async () => ({ userId: 'user-1', tenantId: 'org-1' })) }));
const unavailable = { error: 'onboarding_ai_unconfigured', message: 'AI-assisted setup is unavailable because no model provider is configured. Review and enter your business details manually.' };
beforeEach(async () => {
  installOnboardingLocks(); localStorage.clear(); notifyQueueIdentityChange(); await initializeOnboardingDraft();
  useOnboardingStore.setState({ step: 1, chatStep: 4, bio: '', businessName: 'Owner workshop', whatYouSell: 'I repair bicycles', location: 'Portland', targetAudience: 'Local riders', businessDescription: '', businessType: '', firstProductName: '', firstProductPrice: '', categories: [], aiAgents: [], aiAutoRespond: false, error: '', isLoading: false, startResult: null, skipped: false });
  markOnboardingDraftFromServer();
  vi.stubGlobal('fetch', vi.fn(async (url: string) => ['/api/v1/onboarding/intake', '/api/v1/onboarding/chat', '/api/v1/onboarding/start_zero_click'].includes(url)
    ? Response.json(unavailable, { status: 503 }) : Response.json({})));
  vi.spyOn(console, 'error').mockImplementation(() => {});
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
const calls = (path: string) => vi.mocked(fetch).mock.calls.filter(([url]) => url === '/api/v1/onboarding/' + path);

it('keeps owner inputs and requires an explicit manual review choice when AI intake is unconfigured', async () => {
  render(<OnboardingWizard />);
  await userEvent.click(await screen.findByRole('button', { name: /^Next$/ }));
  const manual = await screen.findByRole('button', { name: 'Review Details Manually' });
  expect(useOnboardingStore.getState().step).toBe(1);
  expect(calls('start')).toHaveLength(0);
  await userEvent.click(manual);
  expect(useOnboardingStore.getState()).toMatchObject({ step: 2, businessName: 'Owner workshop', businessDescription: 'I repair bicycles', firstProductName: '', firstProductPrice: '', aiAgents: [] });
  expect(screen.getByRole('textbox', { name: 'First Product' })).toHaveValue('');
  expect(screen.getByRole('textbox', { name: 'Price' })).toHaveValue('');
});
it('does not treat an arbitrary503 as a verified no-effects manual fallback', async () => {
  vi.mocked(fetch).mockImplementation(async (url: string) => url.endsWith('/intake') ? Response.json({ error: 'unknown_upstream_failure' }, { status: 503 }) : Response.json({}));
  render(<OnboardingWizard />);
  await userEvent.click(await screen.findByRole('button', { name: /^Next$/ }));
  await waitFor(() => expect(useOnboardingStore.getState().isLoading).toBe(false));
  expect(screen.queryByRole('button', { name: 'Review Details Manually' })).toBeNull();
  expect(calls('start')).toHaveLength(0);
});
it('preserves the submitted chat text for explicit manual review without fabricating a product', async () => {
  useOnboardingStore.setState({ step: 0, businessName: '', whatYouSell: '' });
  render(<OnboardingWizard />);
  const input = await screen.findByPlaceholderText('Type a message...');
  await userEvent.type(input, 'I repair bicycles in Portland');
  await userEvent.click(screen.getByRole('button', { name: 'Send' }));
  await userEvent.click(await screen.findByRole('button', { name: 'Review Details Manually' }));
  expect(useOnboardingStore.getState()).toMatchObject({ step: 2, businessDescription: 'I repair bicycles in Portland', firstProductName: '', firstProductPrice: '' });
  expect(calls('start')).toHaveLength(0);
});
it('retires the manual continuation when the verified session changes', async () => {
  render(<OnboardingWizard />);
  await userEvent.click(await screen.findByRole('button', { name: /^Next$/ }));
  await screen.findByRole('button', { name: 'Review Details Manually' });
  act(() => notifyQueueIdentityChange());
  await act(async () => {});
  expect(screen.queryByRole('button', { name: 'Review Details Manually' })).toBeNull();
});
it('keeps the instant prompt and offers manual review only after the explicit unconfigured result', async () => {
  useOnboardingStore.setState({ step: -1, bio: 'I restore bicycles in Portland', whatYouSell: '', businessName: '' });
  render(<OnboardingWizard />);
  await userEvent.click(await screen.findByRole('button', { name: /Generate Storefront/ }));
  await userEvent.click(await screen.findByRole('button', { name: 'Review Details Manually' }));
  expect(useOnboardingStore.getState()).toMatchObject({ step: 2, bio: 'I restore bicycles in Portland', businessDescription: 'I restore bicycles in Portland', firstProductName: '', firstProductPrice: '' });
  expect(calls('start_zero_click')).toHaveLength(1);
  expect(calls('start')).toHaveLength(0);
});

it('retains the new request alongside an existing owner-edited description', async () => {
  useOnboardingStore.setState({ step: 0, businessDescription: 'Existing reviewed details', whatYouSell: '' });
  render(<OnboardingWizard />);
  await userEvent.type(await screen.findByPlaceholderText('Type a message...'), 'Additional owner request');
  await userEvent.click(screen.getByRole('button', { name: 'Send' }));
  await userEvent.click(await screen.findByRole('button', { name: 'Review Details Manually' }));
  expect(useOnboardingStore.getState()).toMatchObject({ bio: 'Additional owner request', businessDescription: 'Existing reviewed details' });
});
