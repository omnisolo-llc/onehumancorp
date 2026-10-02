import {installOnboardingLocks} from './testLocks';
import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import {initializeOnboardingDraft,useOnboardingStore,onboardingDraftPending,markOnboardingDraftFromServer,subscribeOnboardingPersistence} from './store';
import {ownedOnboardingKey,fetchForOnboardingOwner} from './draftSession';
import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
import {sendOnboardingDraft} from './draftWrites';
const owner={userId:'a',tenantId:'ta'};
beforeEach(async()=>{
 installOnboardingLocks();localStorage.clear();notifyQueueIdentityChange();
 vi.stubGlobal('fetch',vi.fn(async()=>Response.json({...owner,expiresAt:Date.now()+60_000})));await initializeOnboardingDraft();
});
afterEach(()=>vi.unstubAllGlobals());
it('keeps newer local data pending when an older no-stamp request receives the next lock grant',async()=>{
 const grants:Array<()=>Promise<void>>=[];
 Object.defineProperty(navigator,'locks',{value:{request:(_name:string,_options:unknown,run:()=>Promise<Response>)=>new Promise<Response>((resolve,reject)=>grants.push(async()=>{try{resolve(await run());}catch(error){reject(error);}}))}});
 const payload=()=>JSON.stringify(JSON.parse(localStorage.getItem(ownedOnboardingKey('draft')!)!).state);
 useOnboardingStore.getState().setBusinessName('A');const older=payload();
 useOnboardingStore.getState().setBusinessName('B');const newer=payload();let remote='';
 vi.mocked(fetch).mockImplementation(async(url,options)=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  remote=JSON.parse(String(options?.body)).businessName;return new Response(null,{status:204});
 });
 const first=sendOnboardingDraft('/api/v1/onboarding/draft',{method:'POST',body:newer},owner);
 const stale=sendOnboardingDraft('/api/v1/onboarding/state',{method:'POST',body:older},owner);
 expect(grants).toHaveLength(2);await grants[0]();await first;expect(onboardingDraftPending()).toBe(false);
 await grants[1]();await stale;expect(remote).toBe('A');expect(onboardingDraftPending()).toBe(true);
 expect(JSON.parse(localStorage.getItem(ownedOnboardingKey('draft')!)!).state.businessName).toBe('B');
});

it('keeps an existing wizard draft pending after a partial chat-state write without a snapshot stamp',async()=>{
 useOnboardingStore.getState().setBusinessName('Latest local draft');markOnboardingDraftFromServer();expect(onboardingDraftPending()).toBe(false);
 vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):new Response(null,{status:204}));
 await fetchForOnboardingOwner('/api/v1/onboarding/state',{method:'POST',body:JSON.stringify({chatMessages:[{role:'user',content:'Draft conversation'}]})},owner);
 expect(onboardingDraftPending()).toBe(true);expect(useOnboardingStore.getState().businessName).toBe('Latest local draft');
});
it('notifies the current view when another document advances its owner write generation',()=>{
 useOnboardingStore.getState().setBusinessName('Latest local draft');markOnboardingDraftFromServer();const notified=vi.fn();const stop=subscribeOnboardingPersistence(notified);
 try{const key=ownedOnboardingKey('write-version')!;localStorage.setItem(key,'other-document-write');window.dispatchEvent(new StorageEvent('storage',{key}));expect(notified).toHaveBeenCalled();expect(onboardingDraftPending()).toBe(true);}finally{stop();}
});
