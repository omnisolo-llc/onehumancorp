import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import {installBuilderLocks} from '../builder/testLocks';
import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
let owner={userId:'a',tenantId:'ta'};
beforeEach(()=>{localStorage.clear();notifyQueueIdentityChange();installBuilderLocks();owner={userId:'a',tenantId:'ta'};vi.stubGlobal('fetch',vi.fn(async()=>Response.json({...owner,expiresAt:Date.now()+60_000})));});
afterEach(()=>{notifyQueueIdentityChange();vi.unstubAllGlobals();});
it('coordinates independent website store modules and rejects a held view late edit',async()=>{
 vi.resetModules();const tabA=await import('./store');const a=await tabA.initializeWebsiteDraft();tabA.useWebsiteBuilderStore.getState().setBusinessName('A current business');
 vi.resetModules();const tabB=await import('./store');await expect(tabB.initializeWebsiteDraft()).rejects.toThrow('open in another view');
 tabB.useWebsiteBuilderStore.getState().setBio('Held B callback');
 const key='omnisolo_onboarding_owned_v1:'+encodeURIComponent(JSON.stringify(['a','ta']))+':website-builder-draft';
 expect(localStorage.getItem(key)).not.toContain('Held B callback');
 tabA.useWebsiteBuilderStore.getState().setBio('A newest unsent bio');a.release();
 const b=await tabB.initializeWebsiteDraft();expect(tabB.useWebsiteBuilderStore.getState().bio).toBe('A newest unsent bio');
 tabB.useWebsiteBuilderStore.getState().setBusinessName('B current business');
 tabA.useWebsiteBuilderStore.getState().setBio('Stale A completion');
 expect(JSON.parse(localStorage.getItem(key)!).data.state).toMatchObject({businessName:'B current business',bio:'A newest unsent bio'});b.release();
});
it('keeps a shared document lease until every mounted consumer releases it',async()=>{
 vi.resetModules();const tabA=await import('./store');const [a1,a2]=await Promise.all([tabA.initializeWebsiteDraft(),tabA.initializeWebsiteDraft()]);
 vi.resetModules();const tabB=await import('./store');a1.release();await expect(tabB.initializeWebsiteDraft()).rejects.toThrow('open in another view');
 tabA.useWebsiteBuilderStore.getState().setBusinessName('Remaining consumer edit');a2.release();a1.release();
 const b=await tabB.initializeWebsiteDraft();expect(tabB.useWebsiteBuilderStore.getState().businessName).toBe('Remaining consumer edit');b.release();
});
it.each(['auth','pagehide'])('releases both independent documents on %s and keeps each owner local copy',async event=>{
 vi.resetModules();const tab=await import('./store');const a=await tab.initializeWebsiteDraft();tab.useWebsiteBuilderStore.getState().setBio('A saved locally');
 if(event==='auth')notifyQueueIdentityChange();else window.dispatchEvent(new Event('pagehide'));
 expect(tab.useWebsiteBuilderStore.getState().bio).toBe('');a.release();
 owner={userId:'b',tenantId:'tb'};notifyQueueIdentityChange();const b=await tab.initializeWebsiteDraft();expect(tab.useWebsiteBuilderStore.getState().bio).toBe('');tab.useWebsiteBuilderStore.getState().setBio('B saved locally');b.release();
 owner={userId:'a',tenantId:'ta'};notifyQueueIdentityChange();const returned=await tab.initializeWebsiteDraft();expect(tab.useWebsiteBuilderStore.getState().bio).toBe('A saved locally');returned.release();
});
