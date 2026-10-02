import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
import {openBuilderScope,openBuilderEditor,releaseBuilderEditor,builderScopeActive,readBuilderDraft,writeBuilderDraft,publishOwnedLayout,captureBuilderRestore,assertBuilderRestore,builderDraftKey} from './ownedDraft';
import {installBuilderLocks as installOnboardingLocks} from './testLocks';
let owner={userId:'a',tenantId:'ta'};
const layout={domain:null,draft:{pages:[{path:'/',title:'Owner layout',blocks:[]}]}};
const site={id:'44444444-4444-4444-8444-444444444444',domain:null};
beforeEach(()=>{localStorage.clear();notifyQueueIdentityChange();installOnboardingLocks();owner={userId:'a',tenantId:'ta'};vi.stubGlobal('fetch',vi.fn(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):Response.json(site)));});
afterEach(()=>vi.unstubAllGlobals());
it('holds owner A bytes without exposing or overwriting them from owner B',async()=>{
 const a=await openBuilderScope();writeBuilderDraft('layout',{bio:'Private A'},a);const keyA=builderDraftKey('layout',a);const saved=localStorage.getItem(keyA);
 owner={userId:'b',tenantId:'tb'};notifyQueueIdentityChange();const b=await openBuilderScope();expect(readBuilderDraft('layout',b)).toBeNull();expect(()=>writeBuilderDraft('layout',{bio:'Stale A callback'},a)).toThrow('session changed');writeBuilderDraft('layout',{bio:'B edit'},b);
 expect(localStorage.getItem(keyA)).toBe(saved);owner={userId:'a',tenantId:'ta'};notifyQueueIdentityChange();const returned=await openBuilderScope();expect(readBuilderDraft('layout',returned)?.data).toEqual({bio:'Private A'});
});
it('holds malformed owned bytes and cannot overwrite them through a normal save',async()=>{
 const scope=await openBuilderScope();const key=builderDraftKey('layout',scope);localStorage.setItem(key,'corrupted original');expect(()=>readBuilderDraft('layout',scope)).toThrow('remains held');expect(()=>writeBuilderDraft('layout',{bio:'New'},scope)).toThrow('remains held');expect(localStorage.getItem(key)).toBe('corrupted original');
});
it('rejects a delayed restore when another tab adds a local-only revision',async()=>{
 const scope=await openBuilderScope();const snapshot=captureBuilderRestore('layout',scope);writeBuilderDraft('layout',{bio:'Newer edit'},scope);expect(()=>assertBuilderRestore(snapshot)).toThrow('changed in another view');expect(readBuilderDraft('layout',scope)?.data).toEqual({bio:'Newer edit'});
});
it.each(['network','body','missing-id','rejected-body','server500','accepted202'])('holds unknown publication %s across reload without another POST',async mode=>{
 const scope=await openBuilderScope();let posts=0;
 vi.mocked(fetch).mockImplementation(async url=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});posts++;
  if(mode==='network')throw new Error('Lost network');
  if(mode==='body')return new Response(new ReadableStream({start(controller){controller.error(new Error('Lost body'));}}));
  return Response.json(mode==='missing-id'?{}:mode==='rejected-body'?{success:false,error:'Rejected'}:site,{status:mode==='server500'?500:mode==='accepted202'?202:200});
 });
 await expect(publishOwnedLayout(scope,layout)).rejects.toThrow();const marker=localStorage.getItem(builderDraftKey('builder-publish-fence',scope));expect(marker).not.toBeNull();
 notifyQueueIdentityChange();const reopened=await openBuilderScope();await expect(publishOwnedLayout(reopened,layout)).rejects.toThrow('previous site save');expect(posts).toBe(1);expect(localStorage.getItem(builderDraftKey('builder-publish-fence',reopened))).toBe(marker);
});
it('allows a corrected request after an explicit client rejection without claiming success',async()=>{
 const scope=await openBuilderScope();let posts=0;vi.mocked(fetch).mockImplementation(async url=>{if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});posts++;return posts===1?Response.json({error:'Invalid layout'},{status:400}):Response.json(site);});
 await expect(publishOwnedLayout(scope,layout)).rejects.toThrow('not acknowledged');expect(localStorage.getItem(builderDraftKey('builder-publish-fence',scope))).toBeNull();expect(await publishOwnedLayout(scope,layout)).toEqual(site);expect(posts).toBe(2);
});
it('does not dispatch when the durable publication marker cannot be stored',async()=>{
 const scope=await openBuilderScope();const key=builderDraftKey('builder-publish-fence',scope);const original=localStorage.setItem.bind(localStorage);const mock=vi.spyOn(localStorage,'setItem').mockImplementation((name,value)=>{if(name===key)throw new Error('quota');original(name,value);});
 try{await expect(publishOwnedLayout(scope,layout)).rejects.toThrow('quota');expect(vi.mocked(fetch).mock.calls.some(([url])=>String(url).endsWith('/publish_draft'))).toBe(false);}finally{mock.mockRestore();}
});
it('holds an ambiguous site save after owner invalidation without altering the next owner draft',async()=>{
 const a=await openBuilderScope();let release!:()=>void;
 vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):new Response(new ReadableStream({start(controller){release=()=>{controller.enqueue(new TextEncoder().encode(JSON.stringify(site)));controller.close();};}})));
 const saving=publishOwnedLayout(a,layout).catch(error=>error);await vi.waitFor(()=>expect(release).toBeDefined());owner={userId:'b',tenantId:'tb'};notifyQueueIdentityChange();const b=await openBuilderScope();writeBuilderDraft('layout',{bio:'B private'},b);const key=builderDraftKey('layout',b);const saved=localStorage.getItem(key);release();expect(await saving).toBeInstanceOf(Error);expect(localStorage.getItem(key)).toBe(saved);
});

it('treats a cached publication receipt as a held prior submission instead of a fresh acknowledgement',async()=>{
 const scope=await openBuilderScope();await publishOwnedLayout(scope,layout);
 await expect(publishOwnedLayout(scope,layout)).rejects.toThrow(/recorded save/i);
 expect(vi.mocked(fetch).mock.calls.filter(([url])=>String(url).endsWith('/publish_draft'))).toHaveLength(1);
});
it('freezes the approved publication body before waiting for the origin lock',async()=>{
 const scope=await openBuilderScope();let grant!:()=>Promise<void>;
 Object.defineProperty(navigator,'locks',{value:{request:(_name:string,_options:unknown,run:()=>Promise<{id:string;domain:string|null}>)=>new Promise((resolve,reject)=>{grant=async()=>{try{resolve(await run());}catch(error){reject(error);}};})}});
 const payload=structuredClone(layout);const saving=publishOwnedLayout(scope,payload);payload.draft.pages[0].title='Changed after approval';await grant();await saving;
 const call=vi.mocked(fetch).mock.calls.find(([url])=>String(url).endsWith('/publish_draft'))!;expect(JSON.parse(String(call[1]?.body)).draft.pages[0].title).toBe('Owner layout');
});

it('prevents a second editor from replacing another tab newer unsent local fields',async()=>{
 const a=await openBuilderEditor('website-builder-draft');
 const initial={business:'Original',bio:'Original description'};
 writeBuilderDraft('website-builder-draft',initial,a);
 await expect(openBuilderEditor('website-builder-draft')).rejects.toThrow('open in another view');
 writeBuilderDraft('website-builder-draft',{...initial,business:'A newer business'},a);
 const staleA=readBuilderDraft<typeof initial>('website-builder-draft',a)!.data;
 releaseBuilderEditor(a);
 const b=await openBuilderEditor('website-builder-draft');
 writeBuilderDraft('website-builder-draft',{...staleA,business:'B newer business'},b);
 expect(()=>writeBuilderDraft('website-builder-draft',{...staleA,bio:'A later edit'},a)).toThrow('session changed');
 expect(readBuilderDraft<typeof initial>('website-builder-draft',b)?.data.business).toBe('B newer business');
 releaseBuilderEditor(b);
});
it('holds editors when Web Locks are unavailable without reading or changing private draft fields',async()=>{
 Object.defineProperty(navigator,'locks',{value:undefined});
 await expect(openBuilderEditor('website-builder-draft')).rejects.toThrow('cannot coordinate');
 expect(vi.mocked(fetch).mock.calls.every(([url])=>String(url).endsWith('/session-identity'))).toBe(true);
});
it('releases an editor after logout and restores the retained copy only for its owner',async()=>{
 const a=await openBuilderEditor('storefront-builder-draft');writeBuilderDraft('storefront-builder-draft',{bio:'A private'},a);
 owner={userId:'b',tenantId:'tb'};notifyQueueIdentityChange();
 expect(builderScopeActive(a)).toBe(false);
 const b=await openBuilderEditor('storefront-builder-draft');expect(readBuilderDraft('storefront-builder-draft',b)).toBeNull();
 writeBuilderDraft('storefront-builder-draft',{bio:'B private'},b);
 owner={userId:'a',tenantId:'ta'};notifyQueueIdentityChange();
 const returned=await openBuilderEditor('storefront-builder-draft');expect(readBuilderDraft('storefront-builder-draft',returned)?.data).toEqual({bio:'A private'});releaseBuilderEditor(returned);
});
it('holds unexpected writes from an older client instead of clobbering their persisted row',async()=>{
 const a=await openBuilderEditor('website-builder-draft');writeBuilderDraft('website-builder-draft',{bio:'A'},a);
 const key=builderDraftKey('website-builder-draft',a);const newer=JSON.stringify({format:1,revision:'external',data:{bio:'Newer external'}});localStorage.setItem(key,newer);
 expect(()=>writeBuilderDraft('website-builder-draft',{bio:'Stale edit'},a)).toThrow('changed in another view');expect(localStorage.getItem(key)).toBe(newer);releaseBuilderEditor(a);
});
