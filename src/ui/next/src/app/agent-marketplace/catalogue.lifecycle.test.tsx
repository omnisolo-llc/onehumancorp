import { act, cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { marketplaceBackend } from './marketplace.test-support';
import Page from './page';
let backend: ReturnType<typeof marketplaceBackend>;
beforeEach(() => { backend = marketplaceBackend(); });
afterEach(() => { cleanup(); notifyQueueIdentityChange(); vi.unstubAllGlobals(); });
it('reviews the exact definition and saves an inactive install that survives remount without dispatching work', async () => {
  let view = render(<Page />); await screen.findByText('Senior Rust Developer');
  const card = screen.getByRole('article', { name: 'Senior Rust Developer' });
  fireEvent.click(within(card).getByRole('button', { name: 'Install Agent' }));
  const review = screen.getByRole('region', { name: 'Review inactive installation' });
  expect(review.querySelector('pre')?.textContent).toBe(backend.state.definitions[0].system_prompt);
  expect(review).toHaveTextContent('No work starts'); expect(backend.posts()).toHaveLength(0);
  await act(async () => fireEvent.click(within(review).getByRole('button', { name: 'Confirm Inactive Installation' })));
  await waitFor(() => expect(within(card).getByRole('button', { name: 'Installed' })).toHaveAttribute('aria-pressed', 'true'));
  expect(backend.posts()).toHaveLength(1); expect(backend.posts()[0][0]).toMatch(/\/definitions\/[^/]+\/install$/);
  view.unmount(); await act(async () => {}); view = render(<Page />);
  await screen.findByRole('button', { name: 'Installed' }); expect(backend.posts()).toHaveLength(1); view.unmount();
});
it('clears private installations immediately on account change and verifies the next account', async () => {
  render(<Page />); await screen.findByText('Senior Rust Developer');
  fireEvent.click(within(screen.getByRole('article', { name: 'Senior Rust Developer' })).getByRole('button', { name: 'Install Agent' }));
  await act(async () => fireEvent.click(screen.getByRole('button', { name: 'Confirm Inactive Installation' })));
  await screen.findByRole('button', { name: 'Installed' });
  backend.state.owner = { userId: 'owner-b', tenantId: 'tenant-b' }; act(() => notifyQueueIdentityChange());
  expect(screen.queryByRole('button', { name: 'Installed' })).not.toBeInTheDocument();
  await screen.findByText('Senior Rust Developer'); expect(screen.queryByRole('button', { name: 'Installed' })).not.toBeInTheDocument();
});
it('follows public pagination without dropping prior definitions or claiming a partial list is complete', async () => {
  backend.state.pageSize = 1; render(<Page />); await screen.findByText('Senior Rust Developer');
  expect(screen.queryByText('Technical Writer')).not.toBeInTheDocument();
  await act(async () => fireEvent.click(screen.getByRole('button', { name: 'Load More Definitions' })));
  await screen.findByText('Technical Writer'); expect(screen.getByText('Senior Rust Developer')).toBeVisible();
  expect(screen.queryByRole('button', { name: 'Load More Definitions' })).not.toBeInTheDocument();
});
it('shows backend failure as unavailable and never substitutes a sample catalogue', async () => {
  backend.fetch.mockImplementation(async (url, options) => url.includes('/definitions') ? new Response('', { status: 503 }) : backend.route(url, options));
  render(<Page />); await screen.findByRole('button', { name: 'Retry marketplace' });
  expect(screen.queryByText('Senior Rust Developer')).not.toBeInTheDocument(); expect(screen.queryByText(/No agents found/)).not.toBeInTheDocument();
});
it('requires the complete private installation list before offering another install',async()=>{
 backend.state.pageSize=1;
 const first=backend.state.definitions[0],second=backend.state.definitions[1];
 const headers={'x-ohc-expected-user':backend.state.owner.userId,'x-ohc-expected-tenant':backend.state.owner.tenantId};
 for(const item of[first,second])await backend.route(`/api/v1/agents/definitions/${item.id}/install`,{method:'POST',headers,body:JSON.stringify({request_id:crypto.randomUUID(),version:item.version,digest:item.digest})});
 render(<Page/>);await screen.findByRole('button',{name:'Installed'});
 expect(screen.getByText(/Load the remaining private installations/)).toBeVisible();
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Load More Installations'})));
 await waitFor(()=>expect(screen.queryByRole('button',{name:'Load More Installations'})).toBeNull());
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Load More Definitions'})));
 await waitFor(()=>expect(screen.getAllByRole('button',{name:'Installed'})).toHaveLength(2));expect(backend.posts()).toHaveLength(0);
});
it('holds a contradictory paginated definition without crashing or replacing reviewed data',async()=>{
 backend.state.pageSize=1;render(<Page/>);await screen.findByText('Senior Rust Developer');
 const original=backend.state.definitions[0];const {definition}=await import('./marketplace.test-support');
 backend.fetch.mockImplementation(async(url,init)=>url.includes('cursor=')?Response.json({definitions:[definition({...original,name:'Contradictory replacement'})],installations:[],next_cursor:null,next_installation_cursor:null}):backend.route(url,init));
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Load More Definitions'})));
 expect(await screen.findByRole('alert')).toHaveTextContent('conflicting immutable records');expect(screen.getByText('Senior Rust Developer')).toBeVisible();expect(screen.queryByText('Contradictory replacement')).toBeNull();expect(backend.posts()).toHaveLength(0);
});
it('does not accept a202 or unbound installation acknowledgement',async()=>{
 backend.fetch.mockImplementation(async(url,init)=>{const response=await backend.route(url,init);return init?.method==='POST'?Response.json(await response.json(),{status:202}):response;});
 render(<Page/>);await screen.findByText('Senior Rust Developer');fireEvent.click(within(screen.getByRole('article',{name:'Senior Rust Developer'})).getByRole('button',{name:'Install Agent'}));
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Confirm Inactive Installation'})));
 await screen.findByRole('button',{name:'Check Saved Status'});expect(screen.queryByRole('button',{name:'Installed'})).toBeNull();
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Check Saved Status'})));
 await screen.findByRole('button',{name:'Installed'});expect(backend.posts()).toHaveLength(1);
});
it('retains distinct immutable versions of the same definition across pages',async()=>{
 const {definition}=await import('./marketplace.test-support');const first=backend.state.definitions[0];backend.state.definitions=[first,definition({...first,version:2,description:'Revised reviewed purpose'})];backend.state.pageSize=1;
 render(<Page/>);await screen.findByText('Senior Rust Developer');await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Load More Definitions'})));
 await screen.findByText('Revised reviewed purpose');expect(screen.getAllByRole('article',{name:'Senior Rust Developer'})).toHaveLength(2);expect(screen.queryByRole('alert')).toBeNull();
 expect(screen.getByText(/Version 1/)).toBeVisible();expect(screen.getByText(/Version 2/)).toBeVisible();expect(backend.posts()).toHaveLength(0);
});
