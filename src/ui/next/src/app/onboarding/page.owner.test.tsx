import {beforeEach as beforeLocks} from 'vitest';
beforeLocks(() => installOnboardingLocks());
import {installOnboardingLocks} from './testLocks';
import { act, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { ownedOnboardingKey } from './draftSession';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import Page from './page';
import { useOnboardingStore } from './store';
import { notifyQueueIdentityChange, readQueueOwner } from '@/lib/sync/queueIdentity';
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
let owner = { userId: 'owner-a', tenantId: 'tenant-a' };
beforeEach(async () => {
 localStorage.clear(); notifyQueueIdentityChange();
 owner = { userId:'owner-a',tenantId:'tenant-a' };
 vi.stubGlobal('fetch',vi.fn(async (url:string) => Response.json(url.endsWith('/session-identity') ? {...owner,expiresAt:Date.now()+60_000} : url.endsWith('/state') || url.endsWith('/draft') ? {step:1,chatStep:1} : {})));
 await readQueueOwner();
 useOnboardingStore.setState({step:1,chatStep:1,businessName:'Owner A private studio',businessDescription:'Owner A confidential draft',whatYouSell:'Private service',location:'Private location',targetAudience:'Private clients',isLoading:false,error:'',skipped:false,startResult:null});
 localStorage.setItem('onboarding-storage-v4',JSON.stringify({version:6,state:{businessName:'Owner A private studio',businessDescription:'Owner A confidential draft'}}));
 localStorage.setItem('onboarding_initial_products',JSON.stringify([{name:'Owner A private offer',price:'99.00'}]));
 act(()=>notifyQueueIdentityChange());
 owner = {userId:'owner-b',tenantId:'tenant-b'};
});
afterEach(()=>{vi.unstubAllGlobals();});
it('does not expose another account’s cached business draft after canonical auth invalidation',async()=>{
 render(<Page/>);
 await screen.findByText("What's the name of your business?");
 expect(screen.queryByDisplayValue('Owner A private studio')).toBeNull();
});
it('does not automatically save a prior account’s unowned local draft into the new session',async()=>{
 render(<Page/>);
 await screen.findByText("What's the name of your business?");
 await new Promise(resolve=>setTimeout(resolve,1200));
 const bodies=vi.mocked(fetch).mock.calls.filter(([url,options])=>url==='/api/v1/onboarding/state'&&options?.method==='POST').map(([,options])=>String(options?.body));
 expect(bodies.join('\n')).not.toContain('Owner A');
});

it('holds legacy bytes unchanged while displaying a verified fresh account',async()=>{
 const legacy=localStorage.getItem('onboarding-storage-v4'); const products=localStorage.getItem('onboarding_initial_products');
 render(<Page/>); await screen.findByText("What's the name of your business?");
 expect(localStorage.getItem('onboarding-storage-v4')).toBe(legacy);
 expect(localStorage.getItem('onboarding_initial_products')).toBe(products);
 expect(screen.getByText(/draft is held on this device/)).toBeVisible();
});
it('restores an owned draft only when returning to that verified owner',async()=>{
 const view=render(<Page/>); const input=await screen.findByPlaceholderText(/Maya's Custom Cakes/);
 await userEvent.type(input,'Owner B new studio');
 const keyB=ownedOnboardingKey('draft')!;
 expect(localStorage.getItem(keyB)).toContain('Owner B new studio');
 owner={userId:'owner-c',tenantId:'tenant-c'}; act(()=>notifyQueueIdentityChange());
 await waitFor(()=>expect(screen.getByPlaceholderText(/Maya's Custom Cakes/)).toHaveValue(''));
 expect(localStorage.getItem(keyB)).toContain('Owner B new studio');
 owner={userId:'owner-b',tenantId:'tenant-b'}; act(()=>notifyQueueIdentityChange());
 await waitFor(()=>expect(screen.getByPlaceholderText(/Maya's Custom Cakes/)).toHaveValue('Owner B new studio'));
 view.unmount();
});
it('drops a pending autosave across an account switch before sending its captured payload',async()=>{
 render(<Page/>); const input=await screen.findByPlaceholderText(/Maya's Custom Cakes/);
 await userEvent.type(input,'Owner B private edit');
 owner={userId:'owner-c',tenantId:'tenant-c'}; act(()=>notifyQueueIdentityChange());
 await waitFor(()=>expect(screen.getByPlaceholderText(/Maya's Custom Cakes/)).toHaveValue(''));
 await new Promise(resolve=>setTimeout(resolve,1200));
 const bodies=vi.mocked(fetch).mock.calls.filter(([url,options])=>url==='/api/v1/onboarding/state'&&options?.method==='POST').map(([,options])=>String(options?.body));
 expect(bodies.join('\n')).not.toContain('Owner B private edit');
});

it('keeps the owner’s current edits visible and reports a local storage write failure',async()=>{
 render(<Page/>); const input=await screen.findByPlaceholderText(/Maya's Custom Cakes/);
 const key=ownedOnboardingKey('draft')!; const saved=localStorage.getItem(key);
 const original=localStorage.setItem.bind(localStorage);
 const failure=vi.spyOn(localStorage,'setItem').mockImplementation((name,value)=>{if(name===key)throw new Error('quota');return original(name,value);});
 try {
  await userEvent.type(input,'Unsaved owned edit');
  expect(input).toHaveValue('Unsaved owned edit');
  expect(localStorage.getItem(key)).toBe(saved);
  expect(await screen.findByText(/device couldn.t save your latest edits/i)).toBeVisible();
 } finally {failure.mockRestore();}
});
it('holds malformed owned storage without silently replacing it',async()=>{
 const first=render(<Page/>); await screen.findByPlaceholderText(/Maya's Custom Cakes/);
 const key=ownedOnboardingKey('draft')!; first.unmount(); localStorage.setItem(key,'not valid JSON');
 act(()=>notifyQueueIdentityChange());
 render(<Page/>);
 expect(await screen.findByRole('alert')).toHaveTextContent(/could not be read|remains held/);
 expect(localStorage.getItem(key)).toBe('not valid JSON');
});
it('discards a late draft response after the verified session changes',async()=>{
 let resolve!: (response:Response)=>void;
 let readStarted=false;
 owner={userId:'owner-a',tenantId:'tenant-a'};
 vi.mocked(fetch).mockImplementation(async (url,options)=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(String(url).endsWith('/draft')&&!options?.method&&owner.userId==='owner-a'){readStarted=true;return new Promise(done=>{resolve=done;});}
  return Response.json({step:1,chatStep:1});
 });
 render(<Page/>); await waitFor(()=>expect(readStarted).toBe(true));
 owner={userId:'owner-b',tenantId:'tenant-b'};act(()=>notifyQueueIdentityChange());
 await screen.findByPlaceholderText(/Maya's Custom Cakes/);
 await act(async()=>resolve(Response.json({step:1,chatStep:1,businessName:'Late A secret'})));
 expect(screen.queryByDisplayValue('Late A secret')).toBeNull();
});
it('does not reveal or autosave a held draft when session verification returns401',async()=>{
 vi.mocked(fetch).mockResolvedValue(new Response('{}',{status:401}));
 render(<Page/>); await screen.findByRole('alert');
 expect(screen.queryByDisplayValue('Owner A private studio')).toBeNull();
 expect(vi.mocked(fetch).mock.calls.some(([,options])=>options?.method==='POST')).toBe(false);
});

it('stops background reloads when the authenticated backend refuses the current session',async()=>{
 let stateReads=0;
 vi.mocked(fetch).mockImplementation(async(url)=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(String(url).endsWith('/state')){stateReads++;if(stateReads>4)return new Promise(()=>{});return new Response('{}',{status:401});}
  return Response.json({});
 });
 render(<Page/>); await new Promise(resolve=>setTimeout(resolve,100));
 expect(stateReads).toBeLessThanOrEqual(1);
 expect(screen.getByRole('alert')).toHaveTextContent(/session|sign in/i);
});

it('never resends component-local conversation or unsent text under the next account',async()=>{
 vi.mocked(fetch).mockImplementation(async(url)=>Response.json(String(url).endsWith('/session-identity')?{...owner,expiresAt:Date.now()+60_000}:String(url).endsWith('/chat')?{reply:'Reply for current owner',is_complete:false}:{step:0}));
 render(<Page/>);const input=await screen.findByPlaceholderText('Type a message...');
 await userEvent.type(input,'Old owner private history');await userEvent.click(screen.getByRole('button',{name:/^Send$/}));
 await screen.findByText('Reply for current owner');await userEvent.type(input,'Old owner unsent text');
 owner={userId:'owner-c',tenantId:'tenant-c'};act(()=>notifyQueueIdentityChange());
 const nextInput=await screen.findByPlaceholderText('Type a message...');const restored=nextInput.getAttribute('value');
 await userEvent.clear(nextInput);await userEvent.type(nextInput,'New owner message');await userEvent.click(screen.getByRole('button',{name:/^Send$/}));
 await waitFor(()=>expect(vi.mocked(fetch).mock.calls.filter(([url])=>String(url).endsWith('/chat'))).toHaveLength(2));
 const body=String(vi.mocked(fetch).mock.calls.filter(([url])=>String(url).endsWith('/chat')).at(-1)![1]?.body);
 expect(body).not.toContain('Old owner');expect(restored).toBe('');expect(screen.queryByText('Old owner private history')).toBeNull();
});
it('does not apply a delayed previous-owner draft acknowledgement to a new owner’s save',async()=>{
 const close: Array<()=>void>=[];
 vi.mocked(fetch).mockImplementation(async(url,options)=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(String(url).endsWith('/draft')&&options?.method==='POST')return new Response(new ReadableStream({start(controller){close.push(()=>controller.close());}}),{status:200});
  return Response.json({step:3,businessName:'Current workspace'});
 });
 render(<Page/>);await userEvent.click(await screen.findByRole('button',{name:/Save Draft/i}));
 await waitFor(()=>expect(close).toHaveLength(1));
 owner={userId:'owner-c',tenantId:'tenant-c'};act(()=>notifyQueueIdentityChange());
 await userEvent.click(await screen.findByRole('button',{name:/Save Draft/i}));
 await waitFor(()=>expect(close).toHaveLength(2));
 await act(async()=>close[0]());
 expect(useOnboardingStore.getState().isLoading).toBe(true);expect(screen.queryByText('Draft Saved!')).toBeNull();
 await act(async()=>close[1]());expect(await screen.findByText('Draft Saved!')).toBeVisible();
});

it('restores pending owner-local edits instead of an older server draft',async()=>{
 vi.mocked(fetch).mockImplementation(async(url)=>Response.json(String(url).endsWith('/session-identity')?{...owner,expiresAt:Date.now()+60_000}:{step:1,chatStep:1,businessName:owner.userId==='owner-b'?'Older server draft':''}));
 render(<Page/>);const input=await screen.findByPlaceholderText(/Maya's Custom Cakes/);
 await userEvent.clear(input);await userEvent.type(input,'Latest pending local draft');
 owner={userId:'owner-c',tenantId:'tenant-c'};act(()=>notifyQueueIdentityChange());
 await waitFor(()=>expect(screen.getByPlaceholderText(/Maya's Custom Cakes/)).toHaveValue(''));
 owner={userId:'owner-b',tenantId:'tenant-b'};act(()=>notifyQueueIdentityChange());
 await waitFor(()=>expect(screen.getByPlaceholderText(/Maya's Custom Cakes/)).toHaveValue('Latest pending local draft'));
 expect(screen.getByText('Local edits are pending save.')).toBeVisible();
});
it('does not call newer edits saved when an earlier snapshot acknowledgement arrives',async()=>{
 let close!:()=>void;
 vi.mocked(fetch).mockImplementation(async(url,options)=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(String(url).endsWith('/draft')&&options?.method==='POST')return new Response(new ReadableStream({start(controller){close=()=>controller.close();}}),{status:200});
  return Response.json({step:3,businessName:'Original draft'});
 });
 render(<Page/>);await userEvent.click(await screen.findByRole('button',{name:/Save Draft/i}));
 await waitFor(()=>expect(close).toBeDefined());
 act(()=>useOnboardingStore.getState().setBusinessName('Newer local edit'));
 await act(async()=>close());
 expect(screen.queryByText('Draft Saved!')).toBeNull();expect(screen.getByText('Local edits are pending save.')).toBeVisible();
 expect(localStorage.getItem(ownedOnboardingKey('draft')!)).toContain('Newer local edit');
});

it('serializes repeated Save Draft clicks while the acknowledgement is pending',async()=>{
 const close:Array<()=>void>=[];
 vi.mocked(fetch).mockImplementation(async(url,options)=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(String(url).endsWith('/draft')&&options?.method==='POST')return new Response(new ReadableStream({start(controller){close.push(()=>controller.close());}}));
  return Response.json({step:3,businessName:'Current draft'});
 });
 render(<Page/>);const button=await screen.findByRole('button',{name:/Save Draft/i});
 try{
  await userEvent.click(button);await waitFor(()=>expect(close).toHaveLength(1));await userEvent.click(button);
  expect(close).toHaveLength(1);expect(button).toBeDisabled();
 }finally{await act(async()=>close.forEach(done=>done()));}
});
it.each([false,true])('holds a local-only second-tab revision changed during remote hydration (already pending=%s)',async pending=>{
 const initial=render(<Page/>);await screen.findByPlaceholderText(/Maya's Custom Cakes/);const key=ownedOnboardingKey('draft')!;initial.unmount();
 const row=JSON.parse(localStorage.getItem(key)!);localStorage.setItem(key,JSON.stringify({...row,state:{...row.state,businessName:'Cached A'},revision:'before-read',acknowledgedRevision:pending?null:'before-read'}));
 act(()=>notifyQueueIdentityChange());let resolve!:(response:Response)=>void;
 vi.mocked(fetch).mockImplementation(async(url,options)=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(String(url).endsWith('/draft')&&!options?.method)return new Promise(done=>{resolve=done;});
  return Response.json({});
 });
 render(<Page/>);await waitFor(()=>expect(resolve).toBeDefined());
 const before=JSON.parse(localStorage.getItem(key)!);const newer=JSON.stringify({...before,state:{...before.state,businessName:'Newer unsent B'},revision:'after-read',acknowledgedRevision:null});localStorage.setItem(key,newer);
 await act(async()=>resolve(Response.json({step:1,businessName:'Older remote A'})));
 expect(localStorage.getItem(key)).toBe(newer);expect(await screen.findByRole('alert')).toHaveTextContent(/draft changed in another view/i);expect(screen.queryByDisplayValue('Cached A')).toBeNull();
});
