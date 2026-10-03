import { act, cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, test, vi } from 'vitest';
import { UnifiedAgentFeed } from './UnifiedAgentFeed';
import { QUEUE_IDENTITY_EPOCH_KEY } from '@/lib/sync/queueIdentity';
vi.mock('../utils/offlineQueue', () => ({ getActions: vi.fn().mockResolvedValue([]), enqueueAction: vi.fn(), removeAction: vi.fn() }));
beforeEach(() => { localStorage.clear(); vi.stubGlobal('fetch', vi.fn(async () => Response.json({items:[]}))); });
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
