import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
import {initializeOnboardingDraft,useOnboardingStore,onboardingDraftPending,markOnboardingDraftFromServer} from './store';
import {ownedOnboardingKey,captureOnboardingRestoreSnapshot,assertOnboardingRestoreSnapshot} from './draftSession';
import {sendOnboardingDraft} from './draftWrites';
import {installOnboardingLocks} from './testLocks';
let owner={userId:'a',tenantId:'ta'};
const read=()=>JSON.parse(localStorage.getItem(ownedOnboardingKey('draft')!)!);
beforeEach(async()=>{
 installOnboardingLocks();localStorage.clear();notifyQueueIdentityChange();owner={userId:'a',tenantId:'ta'};
 vi.stubGlobal('fetch',vi.fn(async()=>Response.json({...owner,expiresAt:Date.now()+60_000})));await initializeOnboardingDraft();
 useOnboardingStore.getState().setBusinessName('A');markOnboardingDraftFromServer();
});
afterEach(()=>vi.unstubAllGlobals());
function permutations(values:string[]):string[][]{return values.length?values.flatMap((value,index)=>permutations(values.filter((_,i)=>i!==index)).map(rest=>[value,...rest])):[[]];}
const traces=permutations(['editB','queueA','queueB','finishA','finishB']).filter(order=>{
 const before=(a:string,b:string)=>order.indexOf(a)<order.indexOf(b);
 return before('queueA','queueB')&&before('queueA','finishA')&&before('queueB','finishB')&&before('finishA','finishB');
});
for(const order of traces)for(const lost of [false,true])for(const editDuringBody of [false,true]){
 it(`preserves the clean-implies-remote-match invariant: ${order.join(' → ')}; lost=${lost}; bodyEdit=${editDuringBody}`,async()=>{
  const grants:Array<()=>Promise<void>>=[];
  Object.defineProperty(navigator,'locks',{value:{request:(_name:string,_options:unknown,run:()=>Promise<Response>)=>new Promise<Response>((resolve,reject)=>grants.push(async()=>{try{resolve(await run());}catch(error){reject(error);}}))}});
  const base=read().state;const bodies={A:JSON.stringify(base),B:JSON.stringify({...base,businessName:'B'})};
  let remote='A';let close:((fail:boolean)=>void)|undefined;let writes=0;let unknown=false;
  vi.mocked(fetch).mockImplementation(async(url,options)=>{
   if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
   const value=JSON.parse(String(options?.body)).businessName;writes++;
   return new Response(new ReadableStream({start(controller){close=fail=>{remote=value;if(fail)controller.error(new Error('Reply lost after commit'));else controller.close();};}}));
  });
  const pending:Record<string,Promise<Response|Error>>={};let nextGrant=0;
  const invariant=()=>{if(!onboardingDraftPending())expect(read().state.businessName).toBe(remote);};
  for(const event of order){
   if(event==='editB')useOnboardingStore.getState().setBusinessName('B');
   else if(event.startsWith('queue')){
    const name=event.endsWith('A')?'A':'B';pending[name]=sendOnboardingDraft('/api/v1/onboarding/draft',{method:'POST',body:bodies[name]},owner).catch(error=>error);
   }else{
    const name=event.endsWith('A')?'A':'B';close=undefined;const grant=grants[nextGrant++]();
    if(!unknown){await vi.waitFor(()=>expect(close).toBeDefined());if(editDuringBody)useOnboardingStore.getState().setBusinessName('C');const fail=lost&&name==='A';close!(fail);if(fail)unknown=true;}
    await grant;await pending[name];
   }
   invariant();
  }
  if(lost){expect(writes).toBe(1);expect(onboardingDraftPending()).toBe(true);}
  const persisted=localStorage.getItem(ownedOnboardingKey('draft')!);await initializeOnboardingDraft();expect(localStorage.getItem(ownedOnboardingKey('draft')!)).toBe(persisted);invariant();
 });
}
it.each(['queued','headers'])('does not apply or dispatch another owner’s snapshot after an owner change at %s',async stage=>{
 const grants:Array<()=>Promise<void>>=[];Object.defineProperty(navigator,'locks',{value:{request:(_name:string,_options:unknown,run:()=>Promise<Response>)=>new Promise<Response>((resolve,reject)=>grants.push(async()=>{try{resolve(await run());}catch(error){reject(error);}}))}});
 let close!:()=>void;let writes=0;
 vi.mocked(fetch).mockImplementation(async url=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  writes++;return new Response(new ReadableStream({start(controller){close=()=>controller.close();}}));
 });
 const pending=sendOnboardingDraft('/api/v1/onboarding/draft',{method:'POST',body:JSON.stringify(read().state)},owner).catch(error=>error);
 const active=stage==='headers'?grants[0]():null;if(active)await vi.waitFor(()=>expect(close).toBeDefined());
 owner={userId:'b',tenantId:'tb'};notifyQueueIdentityChange();await initializeOnboardingDraft();useOnboardingStore.getState().setBusinessName('Private B');markOnboardingDraftFromServer();const before=localStorage.getItem(ownedOnboardingKey('draft')!);
 if(active){close();await active;}else await grants[0]();await pending;
 expect(writes).toBe(stage==='headers'?1:0);expect(localStorage.getItem(ownedOnboardingKey('draft')!)).toBe(before);expect(useOnboardingStore.getState().businessName).toBe('Private B');
});
it.each(['unchanged','draft','products','write-version','owner'])('requires the same restore snapshot after a delayed read: %s',async change=>{
 const snapshot=captureOnboardingRestoreSnapshot();await Promise.resolve();
 if(change==='draft'){const row=read();localStorage.setItem(ownedOnboardingKey('draft')!,JSON.stringify({...row,state:{...row.state,businessName:'Newer local'},revision:'newer'}));}
 if(change==='products')localStorage.setItem(ownedOnboardingKey('products')!,JSON.stringify([{name:'Newer product'}]));
 if(change==='write-version')localStorage.setItem(ownedOnboardingKey('write-version')!,'another-write');
 if(change==='owner')notifyQueueIdentityChange();
 if(change==='unchanged')expect(()=>assertOnboardingRestoreSnapshot(snapshot)).not.toThrow();else expect(()=>assertOnboardingRestoreSnapshot(snapshot)).toThrow('draft changed');
});
