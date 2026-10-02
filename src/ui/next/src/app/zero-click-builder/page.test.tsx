import {act,cleanup,fireEvent,render,screen,waitFor} from '@testing-library/react';
import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
import {installOnboardingLocks} from '../onboarding/testLocks';
import {useOnboardingStore} from '../onboarding/store';
import Page from './page';
const navigation=vi.hoisted(()=>({push:vi.fn()}));vi.mock('next/navigation',()=>({useRouter:()=>navigation}));
vi.mock('../components/PoweredByOmniSolo',()=>({PoweredByOmniSolo:()=>null}));
let identity:()=>Promise<Response>;let chat:()=>Promise<Response>;
beforeEach(()=>{localStorage.clear();notifyQueueIdentityChange();installOnboardingLocks();navigation.push.mockReset();identity=async()=>Response.json({userId:'owner-a',tenantId:'tenant-a',expiresAt:Date.now()+60000});chat=async()=>Response.json({error:'onboarding_ai_unconfigured'},{status:503});vi.stubGlobal('fetch',vi.fn(async(url:string,init?:RequestInit)=>url.endsWith('/session-identity')?identity():url.endsWith('/chat')?chat():url.endsWith('/start_zero_click')?Response.json({success:false}):url.endsWith('/state')&&init?.method==='POST'?new Response(null,{status:204}):Response.json({})));vi.spyOn(console,'error').mockImplementation(()=>{});});
afterEach(()=>{cleanup();notifyQueueIdentityChange();vi.unstubAllGlobals();vi.restoreAllMocks();});
async function send(){const input=await screen.findByRole('textbox');fireEvent.change(input,{target:{value:'I repair bicycles in Portland'}});fireEvent.click(screen.getByRole('button',{name:/Generate Store|Send message/}));}
it('verifies the owner before enabling the maintained builder entry point',async()=>{
 let resolve!:(value:Response)=>void;identity=()=>new Promise(done=>{resolve=done;});const view=render(<Page/>);
 expect(screen.queryByRole('textbox')).toBeNull();expect(vi.mocked(fetch).mock.calls.filter(([,init])=>init?.method==='POST')).toHaveLength(0);view.unmount();if(resolve)await act(async()=>resolve(Response.json({userId:'owner-a',tenantId:'tenant-a',expiresAt:Date.now()+60000})));
});
it('never treats an incomplete200 as a live storefront or a completed preparation',async()=>{
 chat=async()=>Response.json({reply:'A response without a usable catalogue',is_complete:true});render(<Page/>);await send();
 await waitFor(()=>expect(screen.queryByText('Your business is live!')||screen.queryByText(/setup draft is incomplete/)).not.toBeNull());
 expect(screen.queryByText('Your business is live!')).toBeNull();expect(screen.queryByTitle('Live Storefront Preview')).toBeNull();expect(screen.queryByRole('button',{name:/Launch My Store/})).toBeNull();expect(vi.mocked(fetch).mock.calls.filter(([url])=>String(url).endsWith('/start_zero_click'))).toHaveLength(0);
});
it('uses the owned manual review continuation after the exact unconfigured response',async()=>{
 render(<Page/>);await send();const manual=await screen.findByRole('button',{name:'Review Details Manually'});await act(async()=>fireEvent.click(manual));await waitFor(()=>expect(navigation.push).toHaveBeenCalledWith('/onboarding'));
 expect(useOnboardingStore.getState()).toMatchObject({step:2,bio:'I repair bicycles in Portland',firstProductName:'',firstProductPrice:''});expect(vi.mocked(fetch).mock.calls.filter(([url])=>/\/(start|launch|start_zero_click)$/.test(String(url)))).toHaveLength(0);
});
