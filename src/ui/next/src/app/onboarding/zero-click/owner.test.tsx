import {beforeEach as beforeLocks} from 'vitest';
beforeLocks(() => installOnboardingLocks());
import {installOnboardingLocks} from '../testLocks';
import {act,fireEvent,render,screen,waitFor} from '@testing-library/react';
import {afterEach,beforeEach,it,expect,vi} from 'vitest';
import Page from './page';
import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
vi.mock('next/navigation',()=>({useRouter:()=>({push:vi.fn()})}));
vi.mock('../../components/PoweredByOmniSolo',()=>({PoweredByOmniSolo:()=>null}));
let owner={userId:'a',tenantId:'ta'};
beforeEach(()=>{localStorage.clear();notifyQueueIdentityChange();owner={userId:'a',tenantId:'ta'};vi.stubGlobal('fetch',vi.fn(async(url:string,options?:RequestInit)=>Response.json(url.endsWith('/session-identity')?{...owner,expiresAt:Date.now()+60_000}:url.endsWith('/state')&&!options?.method?{chatMessages:owner.userId==='a'?[{role:'user',content:'Owner A private chat'}]:[]}:{})));});
afterEach(()=>vi.unstubAllGlobals());
it('clears visible chat and review on the existing auth invalidation signal',async()=>{
 render(<Page/>);await screen.findByText('Owner A private chat');
 owner={userId:'b',tenantId:'tb'};act(()=>notifyQueueIdentityChange());
 await waitFor(()=>expect(screen.queryByText('Owner A private chat')).toBeNull());
});
it('does not apply or save a late chat response from the previous account',async()=>{
 let resolve!:(response:Response)=>void;
 const base=vi.mocked(fetch).getMockImplementation()!;
 vi.mocked(fetch).mockImplementation((url,options)=>String(url).endsWith('/chat')?new Promise(done=>{resolve=done;}):base(url,options));
 render(<Page/>);const input=await screen.findByPlaceholderText(/e.g. I am a home baker/);
 fireEvent.change(input,{target:{value:'Owner A private prompt'}});fireEvent.click(screen.getByTestId('generate-storefront-btn'));
 await waitFor(()=>expect(resolve).toBeDefined());
 owner={userId:'b',tenantId:'tb'};act(()=>notifyQueueIdentityChange());
 const callsBefore=vi.mocked(fetch).mock.calls.length;
 await act(async()=>resolve(Response.json({reply:'Late A reply',is_complete:true,intake_data:{business_name:'Late A business',initial_products:[{name:'A offer',price:'10.00'}]}})));
 expect(screen.queryByText('Late A reply')).toBeNull();
 const after=vi.mocked(fetch).mock.calls.slice(callsBefore).filter(([,options])=>options?.method==='POST').map(([,options])=>String(options?.body));
 expect(after.join('\n')).not.toContain('Owner A');
});

it('does not revive a protected preparation from a held response body after pagehide',async()=>{
 let close!:()=>void;
 const receipt={preparation_id:'old-prep',status:'prepared',organization_id:'ta',user_id:'a',primary_product_id:'p1',reviewed_request:{},catalog:[{product_id:'p1',name:'Old private catalog',price:'1.00',description:'',variants:[]}]};
 vi.mocked(fetch).mockImplementation(async(url)=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(String(url).endsWith('/state'))return new Response(new ReadableStream({start(controller){close=()=>{controller.enqueue(new TextEncoder().encode(JSON.stringify({preparation:receipt})));controller.close();};}}));
  return Response.json({});
 });
 render(<Page/>);await waitFor(()=>expect(close).toBeDefined());
 act(()=>window.dispatchEvent(new Event('pagehide')));
 await act(async()=>close());
 expect(screen.queryByText('Old private catalog: 1.00')).toBeNull();expect(screen.queryByRole('button',{name:/Launch My Store/i})).toBeNull();
});

it('gives the icon-only chat submit button a stable accessible purpose',async()=>{
 render(<Page/>);await screen.findByPlaceholderText(/e.g. I am a home baker/);
 const send=screen.getByRole('button',{name:'Send message'});expect(send).toBeDisabled();
 fireEvent.change(screen.getByPlaceholderText(/e.g. I am a home baker/),{target:{value:'Describe my studio'}});expect(send).toBeEnabled();
});
