import React from 'react';
import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, test, vi } from 'vitest';
import Page from './page';
import { invalidateOnboardingSession } from '../onboarding/draftSession';
import { invalidateQueueOwner, QUEUE_IDENTITY_EPOCH_KEY, readQueueOwner } from '@/lib/sync/queueIdentity';
const ownerA = { userId: 'location-owner-a', tenantId: 'location-tenant-a' };
const ownerB = { userId: 'location-owner-b', tenantId: 'location-tenant-b' };
const actual = {tasks:[{id:'t',title:'Private owner A task',status:'PENDING'}],alerts:[{id:'a',message:'Private owner A summary',severity:'info'}],staff:[{id:'s',name:'Private owner A staff',role:'Manager'}]};
const deferred = <T,>() => { let resolve!: (value:T)=>void; const promise=new Promise<T>(yes=>{resolve=yes;});return {promise,resolve}; };
let owner=ownerA;
let dashboard:()=>Promise<Response>, draft:()=>Promise<Response>, send:()=>Promise<Response>;
let fetcher:ReturnType<typeof vi.fn>;
beforeEach(()=>{
 localStorage.clear();invalidateQueueOwner();invalidateOnboardingSession();owner=ownerA;
 dashboard=async()=>Response.json(actual);draft=async()=>Response.json({draft:'Private owner A draft'});send=async()=>Response.json({error:'unavailable'},{status:501});
 fetcher=vi.fn(async (url:string)=>{
  if(url==='/api/v1/auth/session-identity')return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(url==='/api/v1/location/dashboard')return dashboard();
  if(url==='/api/v1/agent/draft-escalation')return draft();
  if(url==='/api/v1/location/escalate')return send();
  throw new Error('Unexpected destination');
 });vi.stubGlobal('fetch',fetcher);
});
afterEach(()=>{cleanup();vi.unstubAllGlobals();vi.restoreAllMocks();});
const changes=['auth','storage','null-storage','pagehide','verified-owner','silent-storage'] as const;
async function changeOwner(change:typeof changes[number]){
 await act(async()=>{
  if(change==='auth')window.dispatchEvent(new Event('omnisolo_auth_changed'));
  else if(change==='storage')window.dispatchEvent(new StorageEvent('storage',{key:QUEUE_IDENTITY_EPOCH_KEY}));
  else if(change==='null-storage')window.dispatchEvent(new StorageEvent('storage',{key:null}));
  else if(change==='pagehide')window.dispatchEvent(new Event('pagehide'));
  else if(change==='verified-owner'){owner=ownerB;await readQueueOwner();}
  else localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY,'another-session');
 });
}
for(const change of changes){
 test(`${change} rejects late dashboard body from the previous owner`,async()=>{
  const body=deferred<typeof actual>();const json=vi.fn(()=>body.promise);
  dashboard=async()=>({status:200,ok:true,json}) as unknown as Response;
  render(<Page/>);await waitFor(()=>expect(json).toHaveBeenCalled());await changeOwner(change);await act(async()=>body.resolve(actual));
  expect(screen.queryByText('Private owner A staff')).not.toBeInTheDocument();expect(screen.queryByText('Private owner A summary')).not.toBeInTheDocument();
 });
 test(`${change} clears private records and rejects a late escalation draft`,async()=>{
  const body=deferred<{draft:string}>();const json=vi.fn(()=>body.promise);
  draft=async()=>({status:200,ok:true,json}) as unknown as Response;
  render(<Page/>);fireEvent.click(await screen.findByRole('button',{name:'Escalate to Owner'}));await waitFor(()=>expect(json).toHaveBeenCalled());
  await changeOwner(change);await act(async()=>body.resolve({draft:'Late private owner A draft'}));
  expect(screen.queryByRole('dialog')).not.toBeInTheDocument();expect(screen.queryByText('Private owner A staff')).not.toBeInTheDocument();
 });
}
test('same-owner revalidation hides private content then restores the same manual draft',async()=>{
 render(<Page/>);fireEvent.click(await screen.findByRole('button',{name:'Escalate to Owner'}));
 await waitFor(()=>expect(screen.getByRole('textbox',{name:'Escalation draft'})).toHaveValue('Private owner A draft'));
 fireEvent.change(screen.getByRole('textbox',{name:'Escalation draft'}),{target:{value:'Manually reviewed owner A text'}});
 const identity=deferred<Response>();fetcher.mockImplementationOnce(()=>identity.promise);
 let pending!:Promise<unknown>;act(()=>{pending=readQueueOwner();});
 expect(screen.queryByRole('textbox',{name:'Escalation draft'})).not.toBeInTheDocument();
 await act(async()=>{identity.resolve(Response.json({...ownerA,expiresAt:Date.now()+60_000}));await pending;});
 expect(screen.getByRole('textbox',{name:'Escalation draft'})).toHaveValue('Manually reviewed owner A text');
});
test('changed verified owner cannot dispatch a previous owner manual draft',async()=>{
 render(<Page/>);fireEvent.click(await screen.findByRole('button',{name:'Escalate to Owner'}));
 await waitFor(()=>expect(screen.getByRole('textbox',{name:'Escalation draft'})).toHaveValue('Private owner A draft'));
 owner=ownerB;fireEvent.click(screen.getByRole('button',{name:'Send to Owner'}));
 await waitFor(()=>expect(screen.queryByRole('dialog')).not.toBeInTheDocument());
 expect(fetcher.mock.calls.filter(([url])=>url==='/api/v1/location/escalate')).toHaveLength(0);
});
test('real location reads and draft actions bind both verified identity headers',async()=>{
 render(<Page/>);fireEvent.click(await screen.findByRole('button',{name:'Escalate to Owner'}));
 await waitFor(()=>expect(screen.getByRole('textbox',{name:'Escalation draft'})).toHaveValue('Private owner A draft'));
 for(const url of ['/api/v1/location/dashboard','/api/v1/agent/draft-escalation']){
  const call=fetcher.mock.calls.find(([path])=>path===url);expect(call).toBeDefined();const headers=new Headers(call![1]?.headers);
  expect(headers.get('x-ohc-expected-user')).toBe(ownerA.userId);expect(headers.get('x-ohc-expected-tenant')).toBe(ownerA.tenantId);
 }
});
test('expired same-owner access can be reverified without losing a manual draft',async()=>{
 render(<Page/>);fireEvent.click(await screen.findByRole('button',{name:'Escalate to Owner'}));
 await waitFor(()=>expect(screen.getByRole('textbox',{name:'Escalation draft'})).toHaveValue('Private owner A draft'));
 fireEvent.change(screen.getByRole('textbox',{name:'Escalation draft'}),{target:{value:'Owner A manual text'}});
 const later=Date.now()+120_000;vi.spyOn(Date,'now').mockReturnValue(later);
 await act(async()=>invalidateQueueOwner());
 expect(screen.queryByRole('textbox',{name:'Escalation draft'})).not.toBeInTheDocument();
 fireEvent.click(screen.getByRole('button',{name:'Reverify access'}));
 await waitFor(()=>expect(screen.getByRole('textbox',{name:'Escalation draft'})).toHaveValue('Owner A manual text'));
});
test('duplicate sends share one dispatch and cancellation rejects its late result',async()=>{
 const reply=deferred<Response>();send=()=>reply.promise;
 render(<Page/>);fireEvent.click(await screen.findByRole('button',{name:'Escalate to Owner'}));
 await waitFor(()=>expect(screen.getByRole('textbox',{name:'Escalation draft'})).toHaveValue('Private owner A draft'));
 const button=screen.getByRole('button',{name:'Send to Owner'});fireEvent.click(button);fireEvent.click(button);
 await waitFor(()=>expect(fetcher.mock.calls.filter(([url])=>url==='/api/v1/location/escalate')).toHaveLength(1));
 fireEvent.click(await screen.findByRole('button',{name:'Cancel'}));
 await act(async()=>reply.resolve(Response.json({success:true})));
 expect(screen.queryByRole('dialog')).not.toBeInTheDocument();expect(screen.getByText('Private owner A summary')).toBeVisible();
});
test('unmount rejects a late read without opening private content on a new page',async()=>{
 const body=deferred<typeof actual>();const json=vi.fn(()=>body.promise);dashboard=async()=>({status:200,ok:true,json}) as unknown as Response;
 const view=render(<Page/>);await waitFor(()=>expect(json).toHaveBeenCalled());view.unmount();await act(async()=>body.resolve(actual));
 expect(screen.queryByText('Private owner A staff')).not.toBeInTheDocument();
});
