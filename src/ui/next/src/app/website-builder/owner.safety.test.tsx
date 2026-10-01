import {act,render,screen,waitFor,fireEvent} from '@testing-library/react';
import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import WebsiteBuilderPage from './page';
import StorefrontBuilderPage from '../storefront-builder/page';
import {useWebsiteBuilderStore} from './store';
import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
import {installBuilderLocks as installOnboardingLocks} from '../builder/testLocks';
const push=vi.hoisted(()=>vi.fn());
vi.mock('next/navigation',()=>({useRouter:()=>({push,replace:vi.fn(),refresh:vi.fn()})}));
vi.mock('../../components/help',()=>({useWalkthrough:()=>({startWalkthrough:vi.fn()})}));
vi.mock('../../components/TooltipRegistry',()=>({WithTooltip:({children}:React.PropsWithChildren)=><>{children}</>}));
vi.mock('../../components/Walkthrough',()=>({WalkthroughTarget:({children}:React.PropsWithChildren)=><>{children}</>,InteractiveWalkthrough:()=>null}));
vi.mock('../builder/components',()=>({SmartBlock:({props}: {props:unknown})=><div>{JSON.stringify(props)}</div>,DraggableBlock:({children}:React.PropsWithChildren)=><div>{children}</div>,ActionSheet:()=>null}));
let owner={userId:'b',tenantId:'tb'};
beforeEach(()=>{
 push.mockClear();localStorage.clear();notifyQueueIdentityChange();installOnboardingLocks();owner={userId:'b',tenantId:'tb'};
 vi.stubGlobal('fetch',vi.fn(async(url:string)=>Response.json(url.endsWith('/session-identity')?{...owner,expiresAt:Date.now()+60_000}:{})));
});
afterEach(()=>vi.unstubAllGlobals());
it('holds the unowned website wizard cache instead of exposing its business under a new owner',async()=>{
 const legacy=JSON.stringify({version:2,state:{wizardStep:2,businessName:'Private A business',bio:'Private A description',status:'idle'}});localStorage.setItem('website-builder-storage',legacy);await useWebsiteBuilderStore.persist.rehydrate();
 render(<WebsiteBuilderPage/>);await act(async()=>{await Promise.resolve();});
 expect(screen.queryByDisplayValue('Private A business')).toBeNull();expect(screen.queryByDisplayValue('Private A description')).toBeNull();expect(localStorage.getItem('website-builder-storage')).toBe(legacy);
});
it('does not auto-upload a held unowned website draft under the next session',async()=>{
 localStorage.setItem('website-builder-storage',JSON.stringify({version:2,state:{wizardStep:2,businessName:'Private A business',bio:'Private A description',status:'idle'}}));await useWebsiteBuilderStore.persist.rehydrate();render(<WebsiteBuilderPage/>);
 await waitFor(()=>expect(screen.queryByText('Verifying your builder session…')).toBeNull());
 await act(async()=>{useWebsiteBuilderStore.setState({wizardStep:2});});
 const input=await screen.findByPlaceholderText('What is your business called?');fireEvent.change(input,{target:{value:'Current B business'}});
 await new Promise(resolve=>setTimeout(resolve,1150));
 const writes=vi.mocked(fetch).mock.calls.filter(([,options])=>options?.method==='POST');expect(writes.some(([,options])=>String(options?.body).includes('Private A'))).toBe(false);
});
it('holds unowned storefront fields without displaying them in a new account',async()=>{
 localStorage.setItem('omnisolo_builder_bio','Private A storefront');render(<StorefrontBuilderPage/>);await act(async()=>{await Promise.resolve();});
 expect(screen.queryByDisplayValue('Private A storefront')).toBeNull();expect(localStorage.getItem('omnisolo_builder_bio')).toBe('Private A storefront');
});
it('does not claim unsupported server layout persistence from a dropped builderState payload',async()=>{
 render(<StorefrontBuilderPage/>);const input=await screen.findByPlaceholderText(/mobile dog grooming service/i);fireEvent.change(input,{target:{value:'Current owner local layout'}});
 await new Promise(resolve=>setTimeout(resolve,1150));
 expect(vi.mocked(fetch).mock.calls.some(([url,options])=>url==='/api/v1/onboarding/state'&&options?.method==='POST')).toBe(false);
 expect(await screen.findByText(/saved on this device/i)).toBeVisible();expect(screen.queryByText('Draft Saved!')).toBeNull();
});

it('ignores an old storefront response body after an account change while the next owner is busy',async()=>{
 owner={userId:'a',tenantId:'ta'};let releaseA!:()=>void;let releaseB!:()=>void;let bodyA=false;
 vi.mocked(fetch).mockImplementation(async(url)=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(String(url).endsWith('/builder/generate')){
   const who=owner.userId;const response=new Response(new ReadableStream({start(controller){const release=()=>{controller.enqueue(new TextEncoder().encode(JSON.stringify({pages:[{blocks:[{block_type:'HeroBlock',content:{headline:who==='a'?'Private A response':'Current B response'}}]}]})));controller.close();};if(who==='a')releaseA=release;else releaseB=release;}}));
   if(who==='a'){const json=response.json.bind(response);response.json=()=>{bodyA=true;return json();};}return response;
  }
  return Response.json({});
 });
 render(<StorefrontBuilderPage/>);const a=await screen.findByPlaceholderText(/mobile dog grooming service/i);fireEvent.change(a,{target:{value:'A private request'}});fireEvent.click(screen.getByText('Build My Storefront'));await waitFor(()=>expect(bodyA).toBe(true));
 owner={userId:'b',tenantId:'tb'};act(()=>notifyQueueIdentityChange());const b=await screen.findByPlaceholderText(/mobile dog grooming service/i);fireEvent.change(b,{target:{value:'B current request'}});fireEvent.click(screen.getByText('Build My Storefront'));await waitFor(()=>expect(releaseB).toBeDefined());
 await act(async()=>releaseA());expect(screen.queryByText(/Private A response/)).toBeNull();expect(screen.getByText('Agents are building your store...')).toBeVisible();
 await act(async()=>releaseB());expect(await screen.findByText(/Current B response/)).toBeVisible();
});
it.each(['review','instant'])('moves %s setup to the canonical review without implicitly preparing or launching',async mode=>{
 render(<WebsiteBuilderPage/>);await screen.findByText('Your business, live in minutes.');
 act(()=>useWebsiteBuilderStore.setState({wizardStep:mode==='review'?9:'instant-build',businessName:'Current B business',businessType:'Store',productName:'Current B product',productPrice:'12.34',bio:'Current B description'}));
 fireEvent.click(screen.getByRole('button',{name:mode==='review'?/Publish my business|Review workspace setup/:/^Next$|Review setup options/}));
 await waitFor(()=>expect(push).toHaveBeenCalledWith('/onboarding'));
 expect(vi.mocked(fetch).mock.calls.some(([url])=>['/api/v1/onboarding/start','/api/v1/onboarding/intake','/api/v1/onboarding/launch'].includes(String(url)))).toBe(false);
 expect(screen.queryByText('Success! Your business is live!')).toBeNull();
});
it('reports a real site save without inventing a published URL',async()=>{
 render(<WebsiteBuilderPage/>);await screen.findByText('Your business, live in minutes.');act(()=>useWebsiteBuilderStore.setState({status:'draft',blocks:[{type:'Hero',props:{headline:'B layout'}}]}));
 vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):Response.json({id:'22222222-2222-4222-8222-222222222222',domain:null}));
 fireEvent.click(screen.getByText(/1-Tap Launch|Save site for publishing/));
 expect(await screen.findByText(/Site saved.*publishing.*not.*verified/i)).toBeVisible();expect(screen.queryByText('/bio/myshop')).toBeNull();expect(screen.queryByText('Success! Your business is live!')).toBeNull();
});

it('keeps a newer edited layout local when an earlier publication body completes',async()=>{
 render(<WebsiteBuilderPage/>);await screen.findByText('Your business, live in minutes.');act(()=>useWebsiteBuilderStore.setState({status:'draft',blocks:[{type:'Hero',props:{headline:'Earlier layout'}}]}));
 let release!:()=>void;
 vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):new Response(new ReadableStream({start(controller){release=()=>{controller.enqueue(new TextEncoder().encode(JSON.stringify({id:'22222222-2222-4222-8222-222222222222',domain:null})));controller.close();};}})));
 fireEvent.click(screen.getByText('Save site for publishing'));await waitFor(()=>expect(release).toBeDefined());act(()=>useWebsiteBuilderStore.setState({blocks:[{type:'Hero',props:{headline:'Newer local layout'}}]}));await act(async()=>release());
 expect(await screen.findByText(/Earlier layout saved.*newer.*local/i)).toBeVisible();expect(useWebsiteBuilderStore.getState().blocks[0].props.headline).toBe('Newer local layout');
});
it('cancels a pending setup handoff when the user goes back',async()=>{
 render(<WebsiteBuilderPage/>);await screen.findByText('Your business, live in minutes.');act(()=>useWebsiteBuilderStore.setState({wizardStep:9,businessName:'B business',productName:'B product',productPrice:'12.00'}));
 let release!:(response:Response)=>void;vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?new Promise(done=>{release=done;}):Response.json({}));
 fireEvent.click(screen.getByText('Review workspace setup'));await waitFor(()=>expect(release).toBeDefined());fireEvent.click(screen.getByRole('button',{name:/Back/}));await act(async()=>release(Response.json({...owner,expiresAt:Date.now()+60_000})));
 expect(push).not.toHaveBeenCalled();expect(useWebsiteBuilderStore.getState().wizardStep).toBe('8.5');expect(screen.getByRole('button',{name:/Save Draft/})).not.toBeDisabled();
});
it('holds setup writes while its owner-local draft cannot be persisted',async()=>{
 render(<WebsiteBuilderPage/>);await screen.findByText('Your business, live in minutes.');act(()=>useWebsiteBuilderStore.setState({wizardStep:2}));
 const cacheKey='omnisolo_onboarding_owned_v1:'+encodeURIComponent(JSON.stringify([owner.userId,owner.tenantId]))+':website-builder-draft';const saved=localStorage.getItem(cacheKey);const original=localStorage.setItem.bind(localStorage);
 const storage=vi.spyOn(localStorage,'setItem').mockImplementation((name,value)=>{if(name===cacheKey)throw new Error('quota');original(name,value);});
 try{
  fireEvent.change(screen.getByPlaceholderText('What is your business called?'),{target:{value:'Unpersisted B edit'}});await new Promise(resolve=>setTimeout(resolve,1150));
  expect(localStorage.getItem(cacheKey)).toBe(saved);expect(vi.mocked(fetch).mock.calls.some(([,options])=>options?.method==='POST')).toBe(false);expect(await screen.findByText(/could not save your latest builder edits/i)).toBeVisible();
 }finally{storage.mockRestore();}
});
it('holds a delayed website restore when another tab changes its owner-local draft',async()=>{
 const first=render(<WebsiteBuilderPage/>);await screen.findByText('Your business, live in minutes.');act(()=>useWebsiteBuilderStore.setState({wizardStep:2,businessName:'Current local draft'}));first.unmount();
 const key='omnisolo_onboarding_owned_v1:'+encodeURIComponent(JSON.stringify([owner.userId,owner.tenantId]))+':website-builder-draft';let resolve!:(response:Response)=>void;
 vi.mocked(fetch).mockImplementation(async(url,options)=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):String(url).endsWith('/draft')&&!options?.method?new Promise(done=>{resolve=done;}):Response.json({}));
 const second=render(<WebsiteBuilderPage/>);await waitFor(()=>expect(resolve).toBeDefined());const row=JSON.parse(localStorage.getItem(key)!);const newest=JSON.stringify({...row,revision:'newer-other-tab',data:{...row.data,state:{...row.data.state,businessName:'Other tab unsent'}}});localStorage.setItem(key,newest);await act(async()=>resolve(Response.json({wizardState:{businessName:'Old remote'}})));
 expect(await screen.findByRole('alert')).toHaveTextContent(/changed in another view/);expect(localStorage.getItem(key)).toBe(newest);second.unmount();
 vi.mocked(fetch).mockImplementation(async url=>Response.json(String(url).endsWith('/session-identity')?{...owner,expiresAt:Date.now()+60_000}:{wizardState:{businessName:'Old remote'}}));render(<WebsiteBuilderPage/>);expect(await screen.findByDisplayValue('Other tab unsent')).toBeVisible();
});
it('restores a storefront draft only on returning to its verified owner',async()=>{
 owner={userId:'a',tenantId:'ta'};render(<StorefrontBuilderPage/>);const input=await screen.findByPlaceholderText(/mobile dog grooming service/i);fireEvent.change(input,{target:{value:'A held layout description'}});
 owner={userId:'b',tenantId:'tb'};act(()=>notifyQueueIdentityChange());await waitFor(()=>expect(screen.getByPlaceholderText(/mobile dog grooming service/i)).toHaveValue(''));
 owner={userId:'a',tenantId:'ta'};act(()=>notifyQueueIdentityChange());expect(await screen.findByDisplayValue('A held layout description')).toBeVisible();
});

it.each(['network','server500','invalid-body'])('holds an empty local builder when remote restore is %s',async mode=>{
 vi.mocked(fetch).mockImplementation(async url=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(mode==='network')throw new Error('offline');
  return mode==='invalid-body'?new Response('not JSON'):Response.json({error:'unavailable'},{status:500});
 });
 render(<WebsiteBuilderPage/>);
 expect(await screen.findByRole('alert')).toHaveTextContent(/restore|load|unavailable|confirmed/i);
 expect(screen.queryByText('Your business, live in minutes.')).toBeNull();
 act(()=>useWebsiteBuilderStore.setState({businessName:'One incomplete field'}));
 await new Promise(resolve=>setTimeout(resolve,1100));
 expect(vi.mocked(fetch).mock.calls.some(([,options])=>options?.method==='POST')).toBe(false);
});
it('retires a draft spinner after overlapping site saving without clearing a newer owner operation',async()=>{
 render(<WebsiteBuilderPage/>);await screen.findByText('Your business, live in minutes.');act(()=>useWebsiteBuilderStore.setState({wizardStep:2,businessName:'Current draft'}));
 let release!:()=>void;
 vi.mocked(fetch).mockImplementation(async url=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(String(url).endsWith('/publish_draft'))return Response.json({id:'22222222-2222-4222-8222-222222222222',domain:null});
  return new Response(new ReadableStream({start(controller){release=()=>controller.close();}}));
 });
 fireEvent.click(screen.getByRole('button',{name:'Save Draft'}));await waitFor(()=>expect(release).toBeDefined());
 act(()=>useWebsiteBuilderStore.setState({status:'draft',blocks:[{type:'Hero',props:{headline:'Current site'}}]}));
 fireEvent.click(screen.getByText('Save site for publishing'));await screen.findByText(/Site saved/);
 await act(async()=>release());act(()=>useWebsiteBuilderStore.setState({status:'idle'}));
 expect(screen.getByRole('button',{name:'Save Draft'})).not.toBeDisabled();
});

it('holds the second storefront editor and reopens the latest local row after the first closes',async()=>{
 const first=render(<StorefrontBuilderPage/>);const input=await screen.findByPlaceholderText(/mobile dog grooming service/i);fireEvent.change(input,{target:{value:'First editor private pending draft'}});
 const second=render(<StorefrontBuilderPage/>);await waitFor(()=>expect(second.container.querySelector('[role="alert"]')).toHaveTextContent('open in another view'));
 expect(second.container.querySelector('textarea')).toBeNull();expect(vi.mocked(fetch).mock.calls.some(([,options])=>options?.method==='POST')).toBe(false);
 first.unmount();second.unmount();
 render(<StorefrontBuilderPage/>);expect(await screen.findByDisplayValue('First editor private pending draft')).toBeVisible();
});
it('preserves a local website draft when the remote restore is unavailable',async()=>{
 const first=render(<WebsiteBuilderPage/>);await screen.findByText('Your business, live in minutes.');act(()=>useWebsiteBuilderStore.setState({wizardStep:2,businessName:'Local latest'}));first.unmount();
 vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):Response.json({error:'unavailable'},{status:500}));
 render(<WebsiteBuilderPage/>);expect(await screen.findByDisplayValue('Local latest')).toBeVisible();
});
