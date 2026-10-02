import { act, cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { fields, marketplaceBackend } from '../marketplace.test-support';
import PublishAgentPage from './page';
const navigation = vi.hoisted(() => ({ push: vi.fn() }));
vi.mock('next/navigation', () => ({ useRouter: () => navigation }));
let backend: ReturnType<typeof marketplaceBackend>;
beforeEach(() => { backend = marketplaceBackend(); navigation.push.mockReset(); });
afterEach(() => { cleanup(); notifyQueueIdentityChange(); vi.unstubAllGlobals(); });
async function fill() {
  await waitFor(() => expect(screen.getByLabelText('Agent Name')).toBeEnabled());
  for (const [label, value] of Object.entries({ 'Agent Name': fields.name, Description: fields.description, Role: fields.role, 'System Prompt': fields.system_prompt })) fireEvent.change(screen.getByLabelText(label), { target: { value } });
}
it('reviews the audience and every public field before sending the real publication request', async () => {
  render(<PublishAgentPage />); await fill();
  expect(backend.posts()).toHaveLength(0);
  fireEvent.click(screen.getByRole('button', { name: 'Review Publication' }));
  const review = screen.getByRole('region', { name: 'Public agent definition review' });
  expect(within(review).getByText(/other authenticated users of this OHC marketplace/)).toBeVisible();
  for (const text of [fields.name, fields.description, fields.role]) expect(within(review).getByText(text, { exact: true })).toBeVisible();
  expect(within(review).getByText(/Write a draft/)).toHaveTextContent('Wait for owner approval before sending.');
  expect(backend.posts()).toHaveLength(0);
  await act(async () => fireEvent.click(screen.getByRole('button', { name: 'Publish Publicly' })));
  await waitFor(() => expect(navigation.push).toHaveBeenCalledWith('/agent-marketplace'));
  expect(backend.posts()).toHaveLength(1);
  const body = JSON.parse(String(backend.posts()[0][1]?.body));
  expect(body).toEqual({ ...fields, request_id: expect.any(String) });
  expect(backend.state.definitions.some(item => item.name === fields.name && item.system_prompt === fields.system_prompt)).toBe(true);
});
it('keeps an unknown request across remount and never automatically repeats its POST', async () => {
  backend.fetch.mockImplementation(async (url, options) => options?.method === 'POST' ? Promise.reject(new Error('Lost connection')) : backend.route(url, options));
  let view = render(<PublishAgentPage />); await fill();
  fireEvent.click(screen.getByRole('button', { name: 'Review Publication' }));
  await act(async () => fireEvent.click(screen.getByRole('button', { name: 'Publish Publicly' })));
  await screen.findByRole('button', { name: 'Check Saved Status' });
  expect(navigation.push).not.toHaveBeenCalled(); expect(backend.posts()).toHaveLength(1);
  view.unmount(); await act(async () => {}); view = render(<PublishAgentPage />);
  await screen.findByRole('button', { name: 'Check Saved Status' });
  expect(backend.posts()).toHaveLength(1);
  expect(screen.getByLabelText('System Prompt')).toHaveValue(fields.system_prompt);
  expect(navigation.push).not.toHaveBeenCalled(); view.unmount();
});
it('clears the previous account’s draft before verifying and editing the next account', async () => {
  render(<PublishAgentPage />); await fill();
  backend.state.owner = { userId: 'owner-b', tenantId: 'tenant-b' };
  act(() => notifyQueueIdentityChange());
  expect(screen.getByLabelText('Agent Name')).toHaveValue('');
  await waitFor(() => expect(screen.getByLabelText('Agent Name')).toBeEnabled());
  expect(screen.getByLabelText('System Prompt')).toHaveValue(''); expect(backend.posts()).toHaveLength(0);
});
const deferred = <T,>() => { let resolve!: (value:T)=>void; const promise=new Promise<T>(done=>{resolve=done;});return {promise,resolve}; };
it('holds a second editor until the first closes, then restores the latest private fields',async()=>{
 const first=render(<PublishAgentPage/>);await fill();const second=render(<PublishAgentPage/>);
 await screen.findByText(/draft is open in another view/);expect(second.container.innerHTML).toContain('Agent Name');expect(second.container.querySelector('input')).toBeDisabled();
 first.unmount();second.unmount();await act(async()=>{});render(<PublishAgentPage/>);
 await waitFor(()=>expect(screen.getByLabelText('Agent Name')).toBeEnabled());expect(screen.getByLabelText('Agent Name')).toHaveValue(fields.name);expect(backend.posts()).toHaveLength(0);
});
it('does not dispatch or discard current fields when durable local storage fails',async()=>{
 render(<PublishAgentPage/>);await fill();const original=vi.mocked(localStorage.setItem).getMockImplementation();
 vi.mocked(localStorage.setItem).mockImplementation(()=>{throw new Error('Full disk');});
 try {fireEvent.change(screen.getByLabelText('Agent Name'),{target:{value:'Latest unsaved name'}});expect(screen.getByRole('alert')).toHaveTextContent('could not save');expect(screen.getByLabelText('Agent Name')).toHaveValue('Latest unsaved name');expect(screen.getByRole('button',{name:'Review Publication'})).toBeDisabled();expect(backend.posts()).toHaveLength(0);}
 finally {vi.mocked(localStorage.setItem).mockImplementation(original!);}
});
it('keeps original request fields and identity held when replay is denied after a lost reply',async()=>{
 let lost=true;backend.fetch.mockImplementation(async(url,init)=>init?.method==='POST'&&lost?Promise.reject(new Error('Lost reply')):backend.route(url,init));
 render(<PublishAgentPage/>);await fill();fireEvent.click(screen.getByRole('button',{name:'Review Publication'}));await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Publish Publicly'})));
 await screen.findByRole('button',{name:'Retry Same Reviewed Request'});lost=false;backend.state.roleAllowed=false;
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Retry Same Reviewed Request'})));
 expect(screen.getByRole('region',{name:'Saved agent operation recovery'})).toHaveTextContent(fields.name);expect(screen.getByLabelText('Agent Name')).toBeDisabled();
 expect(backend.posts()).toHaveLength(2);expect(backend.posts()[0][1]?.body).toBe(backend.posts()[1][1]?.body);expect(navigation.push).not.toHaveBeenCalled();
});
it('permits a newly reviewed request after a definitive first-submission role rejection',async()=>{
 backend.state.roleAllowed=false;render(<PublishAgentPage/>);await fill();fireEvent.click(screen.getByRole('button',{name:'Review Publication'}));await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Publish Publicly'})));
 await waitFor(()=>expect(screen.getByLabelText('Agent Name')).toBeEnabled());expect(screen.getByRole('status')).toHaveTextContent('Only an owner or administrator');
 backend.state.roleAllowed=true;fireEvent.change(screen.getByLabelText('Agent Name'),{target:{value:'New reviewed name'}});fireEvent.click(screen.getByRole('button',{name:'Review Publication'}));await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Publish Publicly'})));
 await waitFor(()=>expect(navigation.push).toHaveBeenCalledWith('/agent-marketplace'));expect(backend.posts()).toHaveLength(2);expect(JSON.parse(String(backend.posts()[0][1]?.body)).request_id).not.toBe(JSON.parse(String(backend.posts()[1][1]?.body)).request_id);
});
it('retires a late publication response body after a verified account change without navigating the new account',async()=>{
 const body=deferred<unknown>();backend.fetch.mockImplementation(async(url,init)=>{const response=await backend.route(url,init);return init?.method==='POST'?{status:200,json:()=>body.promise} as Response:response;});
 render(<PublishAgentPage/>);await fill();fireEvent.click(screen.getByRole('button',{name:'Review Publication'}));fireEvent.click(screen.getByRole('button',{name:'Publish Publicly'}));
 await waitFor(()=>expect(backend.posts()).toHaveLength(1));const receipt=[...backend.state.receipts.values()][0];
 backend.state.owner={userId:'owner-b',tenantId:'tenant-b'};act(()=>notifyQueueIdentityChange());await waitFor(()=>expect(screen.getByLabelText('Agent Name')).toBeEnabled());
 await act(async()=>body.resolve(receipt));expect(screen.getByLabelText('Agent Name')).toHaveValue('');expect(navigation.push).not.toHaveBeenCalled();
});
it('holds editing without Web Locks and preserves existing local data',async()=>{
 Object.defineProperty(navigator,'locks',{value:undefined});localStorage.setItem('unowned-agent-draft','Private legacy data');render(<PublishAgentPage/>);
 await screen.findByText(/browser cannot coordinate safe local drafts/);expect(screen.getByLabelText('Agent Name')).toBeDisabled();expect(localStorage.getItem('unowned-agent-draft')).toBe('Private legacy data');expect(backend.posts()).toHaveLength(0);
});
it('lets the owner correct an oversized field before reviewing and preserves the valid draft',async()=>{
 render(<PublishAgentPage/>);await fill();fireEvent.change(screen.getByLabelText('Agent Name'),{target:{value:'x'.repeat(121)}});
 expect(screen.getByRole('alert')).toHaveTextContent('within 120');expect(screen.getByRole('button',{name:'Review Publication'})).toBeDisabled();expect(screen.getByLabelText('Agent Name')).toBeEnabled();
 fireEvent.change(screen.getByLabelText('Agent Name'),{target:{value:'Corrected name'}});expect(screen.queryByRole('alert')).toBeNull();fireEvent.click(screen.getByRole('button',{name:'Review Publication'}));expect(screen.getByRole('region',{name:'Public agent definition review'})).toHaveTextContent('Corrected name');expect(backend.posts()).toHaveLength(0);
});
it('does not let operation-status recovery unlock a corrupt owner-local publication draft',async()=>{
 const {openBuilderScope,builderDraftKey}=await import('../../builder/ownedDraft');const scope=await openBuilderScope();const key=builderDraftKey('agent-publication-draft',scope);
 const raw=JSON.stringify({format:1,revision:'original-row',data:{...fields,name:['corrupt name']}});localStorage.setItem(key,raw);
 render(<PublishAgentPage/>);await screen.findByText('The saved publication draft remains held.');expect(screen.getByLabelText('Agent Name')).toBeDisabled();
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Check Saved Status'})));
 expect(screen.getByLabelText('Agent Name')).toBeDisabled();fireEvent.change(screen.getByLabelText('Agent Name'),{target:{value:'Replacement'}});expect(localStorage.getItem(key)).toBe(raw);expect(backend.posts()).toHaveLength(0);
});
it('retires an acknowledged publication review while navigation remains pending',async()=>{
 render(<PublishAgentPage/>);await fill();fireEvent.click(screen.getByRole('button',{name:'Review Publication'}));await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Publish Publicly'})));
 await waitFor(()=>expect(navigation.push).toHaveBeenCalledWith('/agent-marketplace'));
 expect(screen.queryByRole('button',{name:'Publish Publicly'})).toBeNull();expect(screen.queryByRole('region',{name:'Public agent definition review'})).toBeNull();expect(backend.posts()).toHaveLength(1);
});
