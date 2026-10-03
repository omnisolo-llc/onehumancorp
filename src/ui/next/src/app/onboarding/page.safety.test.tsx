import {beforeEach as beforeLocks} from 'vitest';
beforeLocks(() => installOnboardingLocks());
import {installOnboardingLocks} from './testLocks';
import { initializeOnboardingDraft, markOnboardingDraftFromServer } from './store';
import { writeOwnedOnboardingItem } from './draftSession';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { act, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
const push = vi.hoisted(() => vi.fn());
vi.mock('next/navigation', () => ({ useRouter: () => ({ push }), usePathname: () => '/onboarding' }));
const preparation = { status: 'prepared', preparation_id: 'preparation-1', organization_id: 'organization-1', user_id: 'user-1', primary_product_id: 'product-1', reviewed_request: {}, catalog: [{ product_id: 'product-1', name: 'Consultation', price: '25.00', description: '', variants: [] }] };
const prepared = { success: true, ...preparation, preparation, product_ids: ['product-1'] };
const launched = { success: true, status: 'launched', preparation_id: prepared.preparation_id, organization_id: prepared.organization_id, user_id: prepared.user_id };
let start: () => Promise<Response>;
let launch: () => Promise<Response>;
beforeEach(async () => {
  localStorage.clear(); notifyQueueIdentityChange(); await initializeOnboardingDraft(); push.mockReset();
  useOnboardingStore.setState({ step: 3, chatStep: 4, businessDescription: 'Owner studio', businessName: 'Owner Studio', whatYouSell: 'Consulting', location: 'Online', targetAudience: 'Local businesses', businessType: 'Services', categories: ['services'], firstProductName: 'Consultation', firstProductPrice: '25.00', aiAgents: [], aiAutoRespond: false, error: '', isLoading: false, startResult: null, skipped: false });
  markOnboardingDraftFromServer();
  start = async () => Response.json(prepared);
  launch = async () => Response.json(launched);
  vi.stubGlobal('fetch', vi.fn(async (url: string) => url === '/api/v1/onboarding/start' ? start() : url === '/api/v1/onboarding/launch' ? launch() : Response.json({})));
  vi.spyOn(console, 'error').mockImplementation(() => {});
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
const calls = (path: string) => vi.mocked(fetch).mock.calls.filter(([url]) => url === `/api/v1/onboarding/${path}`);
it('does not report a contradictory200 draft acknowledgement as saved', async () => {
 vi.mocked(fetch).mockImplementation(async (_url, options) => Response.json(options?.method === 'POST' ? { success: false, error: 'Disk save failed' } : {}));
 render(<OnboardingWizard />);
 await userEvent.click(await screen.findByRole('button', { name: /Save Draft/i }));
 await screen.findByText('Disk save failed');
 expect(screen.queryByText('Draft Saved!')).toBeNull();
});
async function approve() {
  render(<OnboardingWizard />);
  await userEvent.click(await screen.findByRole('button', { name: /Approve.*(?:Publish|setup|launch)/i }));
}
it('projects nullable intake products into the strict preparation contract', async () => {
  writeOwnedOnboardingItem('products', JSON.stringify([
    { name: 'Consultation', price: '25.00', description: null, variants: null },
    { name: 'Second service', price: 15, description: 'Reviewed details', variants: [{ name: 'Extended', price_modifier: 5, model_note: 'omit' }], model_note: 'omit' },
  ]));
  await approve();
  await waitFor(() => expect(calls('start')).toHaveLength(1));
  const payload = JSON.parse(String(calls('start')[0][1]?.body));
  expect(payload.initial_products).toEqual([
    { name: 'Consultation', price: '25.00', description: '', variants: [] },
    { name: 'Second service', price: '15', description: 'Reviewed details', variants: [{ name: 'Extended', price_modifier: '5' }] },
  ]);
});
it('does not automatically repeat preparation after an unknown network outcome', async () => {
  start = async () => { throw new Error('Connection lost'); };
  await approve();
  await waitFor(() => expect(useOnboardingStore.getState().isLoading).toBe(false));
  expect(calls('start')).toHaveLength(1);
  expect(calls('launch')).toHaveLength(0);
});
it('rejects a200 preparation failure instead of advancing to launch', async () => {
  start = async () => Response.json({ ...prepared, success: false, error: 'Preparation rejected' });
  await approve();
  await waitFor(() => expect(useOnboardingStore.getState().isLoading).toBe(false));
  expect(calls('launch')).toHaveLength(0);
  expect(localStorage.getItem('has_onboarded')).toBeNull();
});
it('does not mark setup complete from a200 malformed launch acknowledgement', async () => {
  launch = async () => Response.json({ status: 'error' });
  await approve();
  await waitFor(() => expect(useOnboardingStore.getState().isLoading).toBe(false));
  expect(localStorage.getItem('has_onboarded')).toBeNull();
  expect(useOnboardingStore.getState().step).not.toBe(5);
});
it('retries a failed launch with its committed preparation instead of preparing twice', async () => {
  let count = 0;
  launch = async () => ++count === 1 ? new Response('{}', { status: 503 }) : Response.json(launched);
  await approve();
  await waitFor(() => expect(useOnboardingStore.getState().isLoading).toBe(false));
  expect(localStorage.getItem('has_onboarded')).toBeNull();
  await userEvent.click(screen.getByRole('button', { name: /Approve.*(?:Publish|setup|launch)/i }));
  await waitFor(() => expect(useOnboardingStore.getState().step).toBe(5));
  expect(calls('start')).toHaveLength(1);
  expect(calls('launch')).toHaveLength(2);
  for (const [, request] of calls('launch')) expect(JSON.parse(String(request?.body))).toEqual({ preparation_id: prepared.preparation_id });
});
it('does not launch after the owner skips while preparation is in flight', async () => {
  let resolve!: (response: Response) => void;
  start = () => new Promise<Response>(done => { resolve = done; });
  await approve();
  await userEvent.click(screen.getByRole('button', { name: 'Skip setup' }));
  await act(async () => resolve(Response.json(prepared)));
  await new Promise(done => setTimeout(done, 600));
  expect(calls('launch')).toHaveLength(0);
  expect(push).toHaveBeenCalledWith('/dashboard');
});

it('restores only a protected preparation and launches it without preparing again', async () => {
 vi.mocked(fetch).mockImplementation(async (url, options) => url === '/api/v1/onboarding/state' && !options?.method ? Response.json({ preparation, step: 5 }) : url === '/api/v1/onboarding/launch' ? Response.json(launched) : Response.json({ step: 5, startResult: { success: true } }));
 await approve();
 await waitFor(() => expect(useOnboardingStore.getState().step).toBe(5));
 expect(calls('start')).toHaveLength(0); expect(calls('launch')).toHaveLength(1);
});
it('does not restore a mutable draft or local completion claim as acknowledged setup', async () => {
 useOnboardingStore.setState({ step: 5, startResult: { status: 'launched' } });
 vi.mocked(fetch).mockResolvedValue(Response.json({ step: 5, startResult: prepared }));
 render(<OnboardingWizard />);
 expect(await screen.findByRole('button', { name: /Approve.*setup/i })).toBeVisible();
 expect(screen.queryByText('Setup complete')).toBeNull(); expect(calls('launch')).toHaveLength(0);
});
it('preserves stable product and variant IDs when revising the primary product after reordered reload', async () => {
 const receipt = { ...preparation, catalog: [{ product_id: 'product-2', name: 'Unchanged', price: '10.00', description: 'Keep me', variants: [{ variant_id: 'variant-2', name: 'Blue', price_modifier: '2.00' }] }, preparation.catalog[0]] };
 vi.mocked(fetch).mockImplementation(async (url, options) => url === '/api/v1/onboarding/state' && !options?.method ? Response.json({ preparation: receipt }) : url === '/api/v1/onboarding/start' ? Response.json({ ...prepared, preparation: receipt }) : url === '/api/v1/onboarding/launch' ? Response.json(launched) : Response.json({}));
 render(<OnboardingWizard />);
 await screen.findByRole('button', { name: /Approve.*setup/i });
 act(() => useOnboardingStore.setState({ firstProductName: 'Revised consulting', firstProductPrice: '40.00' }));
 await userEvent.click(screen.getByRole('button', { name: /Approve.*setup/i }));
 await waitFor(() => expect(calls('start')).toHaveLength(1));
 const payload = JSON.parse(String(calls('start')[0][1]?.body));
 expect(payload.replaces_preparation_id).toBe('preparation-1');
 expect(payload.initial_products[0]).toEqual(receipt.catalog[0]);
 expect(payload.initial_products[1]).toMatchObject({ product_id: 'product-1', name: 'Revised consulting', price: '40.00' });
});

it.each(['skip', 'unmount'])('ignores a manual intake result after %s', async mode => {
 let resolve!: (response: Response) => void;
 useOnboardingStore.setState({ step: 1, chatStep: 4, targetAudience: 'Local owners' });
 vi.mocked(fetch).mockImplementation(async url => url === '/api/v1/onboarding/intake' ? new Promise(done => { resolve = done; }) : Response.json({}));
 const view = render(<OnboardingWizard />);
 await userEvent.click(await screen.findByRole('button', { name: /^Next$/ }));
 await waitFor(() => expect(calls('intake')).toHaveLength(1));
 if (mode === 'skip') await userEvent.click(screen.getByRole('button', { name:'Skip setup' })); else view.unmount();
 expect(useOnboardingStore.getState().isLoading).toBe(false);
 const before = useOnboardingStore.getState(); const writesBefore = calls('state').length;
 await act(async () => resolve(Response.json({ business_name:'Late business',business_type:'Late type',categories:['services'],initial_products:[{name:'Late product',price:'99.00'}] })));
 expect(useOnboardingStore.getState().businessName).toBe(before.businessName);
 expect(useOnboardingStore.getState().step).toBe(before.step);
 expect(calls('state')).toHaveLength(writesBefore);
});

it('preserves a saved revised draft on reload while binding the committed receipt separately', async () => {
 vi.mocked(fetch).mockImplementation(async (url, options) => url === '/api/v1/onboarding/state' && !options?.method ? Response.json({preparation}) : url === '/api/v1/onboarding/draft' && !options?.method ? Response.json({step:3,businessName:'Revised studio',firstProductName:'Revised offer',firstProductPrice:'40.00'}) : url === '/api/v1/onboarding/start' ? Response.json(prepared) : url === '/api/v1/onboarding/launch' ? Response.json(launched) : Response.json({}));
 render(<OnboardingWizard />);
 await screen.findByRole('button',{name:/Approve.*setup/i});
 expect(useOnboardingStore.getState()).toMatchObject({businessName:'Revised studio',firstProductName:'Revised offer',firstProductPrice:'40.00'});
 await userEvent.click(screen.getByRole('button',{name:/Approve.*setup/i}));
 await waitFor(()=>expect(calls('start')).toHaveLength(1));
 const payload=JSON.parse(String(calls('start')[0][1]?.body));
 expect(payload).toMatchObject({replaces_preparation_id:'preparation-1',company_name:'Revised studio'});
 expect(payload.initial_products[0]).toMatchObject({product_id:'product-1',name:'Revised offer',price:'40.00'});
});
it.each(['organization_id','user_id'])('rejects a revision receipt with contradictory %s', async field => {
 vi.mocked(fetch).mockImplementation(async (url, options) => url === '/api/v1/onboarding/state' && !options?.method ? Response.json({preparation}) : url === '/api/v1/onboarding/start' ? Response.json({...prepared,[field]:'foreign',preparation:{...preparation,[field]:'foreign'}}) : Response.json({}));
 render(<OnboardingWizard />); await screen.findByRole('button',{name:/Approve.*setup/i});
 act(()=>useOnboardingStore.setState({firstProductName:'Edited'}));
 await userEvent.click(screen.getByRole('button',{name:/Approve.*setup/i}));
 await waitFor(()=>expect(useOnboardingStore.getState().isLoading).toBe(false));
 expect(calls('launch')).toHaveLength(0);
 expect(useOnboardingStore.getState().error).toMatch(/identity|match|changed/i);
});

vi.mock('@/lib/sync/queueIdentity', async importOriginal => ({ ...await importOriginal<typeof import('@/lib/sync/queueIdentity')>(), readQueueOwner: vi.fn(async () => ({ userId: 'user-1', tenantId: 'organization-1' })) }));
