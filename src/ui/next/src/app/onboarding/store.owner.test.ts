import {beforeEach,afterEach,it,expect,vi} from 'vitest';
import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
import {initializeOnboardingDraft,useOnboardingStore,captureOnboardingDraftWrite,acknowledgeOnboardingDraft,onboardingDraftPending,markOnboardingDraftFromServer} from './store';
import {ownedOnboardingKey} from './draftSession';
let owner={userId:'a',tenantId:'ta'};
function payload(){const raw=JSON.parse(localStorage.getItem(ownedOnboardingKey('draft')!)!);return JSON.stringify(raw.state);}
beforeEach(async()=>{localStorage.clear();notifyQueueIdentityChange();owner={userId:'a',tenantId:'ta'};vi.stubGlobal('fetch',vi.fn(async()=>Response.json({...owner,expiresAt:Date.now()+60_000})));await initializeOnboardingDraft();useOnboardingStore.getState().setBusinessName('Original');markOnboardingDraftFromServer();});
afterEach(()=>vi.unstubAllGlobals());
it('only acknowledges the matching persisted revision and keeps newer edits pending',()=>{
 useOnboardingStore.getState().setBusinessName('First edit');const stamp=captureOnboardingDraftWrite(payload());expect(stamp).not.toBeNull();
 useOnboardingStore.getState().setBusinessName('Newer edit');
 expect(acknowledgeOnboardingDraft(stamp)).toBe(false);expect(onboardingDraftPending()).toBe(true);
 expect(localStorage.getItem(ownedOnboardingKey('draft')!)).toContain('Newer edit');
});
it('keeps the newest local snapshot pending when acknowledgements arrive in reverse order',()=>{
 useOnboardingStore.getState().setBusinessName('First');const first=captureOnboardingDraftWrite(payload());
 useOnboardingStore.getState().setBusinessName('Second');const second=captureOnboardingDraftWrite(payload());
 expect(acknowledgeOnboardingDraft(second)).toBe(true);expect(onboardingDraftPending()).toBe(false);
 expect(acknowledgeOnboardingDraft(first)).toBe(false);expect(onboardingDraftPending()).toBe(true);
 expect(useOnboardingStore.getState().businessName).toBe('Second');
});
it('never lets an old owner acknowledgement clean the current owner’s draft',async()=>{
 useOnboardingStore.getState().setBusinessName('A pending');const first=captureOnboardingDraftWrite(payload());
 owner={userId:'b',tenantId:'tb'};notifyQueueIdentityChange();await initializeOnboardingDraft();useOnboardingStore.getState().setBusinessName('B pending');
 const key=ownedOnboardingKey('draft')!;const before=localStorage.getItem(key);
 expect(acknowledgeOnboardingDraft(first)).toBe(false);expect(localStorage.getItem(key)).toBe(before);expect(onboardingDraftPending()).toBe(true);
});
it('does not mark a partial body as an acknowledgement of the whole local draft',()=>{
 useOnboardingStore.getState().setBusinessName('Pending');
 expect(captureOnboardingDraftWrite(JSON.stringify({businessName:'Pending',businessDescription:'',firstProductName:'',firstProductPrice:''}))).toBeNull();
 expect(onboardingDraftPending()).toBe(true);
});
it('cannot bind an in-memory payload to a different persisted snapshot from another tab',()=>{
 useOnboardingStore.getState().setBusinessName('Tab A');const a=payload();const key=ownedOnboardingKey('draft')!;const row=JSON.parse(localStorage.getItem(key)!);
 localStorage.setItem(key,JSON.stringify({...row,state:{...row.state,businessName:'Tab B newer'},revision:'tab-b',acknowledgedRevision:null}));
 expect(captureOnboardingDraftWrite(a)).toBeNull();expect(onboardingDraftPending()).toBe(true);
});
it('does not overwrite another tab’s newer local fields on a runtime-only state update',()=>{
 useOnboardingStore.getState().setBusinessName('Tab A');const key=ownedOnboardingKey('draft')!;const row=JSON.parse(localStorage.getItem(key)!);const newer=JSON.stringify({...row,state:{...row.state,businessName:'Tab B newer'},revision:'tab-b',acknowledgedRevision:null});
 localStorage.setItem(key,newer);useOnboardingStore.getState().setIsLoading(false);
 expect(localStorage.getItem(key)).toBe(newer);
});
it('restores a newer owner-local revision when the same owner remounts',async()=>{
 useOnboardingStore.getState().setBusinessName('Cached A');const key=ownedOnboardingKey('draft')!;const row=JSON.parse(localStorage.getItem(key)!);
 const newer=JSON.stringify({...row,state:{...row.state,businessName:'Newer tab B'},revision:'tab-b',acknowledgedRevision:null});localStorage.setItem(key,newer);
 await initializeOnboardingDraft();expect(useOnboardingStore.getState().businessName).toBe('Newer tab B');expect(localStorage.getItem(key)).toBe(newer);
});
