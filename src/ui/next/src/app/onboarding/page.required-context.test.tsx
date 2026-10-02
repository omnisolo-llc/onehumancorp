import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { installOnboardingLocks } from './testLocks';
import { initializeOnboardingDraft, markOnboardingDraftFromServer, useOnboardingStore } from './store';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import OnboardingWizard from './page';

vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }), usePathname: () => '/onboarding' }));
vi.mock('@/lib/sync/queueIdentity', async original => ({ ...await original<typeof import('@/lib/sync/queueIdentity')>(), readQueueOwner: vi.fn(async () => ({ userId: 'user-1', tenantId: 'org-1' })) }));
const calls = (path: string) => vi.mocked(fetch).mock.calls.filter(([url]) => url === `/api/v1/onboarding/${path}`);
beforeEach(async () => {
  installOnboardingLocks(); localStorage.clear(); notifyQueueIdentityChange(); await initializeOnboardingDraft();
  useOnboardingStore.setState({ step: 2, bio: 'I advise local businesses', businessDescription: 'I advise local businesses', businessName: 'Owner Studio', businessType: 'Services', whatYouSell: '', location: '', targetAudience: '', categories: [], websiteTemplate: 'Modern', domainChoice: 'subdomain', firstProductName: 'Consultation', firstProductPrice: '25.00', aiAgents: [], aiAutoRespond: false, isLoading: false, error: '', startResult: null, skipped: false });
  markOnboardingDraftFromServer();
  vi.stubGlobal('fetch', vi.fn(async (url: string, options?: RequestInit) => {
    if (url === '/api/v1/onboarding/start') return new Response(null, { status: 400 });
    if (options?.method === 'POST') return new Response(null, { status: 204 });
    return Response.json({});
  }));
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
it.each(['', '   '])('requires owner location and audience before leaving review: %j', value => {
  useOnboardingStore.setState({ location: value, targetAudience: value });
  return (async () => {
    render(<OnboardingWizard />); fireEvent.click(await screen.findByRole('button', { name: 'Continue' }));
    expect(useOnboardingStore.getState().step).toBe(2);
    expect(screen.getByText('Location is required to prepare your workspace.')).toBeVisible();
    expect(screen.getByText('Target audience is required to prepare your workspace.')).toBeVisible();
    expect(calls('start')).toHaveLength(0);
  })();
});
it('returns an incomplete restored style step to review before any preparation POST', async () => {
  useOnboardingStore.setState({ step: 3 }); render(<OnboardingWizard />);
  fireEvent.click(await screen.findByRole('button', { name: 'Approve & Complete Setup' }));
  await waitFor(() => expect(useOnboardingStore.getState().isLoading).toBe(false));
  expect(calls('start')).toHaveLength(0); expect(calls('launch')).toHaveLength(0); expect(useOnboardingStore.getState().step).toBe(2);
});
it('submits the explicitly reviewed context and keeps preparation and launch receipts bound', async () => {
  let saved: Record<string, unknown> = {};
  vi.mocked(fetch).mockImplementation(async (url, options) => {
    const preparation = { preparation_id: 'prep-1', status: 'prepared', organization_id: 'org-1', user_id: 'user-1', primary_product_id: 'product-1', reviewed_request: saved, catalog: [{ product_id: 'product-1', name: 'Consultation', price: '25.00', description: '', variants: [] }] };
    if (url === '/api/v1/onboarding/start') { saved = JSON.parse(String(options?.body)); return Response.json({ success: true, ...preparation, preparation: { ...preparation, reviewed_request: saved } }); }
    if (url === '/api/v1/onboarding/launch') return Response.json({ success: true, ...preparation, status: 'launched', preparation: { ...preparation, status: 'launched' } });
    return options?.method === 'POST' ? new Response(null, { status: 204 }) : Response.json({});
  });
  render(<OnboardingWizard />); await screen.findByRole('button', { name: 'Continue' });
  const location = screen.getByRole('textbox', { name: 'Location' }); const audience = screen.getByRole('textbox', { name: 'Target Audience' });
  expect(location).toHaveValue(''); expect(audience).toHaveValue('');
  fireEvent.change(location, { target: { value: 'Austin, TX' } }); fireEvent.change(audience, { target: { value: 'Local business owners' } });
  fireEvent.click(screen.getByRole('button', { name: 'Continue' })); fireEvent.click(await screen.findByRole('button', { name: 'Approve & Complete Setup' }));
  await waitFor(() => expect(useOnboardingStore.getState().step).toBe(5));
  expect(saved).toMatchObject({ company_name: 'Owner Studio', company_description: 'I advise local businesses', location: 'Austin, TX', target_audience: 'Local business owners', first_product_name: 'Consultation', first_product_price: '25.00' });
  expect(calls('start')).toHaveLength(1); expect(calls('launch')).toHaveLength(1);
  expect(JSON.parse(String(calls('launch')[0][1]?.body))).toEqual({ preparation_id: 'prep-1' });
});
