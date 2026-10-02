import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { marketplaceBackend } from './marketplace.test-support';
import Page from './page';
const deferred=<T,>()=>{let resolve!:(value:T)=>void;let reject!:(error:Error)=>void;const promise=new Promise<T>((yes,no)=>{resolve=yes;reject=no;});return{promise,resolve,reject};};
let backend:ReturnType<typeof marketplaceBackend>;
beforeEach(()=>{backend=marketplaceBackend();});
afterEach(()=>{cleanup();notifyQueueIdentityChange();vi.unstubAllGlobals();});
const page=(definitions:unknown[])=>({definitions,installations:[],next_cursor:null,next_installation_cursor:null});
it('keeps the full catalogue after clearing search even when the old body arrives last',async()=>{
 const old=deferred<unknown>(); backend.fetch.mockImplementation(async(url,init)=>url.endsWith('q=Senior')?{status:200,json:()=>old.promise} as Response:backend.route(url,init));
 render(<Page/>);await screen.findByText('Technical Writer');fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:'Senior'}});
 await waitFor(()=>expect(backend.fetch.mock.calls.some(([url])=>url.endsWith('q=Senior'))).toBe(true));
 fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:''}});await screen.findByText('Technical Writer');
 await act(async()=>old.resolve(page([backend.state.definitions[0]])));expect(await screen.findByText('Technical Writer')).toBeVisible();
});
it('does not show an old search error after the current search succeeds',async()=>{
 const old=deferred<Response>();backend.fetch.mockImplementation((url,init)=>url.endsWith('q=Senior')?old.promise:backend.route(url,init));
 render(<Page/>);await screen.findByText('Technical Writer');fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:'Senior'}});
 await waitFor(()=>expect(backend.fetch.mock.calls.some(([url])=>url.endsWith('q=Senior'))).toBe(true));
 fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:''}});await screen.findByText('Technical Writer');
 await act(async()=>old.reject(new Error('Previous request failed')));expect(screen.queryByRole('alert')).toBeNull();
});
it('keeps current loading and results when an older request finishes first',async()=>{
 const older=deferred<unknown>(),current=deferred<unknown>();backend.fetch.mockImplementation(async(url,init)=>url.endsWith('q=Senior')?{status:200,json:()=>older.promise} as Response:url.endsWith('q=Writer')?{status:200,json:()=>current.promise} as Response:backend.route(url,init));
 render(<Page/>);await screen.findByText('Senior Rust Developer');fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:'Senior'}});
 await waitFor(()=>expect(backend.fetch.mock.calls.some(([url])=>url.endsWith('q=Senior'))).toBe(true));fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:'Writer'}});
 await waitFor(()=>expect(backend.fetch.mock.calls.some(([url])=>url.endsWith('q=Writer'))).toBe(true));
 await act(async()=>older.resolve(page([backend.state.definitions[0]])));expect(screen.queryByText('Senior Rust Developer')).toBeNull();expect(screen.getByText('Loading agent definitions…')).toBeVisible();
 await act(async()=>current.resolve(page([backend.state.definitions[1]])));expect(await screen.findByText('Technical Writer')).toBeVisible();
});
