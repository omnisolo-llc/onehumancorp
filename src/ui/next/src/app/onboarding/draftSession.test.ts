import {beforeEach as beforeLocks} from 'vitest';
beforeLocks(() => installOnboardingLocks());
import {installOnboardingLocks} from './testLocks';
import {beforeEach,afterEach,it,expect,vi} from 'vitest';
import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
import {openOnboardingSession,fetchForOnboardingOwner,onboardingOwner} from './draftSession';
const owner={userId:'a',tenantId:'ta'};
beforeEach(()=>{localStorage.clear();notifyQueueIdentityChange();vi.stubGlobal('fetch',vi.fn(async(url:string)=>Response.json(url.endsWith('/session-identity')?{...owner,expiresAt:Date.now()+60_000}:{})));});
afterEach(()=>vi.unstubAllGlobals());
it.each(['session_identity_changed','queued owner does not match the current session'])('holds the visible owner when the server returns auth409 %s',async error=>{
 const expected=await openOnboardingSession();
 vi.mocked(fetch).mockImplementation(async url=>Response.json(String(url).endsWith('/session-identity')?{...owner,expiresAt:Date.now()+60_000}:{error},{status:String(url).endsWith('/session-identity')?200:409}));
 await fetchForOnboardingOwner('/api/v1/onboarding/draft',{method:'POST',body:'{}'},expected);
 expect(onboardingOwner()).toBeNull();
});
it('retains owner context for an ordinary preparation revision409',async()=>{
 const expected=await openOnboardingSession();
 vi.mocked(fetch).mockImplementation(async url=>Response.json(String(url).endsWith('/session-identity')?{...owner,expiresAt:Date.now()+60_000}:{error:'prepared_catalog_changed'},{status:String(url).endsWith('/session-identity')?200:409}));
 await fetchForOnboardingOwner('/api/v1/onboarding/start',{method:'POST',body:'{}'},expected);
 expect(onboardingOwner()).toEqual(owner);
});
it('never sends an owner precondition or payload to an external origin',async()=>{
 const expected=await openOnboardingSession(); const count=vi.mocked(fetch).mock.calls.length;
 await expect(fetchForOnboardingOwner('https://other.example/api/v1/onboarding/draft',{method:'POST',body:'private'},expected)).rejects.toThrow('Invalid onboarding destination');
 expect(vi.mocked(fetch).mock.calls).toHaveLength(count);
});

it('serializes draft commits across delayed response bodies before accepting a newer save',async()=>{
 const expected=await openOnboardingSession();let releaseFirst!:()=>void;let remote='';const dispatched:string[]=[];
 vi.mocked(fetch).mockImplementation(async(url,options)=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  const value=String(options?.body);dispatched.push(value);
  if(value==='A')return new Response(new ReadableStream({start(controller){releaseFirst=()=>{remote='A';controller.close();};}}));
  remote='B';return new Response(null,{status:204});
 });
 const first=fetchForOnboardingOwner('/api/v1/onboarding/draft',{method:'POST',body:'A'},expected).then(async response=>{await response.text();});
 await vi.waitFor(()=>expect(releaseFirst).toBeDefined());
 const second=fetchForOnboardingOwner('/api/v1/onboarding/state',{method:'POST',body:'B'},expected).then(async response=>{await response.text();});
 await new Promise(resolve=>setTimeout(resolve,10));
 const beforeRelease=[...dispatched];releaseFirst();await Promise.all([first,second]);
 expect(beforeRelease).toEqual(['A']);expect(remote).toBe('B');
});
