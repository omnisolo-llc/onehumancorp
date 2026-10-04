import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import {installOnboardingLocks} from './testLocks';
import {serializeOnboardingDraftWrite,onboardingWriteFenceKey,onboardingDraftWriteProblem,onboardingDraftWritesBusy,subscribeOnboardingWriteState} from './draftWriteGate';
const owner={userId:'owner',tenantId:'tenant'};
beforeEach(()=>{localStorage.clear();installOnboardingLocks();});
afterEach(()=>vi.unstubAllGlobals());
it('uses the shared origin lock across independent document module instances',async()=>{
 const firstTab=await import('./draftWriteGate');vi.resetModules();const secondTab=await import('./draftWriteGate');
 let release!:()=>void;let remote='';const commits:string[]=[];
 const first=firstTab.serializeOnboardingDraftWrite(owner,()=>true,async dispatched=>{dispatched();commits.push('A');return new Response(new ReadableStream({start(controller){release=()=>{remote='A';controller.close();};}}));});
 await vi.waitFor(()=>expect(release).toBeDefined());
 const second=secondTab.serializeOnboardingDraftWrite(owner,()=>true,async dispatched=>{dispatched();commits.push('B');remote='B';return new Response(null,{status:204});});
 await new Promise(resolve=>setTimeout(resolve,10));expect(commits).toEqual(['A']);
 release();await Promise.all([first,second]);expect(remote).toBe('B');expect(localStorage.getItem(onboardingWriteFenceKey(owner))).toBeNull();
});
it('keeps an unknown dispatched result fenced through reload and rejects later mutations',async()=>{
 const send=vi.fn(async dispatched=>{dispatched();throw new Error('Connection lost');});
 await expect(serializeOnboardingDraftWrite(owner,()=>true,send)).rejects.toThrow('Connection lost');
 const saved=localStorage.getItem(onboardingWriteFenceKey(owner));expect(saved).not.toBeNull();
 vi.resetModules();const reopened=await import('./draftWriteGate');
 await expect(reopened.serializeOnboardingDraftWrite(owner,()=>true,send)).rejects.toThrow('reconciliation');
 expect(send).toHaveBeenCalledTimes(1);expect(localStorage.getItem(onboardingWriteFenceKey(owner))).toBe(saved);expect(reopened.onboardingDraftWriteProblem(owner)).toMatch(/reconciliation/);
});
it('holds a crash-surviving in-flight marker without dispatch',async()=>{
 localStorage.setItem(onboardingWriteFenceKey(owner),'crash evidence');const send=vi.fn();
 await expect(serializeOnboardingDraftWrite(owner,()=>true,send)).rejects.toThrow('reconciliation');expect(send).not.toHaveBeenCalled();expect(localStorage.getItem(onboardingWriteFenceKey(owner))).toBe('crash evidence');
});
it('holds a reply whose body fails after successful headers',async()=>{
 await expect(serializeOnboardingDraftWrite(owner,()=>true,async dispatched=>{dispatched();return new Response(new ReadableStream({start(controller){controller.error(new Error('Reply lost'));}}));})).rejects.toThrow('Reply lost');
 expect(localStorage.getItem(onboardingWriteFenceKey(owner))).not.toBeNull();expect(onboardingDraftWriteProblem(owner)).toMatch(/reconciliation/);
});
it('fails closed without origin locks and keeps local data',async()=>{
 localStorage.setItem('owned-draft','latest edit');Object.defineProperty(navigator,'locks',{value:undefined});const send=vi.fn();
 await expect(serializeOnboardingDraftWrite(owner,()=>true,send)).rejects.toThrow('cannot coordinate safe saves');expect(send).not.toHaveBeenCalled();expect(localStorage.getItem('owned-draft')).toBe('latest edit');
});
it('does not send when persisting the in-flight fence fails',async()=>{
 const storage=vi.spyOn(localStorage,'setItem').mockImplementation(()=>{throw new Error('quota');});const send=vi.fn();
 try{await expect(serializeOnboardingDraftWrite(owner,()=>true,send)).rejects.toThrow('could not record');expect(send).not.toHaveBeenCalled();}finally{storage.mockRestore();}
});
it('allows a corrected request after an explicit client rejection without acknowledging it',async()=>{
 const response=await serializeOnboardingDraftWrite(owner,()=>true,async dispatched=>{dispatched();return Response.json({error:'invalid draft'},{status:400});});
 expect(response.status).toBe(400);expect(localStorage.getItem(onboardingWriteFenceKey(owner))).toBeNull();
 await serializeOnboardingDraftWrite(owner,()=>true,async dispatched=>{dispatched();return new Response(null,{status:204});});
});

it('reports every admitted write before lock acquisition and stays busy until the final queued write settles', async () => {
 const grants: (() => Promise<void>)[] = [];
 Object.defineProperty(navigator, 'locks', {value:{request: (_key:string, _options:unknown, run:()=>Promise<Response>) => new Promise<Response>((resolve, reject) => grants.push(async () => {try {resolve(await run());} catch (error) {reject(error);}}))}});
 const states: boolean[] = [];
 const stop = subscribeOnboardingWriteState(() => states.push(onboardingDraftWritesBusy(owner)));
 try {
  const send = vi.fn(async dispatched => {dispatched();return new Response(null,{status:204});});
  const first = serializeOnboardingDraftWrite(owner,()=>true,send);
  const second = serializeOnboardingDraftWrite(owner,()=>true,send);
  expect(onboardingDraftWritesBusy(owner)).toBe(true);
  expect(onboardingDraftWritesBusy({userId:'other',tenantId:'other'})).toBe(false);
  expect(send).not.toHaveBeenCalled();
  await grants[0]();await first;
  expect(onboardingDraftWritesBusy(owner)).toBe(true);
  expect(states).not.toContain(false);
  await grants[1]();await second;
  expect(onboardingDraftWritesBusy(owner)).toBe(false);
  expect(states.at(-1)).toBe(false);
  expect(send).toHaveBeenCalledTimes(2);
 } finally {stop();}
});

it('clears admitted readiness when lock acquisition fails or an unknown write blocks the queued successor', async () => {
 const brokenLocks = {request: vi.fn(async () => {throw new Error('lock unavailable');})};
 Object.defineProperty(navigator, 'locks', {value:brokenLocks});
 await expect(serializeOnboardingDraftWrite(owner,()=>true,vi.fn())).rejects.toThrow('lock unavailable');
 expect(onboardingDraftWritesBusy(owner)).toBe(false);
 installOnboardingLocks();
 let fail!: (error:Error) => void;
 const first = serializeOnboardingDraftWrite(owner,()=>true,async dispatched=>{dispatched();return new Promise((_resolve,reject)=>{fail=reject;});});
 const firstRejected = expect(first).rejects.toThrow('reply lost');
 const nextSend = vi.fn();
 const second = serializeOnboardingDraftWrite(owner,()=>true,nextSend);
 const secondRejected = expect(second).rejects.toThrow('reconciliation');
 await vi.waitFor(()=>expect(fail).toBeDefined());
 expect(onboardingDraftWritesBusy(owner)).toBe(true);
 fail(new Error('reply lost'));await Promise.all([firstRejected,secondRejected]);
 expect(onboardingDraftWritesBusy(owner)).toBe(false);
 expect(nextSend).not.toHaveBeenCalled();
 expect(onboardingDraftWriteProblem(owner)).toMatch(/reconciliation/);
 expect(localStorage.getItem(onboardingWriteFenceKey(owner))).not.toBeNull();
});
