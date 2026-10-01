import { render, screen, fireEvent, act } from '@testing-library/react';
import { beforeEach, afterEach, it, expect, vi } from 'vitest';
import Page from './page';
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
vi.mock('../../components/PoweredByOmniSolo', () => ({ PoweredByOmniSolo: () => null }));
const receipt = { preparation_id: 'prep-1', organization_id: 'org-1', user_id: 'user-1', status: 'prepared', primary_product_id: 'p1', reviewed_request: {}, catalog: [{ product_id: 'p1', name: 'Consulting', price: '25.00', description: '', variants: [] }] };
const result = { success: true, ...receipt, preparation: receipt };
let chat: () => Promise<Response>;
let start: () => Promise<Response>;
let launch: () => Promise<Response>;
beforeEach(() => {
  localStorage.clear();
  chat = async () => Response.json({ reply: 'Review these details', is_complete: true, intake_data: { business_name: 'My studio', business_type: 'Services', categories: ['services'], initial_products: [{ name: 'Consulting', price: '25.00' }] } });
  start = async () => Response.json(result);
  launch = async () => Response.json({ ...result, status: 'launched' });
  vi.stubGlobal('fetch', vi.fn(async (url: string) => url.endsWith('/chat') ? chat() : url.endsWith('/start') ? start() : url.endsWith('/launch') ? launch() : Response.json({})));
  vi.spyOn(console, 'error').mockImplementation(() => {});
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
const calls = (path: string) => vi.mocked(fetch).mock.calls.filter(([url]) => String(url).endsWith('/'+path));
async function submit() {
 const view = render(<Page />);
 const input = await screen.findByPlaceholderText(/e.g. I am a home baker/);
 fireEvent.change(input, { target: { value: 'I offer consulting' } });
 fireEvent.click(screen.getByTestId('generate-storefront-btn'));
 return view;
}
it('chat completion presents review without preparing or launching automatically', async () => {
 await submit();
 expect(await screen.findByRole('button', { name: /Approve.*Prepare/i })).toBeEnabled();
 expect(calls('start')).toHaveLength(0); expect(calls('launch')).toHaveLength(0);
 expect(screen.queryByText(/business is live/i)).toBeNull();
});
it('missing intake never fabricates completion', async () => {
 chat = async () => Response.json({ is_complete: true, reply: 'Done' });
 await submit(); await screen.findByText(/draft is incomplete/i);
 expect(calls('start')).toHaveLength(0); expect(screen.queryByRole('button', { name: /Launch My Store/i })).toBeNull();
});
it('requires strict preparation and matching explicit launch acknowledgements', async () => {
 await submit(); fireEvent.click(await screen.findByRole('button', { name: /Approve.*Prepare/i }));
 const button = await screen.findByRole('button', { name: /Launch My Store/i });
 expect(calls('launch')).toHaveLength(0); expect(localStorage.getItem('has_onboarded')).toBeNull();
 launch = async () => Response.json({ success: true, status: 'launched', preparation_id: 'someone-else' });
 fireEvent.click(button); await screen.findByText(/completion could not be verified/i);
 expect(localStorage.getItem('has_onboarded')).toBeNull();
});
it('a200 preparation rejection never produces success', async () => {
 start = async () => Response.json({ ...result, success: false });
 await submit(); fireEvent.click(await screen.findByRole('button', { name: /Approve.*Prepare/i }));
 await screen.findByText(/not acknowledged/i); expect(calls('start')).toHaveLength(1);
 expect(screen.queryByRole('button', { name: /Launch My Store/i })).toBeNull();
});
it('ignores a chat response after navigation unmount', async () => {
 let resolve!: (r: Response) => void;
 chat = () => new Promise(done => { resolve = done; });
 const view = await submit(); view.unmount();
 await act(async () => resolve(Response.json({ reply: 'done', is_complete: true, intake_data: { business_name: 'Late', initial_products: [{ name: 'Late', price: '10' }] } })));
 expect(calls('start')).toHaveLength(0);
});
it('restores a protected prepared receipt on reload without resubmitting chat or start', async () => {
 vi.mocked(fetch).mockImplementation(async (url) => Response.json(String(url).endsWith('/state') ? { step: 5, preparation: receipt } : {}));
 render(<Page />); expect(await screen.findByRole('button', { name: /Launch My Store/i })).toBeEnabled();
 expect(calls('chat')).toHaveLength(0); expect(calls('start')).toHaveLength(0);
 expect(screen.queryByText('Setup complete')).toBeNull();
});

it.each(['organization_id', 'user_id'])('rejects changed %s when recovering an unknown launch', async field => {
 await submit(); fireEvent.click(await screen.findByRole('button', { name:/Approve.*Prepare/i }));
 launch = async () => { throw new Error('Connection lost'); };
 fireEvent.click(await screen.findByRole('button', { name:/Launch My Store/i }));
 await screen.findByText('Connection lost');
 vi.mocked(fetch).mockImplementation(async url => Response.json(String(url).endsWith('/state') ? {preparation:{...receipt,status:'launched',[field]:'different-owner'}} : {}));
 fireEvent.click(screen.getByRole('button', { name:/Launch My Store/i }));
 await screen.findByText(/Setup changed/);
 expect(screen.queryByText('Setup complete')).toBeNull();
 expect(localStorage.getItem('has_onboarded')).toBeNull();
});
