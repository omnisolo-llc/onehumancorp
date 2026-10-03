import { act, cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, test, vi } from 'vitest';
import { UnifiedAgentFeed } from './UnifiedAgentFeed';
import { QUEUE_IDENTITY_EPOCH_KEY, invalidateQueueOwner, readQueueOwner } from '@/lib/sync/queueIdentity';
vi.mock('../utils/offlineQueue', () => ({ getActions: vi.fn().mockResolvedValue([]), enqueueAction: vi.fn(), removeAction: vi.fn() }));
beforeEach(async () => { localStorage.clear(); invalidateQueueOwner(); vi.stubGlobal('fetch', vi.fn(async (url:string) => url.endsWith('/session-identity') ? memoryOwner() : Response.json({items:[]}))); await readQueueOwner(); });
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });
for (const text of ['What is my favorite cake?', 'My favorite cake is chocolate', 'Please remember this private note']) {
 test(`does not manufacture a memory answer or saved receipt for ${text}`, async () => {
  localStorage.setItem('user_favorite_cake','Another owner private preference');
  render(<UnifiedAgentFeed initialData={{items:[],activity:[]}} />);
  fireEvent.change(screen.getByPlaceholderText('Message...'),{target:{value:text}});
  fireEvent.click(screen.getByRole('button',{name:'Send'}));
  expect(await screen.findByRole('alert')).toHaveTextContent('Your draft has not been sent or saved');
  expect(screen.getByPlaceholderText('Message...')).toHaveValue(text);
  expect(screen.queryByText(/consolidated memory|Understood\.|has been remembered|I.ll remember/i)).not.toBeInTheDocument();
  expect(screen.queryByText('Another owner private preference')).not.toBeInTheDocument();
  expect(localStorage.getItem('user_favorite_cake')).toBe('Another owner private preference');
  expect(vi.mocked(fetch).mock.calls.some(([url,options]) => options?.method === 'POST' || String(url).includes('memory'))).toBe(false);
 });
}
for (const change of ['auth','storage','all-storage','pagehide'] as const) {
 test(`retires the unsent draft when ${change} invalidates the session`, async () => {
  render(<UnifiedAgentFeed initialData={{items:[],activity:[]}} />);
  fireEvent.change(screen.getByPlaceholderText('Message...'),{target:{value:'Unsent private text'}});
  await act(async () => {
   if(change==='auth')window.dispatchEvent(new Event('omnisolo_auth_changed'));
   else if(change==='pagehide')window.dispatchEvent(new Event('pagehide'));
   else window.dispatchEvent(new StorageEvent('storage',{key:change==='all-storage'?null:QUEUE_IDENTITY_EPOCH_KEY}));
  });
  expect(screen.getByPlaceholderText('Message...')).toHaveValue('');
  expect(screen.queryByText('Unsent private text')).not.toBeInTheDocument();
 });
}

const memoryOwner = (userId = 'owner-a', tenantId = 'tenant-a') => Response.json({ userId, tenantId, expiresAt: Date.now() + 60_000 });
test('retires an unsent private draft after verified owner replacement without a DOM auth event', async () => {
 vi.mocked(fetch).mockImplementation(async () => memoryOwner()); await readQueueOwner();
 render(<UnifiedAgentFeed initialData={{items:[],activity:[]}} />);
 fireEvent.change(screen.getByPlaceholderText('Message...'),{target:{value:'Prior owner unsent draft'}});
 vi.mocked(fetch).mockImplementation(async () => memoryOwner('owner-b','tenant-b'));
 await act(async () => { await readQueueOwner(); });
 expect(screen.getByPlaceholderText('Message...')).toHaveValue('');
 expect(screen.queryByText('Prior owner unsent draft')).not.toBeInTheDocument();
});
test('hides a draft while its owner is reverified and restores only that same owner', async () => {
 vi.mocked(fetch).mockImplementation(async () => memoryOwner()); await readQueueOwner();
 render(<UnifiedAgentFeed initialData={{items:[],activity:[]}} />);
 const input=screen.getByPlaceholderText('Message...');
 fireEvent.change(input,{target:{value:'Same owner unsent draft'}});
 let release!: (response:Response)=>void;
 const response = new Promise<Response>(resolve => { release=resolve; });
 vi.mocked(fetch).mockImplementation(() => response);
 let verification!:ReturnType<typeof readQueueOwner>;
 await act(async () => { verification=readQueueOwner(); });
 try { expect(input).toHaveValue(''); expect(input).toBeDisabled(); }
 finally { await act(async () => { release(memoryOwner()); await verification; }); }
 expect(input).toHaveValue('Same owner unsent draft');
 expect(input).toBeEnabled();
});
test('a silent storage epoch change retires the prior draft before submission feedback', async () => {
 vi.mocked(fetch).mockImplementation(async () => memoryOwner()); await readQueueOwner();
 render(<UnifiedAgentFeed initialData={{items:[],activity:[]}} />);
 fireEvent.change(screen.getByPlaceholderText('Message...'),{target:{value:'Prior epoch unsent draft'}});
 localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY,'silently-replaced');
 fireEvent.click(screen.getByRole('button',{name:'Send'}));
 expect(screen.getByPlaceholderText('Message...')).toHaveValue('');
 expect(screen.queryByText('Prior epoch unsent draft')).not.toBeInTheDocument();
});
test('same-owner verification in a silently replaced session does not adopt the earlier draft', async () => {
 render(<UnifiedAgentFeed initialData={{items:[],activity:[]}} />);
 fireEvent.change(screen.getByPlaceholderText('Message...'),{target:{value:'Previous session private draft'}});
 localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY,'new-session-same-owner');
 await act(async () => { await readQueueOwner(); });
 expect(screen.getByPlaceholderText('Message...')).toHaveValue('');
});
