import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, expect, it, vi } from 'vitest';
import AgentMarketplacePage from './page';
const developer={id:'dev',name:'Senior Rust Developer',description:'Development',author:'Fixture',version:'1',endpoint:'https://example.test/dev'};
const writer={...developer,id:'writer',name:'Technical Writer'};
const deferred=<T,>()=>{let resolve!:(value:T)=>void;let reject!:(error:Error)=>void;const promise=new Promise<T>((yes,no)=>{resolve=yes;reject=no;});return {promise,resolve,reject};};
afterEach(()=>vi.unstubAllGlobals());
it('keeps the complete list after clearing search even when the old filtered body arrives last',async()=>{
 const old=deferred<unknown>();
 vi.stubGlobal('fetch',vi.fn(async input=>String(input).endsWith('q=Senior')?{ok:true,json:()=>old.promise}:Response.json([developer,writer])));
 render(<AgentMarketplacePage/>);await screen.findByText('Technical Writer');
 fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:'Senior'}});
 await waitFor(()=>expect(vi.mocked(fetch).mock.calls.some(([url])=>String(url).endsWith('q=Senior'))).toBe(true));
 fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:''}});await screen.findByText('Technical Writer');
 await act(async()=>old.resolve([developer]));expect(screen.getByText('Technical Writer')).toBeVisible();
});
it('does not show an old search error after the current search succeeds',async()=>{
 const old=deferred<Response>();
 vi.stubGlobal('fetch',vi.fn(input=>String(input).endsWith('q=Senior')?old.promise:Promise.resolve(Response.json([writer]))));
 render(<AgentMarketplacePage/>);await screen.findByText('Technical Writer');
 fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:'Senior'}});
 fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:''}});await screen.findByText('Technical Writer');
 await act(async()=>old.reject(new Error('Previous request failed')));expect(screen.queryByRole('alert')).toBeNull();
});
it('keeps the current request loading when an older request finishes first',async()=>{
 const older=deferred<unknown>(),current=deferred<unknown>();
 vi.stubGlobal('fetch',vi.fn(async input=>String(input).endsWith('q=Senior')?{ok:true,json:()=>older.promise}:String(input).endsWith('q=Writer')?{ok:true,json:()=>current.promise}:Response.json([developer])));
 render(<AgentMarketplacePage/>);await screen.findByText('Senior Rust Developer');
 fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:'Senior'}});
 fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:'Writer'}});
 await act(async()=>older.resolve([developer]));expect(screen.queryByText('Senior Rust Developer')).toBeNull();
 await act(async()=>current.resolve([writer]));expect(screen.getByText('Technical Writer')).toBeVisible();
});
