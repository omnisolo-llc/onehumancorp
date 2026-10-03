import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import {installOnboardingLocks} from './testLocks';
import {serializeOnboardingDraftWrite,onboardingWriteFenceKey,onboardingDraftWriteProblem} from './draftWriteGate';
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
