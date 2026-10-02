import {act,render,screen,waitFor} from '@testing-library/react';
import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import KitchenView from './page';
import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
vi.mock('../components/AppShell',()=>({AppShell:({children}:React.PropsWithChildren)=><main>{children}</main>}));
const sync=vi.hoisted(()=>({enqueue:vi.fn(),getQueueLength:vi.fn(async()=>0)}));
vi.mock('../../lib/sync/SyncManager',()=>({SyncManager:{getInstance:()=>sync}}));
let owner={userId:'a',tenantId:'ta'};
beforeEach(()=>{localStorage.clear();notifyQueueIdentityChange();vi.clearAllMocks();sync.enqueue.mockReset().mockResolvedValue(undefined);owner={userId:'a',tenantId:'ta'};vi.stubGlobal('fetch',vi.fn(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):Response.json({error:'unavailable'},{status:500})));});
afterEach(()=>vi.unstubAllGlobals());
it('holds unassigned legacy kitchen data without exposing it after another account verifies',async()=>{
 const orders=JSON.stringify([{id:'private-order-a',customer_name:'Private A customer',status:'pending'}]);const menu=JSON.stringify([{id:'private-menu-a',name:'Private A menu',is_sold_out:false}]);
 localStorage.setItem('kds_orders_cache',orders);localStorage.setItem('kds_menu_cache',menu);owner={userId:'b',tenantId:'tb'};
 render(<KitchenView/>);await waitFor(()=>expect(vi.mocked(fetch).mock.calls.some(([url])=>String(url).endsWith('/inventory'))).toBe(true));
 expect(screen.queryByText(/Private A customer/)).toBeNull();expect(screen.queryByText(/Private A menu/)).toBeNull();expect(localStorage.getItem('kds_orders_cache')).toBe(orders);expect(localStorage.getItem('kds_menu_cache')).toBe(menu);
});
it('clears private visible kitchen records immediately on the canonical account change',async()=>{
 vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):Response.json(String(url).endsWith('/orders')?{orders:[{id:'order-a',customer_name:'Private A customer',status:'pending',updated_at:'2026-10-01T00:00:00Z'}]}:{inventory:[{id:'menu-a',name:'Private A menu',is_sold_out:false,updated_at:'2026-10-01T00:00:00Z'}]}));
 render(<KitchenView/>);await screen.findByText(/Private A customer/);await screen.findByText('Private A menu');
 owner={userId:'b',tenantId:'tb'};vi.mocked(fetch).mockImplementation(()=>new Promise(()=>{}));act(()=>notifyQueueIdentityChange());
 expect(screen.queryByText(/Private A customer/)).toBeNull();expect(screen.queryByText('Private A menu')).toBeNull();
});
it('restores only the returning verified owner snapshot after failed reads',async()=>{
 let unavailable=false;vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):unavailable?Response.json({error:'unavailable'},{status:500}):Response.json(String(url).endsWith('/orders')?{orders:[{id:'order-a',customer_name:'Private A customer',status:'pending',updated_at:'2026-10-01T00:00:00Z'}]}:{inventory:[]}));
 render(<KitchenView/>);await screen.findByText(/Private A customer/);unavailable=true;owner={userId:'b',tenantId:'tb'};act(()=>notifyQueueIdentityChange());
 await waitFor(()=>expect(screen.queryByText(/Private A customer/)).toBeNull());owner={userId:'a',tenantId:'ta'};act(()=>notifyQueueIdentityChange());expect(await screen.findByText(/Private A customer/)).toBeVisible();expect(screen.getByText(/saved.*orders|cached.*orders/i)).toBeVisible();
});
it('ignores an earlier owner response body after the next account loads',async()=>{
 let finishA!:()=>void;
 vi.mocked(fetch).mockImplementation(async url=>{
  if(String(url).endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(String(url).endsWith('/inventory'))return Response.json({inventory:[]});
  if(owner.userId==='a')return new Response(new ReadableStream({start(controller){finishA=()=>{controller.enqueue(new TextEncoder().encode(JSON.stringify({orders:[{id:'order-a',customer_name:'Late private A',status:'pending'}]})));controller.close();};}}));
  return Response.json({orders:[{id:'order-b',customer_name:'Current B customer',status:'pending',updated_at:'2026-10-01T00:00:00Z'}]});
 });
 render(<KitchenView/>);await waitFor(()=>expect(finishA).toBeDefined());owner={userId:'b',tenantId:'tb'};act(()=>notifyQueueIdentityChange());await screen.findByText(/Current B customer/);await act(async()=>finishA());expect(screen.queryByText(/Late private A/)).toBeNull();
});
it('keeps storage failure visible without pretending a fresh snapshot was cached',async()=>{
 const original=localStorage.setItem.bind(localStorage);const storage=vi.spyOn(localStorage,'setItem').mockImplementation((key,value)=>{if(key.includes(':kitchen-'))throw new Error('quota');original(key,value);});
 vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):Response.json(String(url).endsWith('/orders')?{orders:[{id:'order-a',customer_name:'Current A customer',status:'pending',updated_at:'2026-10-01T00:00:00Z'}]}:{inventory:[]}));
 try{render(<KitchenView/>);await screen.findByText(/Current A customer/);expect(await screen.findByText(/could not.*cache|could not.*save.*device/i)).toBeVisible();}finally{storage.mockRestore();}
});
it('passes the verified view owner and exact observed token into durable enqueue',async()=>{
 const timestamp='2026-10-01T00:00:00.123456+00:00';vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):Response.json(String(url).endsWith('/orders')?{orders:[{id:'order-a',customer_name:'Current A customer',status:'pending',updated_at:timestamp}]}:{inventory:[]}));
 render(<KitchenView/>);const button=await screen.findByRole('button',{name:'Mark Ready & Notify'});await act(async()=>button.click());expect(sync.enqueue).toHaveBeenCalledWith(expect.objectContaining({payload:expect.objectContaining({expected_updated_at:timestamp,expected_status:'pending'})}),owner);
});
it('holds malformed owned cache bytes when fresh data is unavailable',async()=>{
 const key='omnisolo_onboarding_owned_v1:'+encodeURIComponent(JSON.stringify(['a','ta']))+':kitchen-orders-v1';localStorage.setItem(key,'corrupted original');render(<KitchenView/>);
 expect(await screen.findByRole('alert')).toHaveTextContent(/could not be read|remains held/i);expect(localStorage.getItem(key)).toBe('corrupted original');expect(screen.queryByText('No active orders')).toBeNull();
});
it('does not describe a failed uncached read as an authoritative empty kitchen',async()=>{
 render(<KitchenView/>);expect(await screen.findByText('Orders are unavailable.')).toBeVisible();expect(screen.queryByText('No active orders')).toBeNull();
});
it('does not fall back to private cached records after an authentication rejection',async()=>{
 const key='omnisolo_onboarding_owned_v1:'+encodeURIComponent(JSON.stringify(['a','ta']))+':kitchen-orders-v1';localStorage.setItem(key,JSON.stringify({format:1,records:[{id:'a',customer_name:'Private cached A',status:'pending'}]}));
 vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):new Response('{}',{status:401}));
 render(<KitchenView/>);expect(await screen.findByRole('alert')).toHaveTextContent(/session|verified/i);expect(screen.queryByText(/Private cached A/)).toBeNull();expect(localStorage.getItem(key)).toContain('Private cached A');
});
it('does not apply an old enqueue failure after the next account loads',async()=>{
 let failA!:(cause:Error)=>void;sync.enqueue.mockImplementationOnce(()=>new Promise((_,reject)=>{failA=reject;}));
 vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):Response.json(String(url).endsWith('/orders')?{orders:[{id:'order-'+owner.userId,customer_name:owner.userId==='a'?'A customer':'B customer',status:'pending',updated_at:'2026-10-01T00:00:00Z'}]}:{inventory:[]}));
 render(<KitchenView/>);const button=await screen.findByRole('button',{name:'Mark Ready & Notify'});await act(async()=>button.click());await waitFor(()=>expect(failA).toBeDefined());
 owner={userId:'b',tenantId:'tb'};act(()=>notifyQueueIdentityChange());await screen.findByText(/B customer/);await act(async()=>failA(new Error('A storage failed')));
 expect(screen.queryByText(/A customer/)).toBeNull();expect(screen.queryByText(/This change could not be saved/)).toBeNull();expect(screen.getByRole('button',{name:'Mark Ready & Notify'})).toBeVisible();
});
it('keeps a newer account inventory operation locked when an earlier completion arrives',async()=>{
 let failA!:(cause:Error)=>void;let finishB!:()=>void;
 sync.enqueue.mockImplementationOnce(()=>new Promise((_,reject)=>{failA=reject;})).mockImplementationOnce(()=>new Promise<void>(resolve=>{finishB=resolve;}));
 vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60_000}):Response.json(String(url).endsWith('/orders')?{orders:[]}:{inventory:[{id:'shared-product',name:owner.userId==='a'?'A menu':'B menu',is_sold_out:false,updated_at:'2026-10-01T00:00:00Z'}]}));
 render(<KitchenView/>);await screen.findByText('A menu');act(()=>screen.getByRole('button',{name:'Mark Sold Out'}).click());await waitFor(()=>expect(failA).toBeDefined());
 owner={userId:'b',tenantId:'tb'};act(()=>notifyQueueIdentityChange());await screen.findByText('B menu');act(()=>screen.getByRole('button',{name:'Mark Sold Out'}).click());await waitFor(()=>expect(finishB).toBeDefined());
 await act(async()=>failA(new Error('old storage failure')));expect(screen.queryByText(/This change could not be saved/)).toBeNull();act(()=>screen.getByRole('button',{name:/^Sold Out$/}).click());expect(sync.enqueue).toHaveBeenCalledTimes(2);
 await act(async()=>finishB());expect(screen.getByRole('button',{name:/^Sold Out$/})).toBeVisible();
});
it('does not overwrite a newer same-owner snapshot when an unmounted view finishes late', async () => {
 let finishOld!:()=>void; let ordersRead=0;
 vi.mocked(fetch).mockImplementation(async url=>{
  if(String(url).endsWith('/session-identity')) return Response.json({...owner,expiresAt:Date.now()+60_000});
  if(String(url).endsWith('/inventory')) return Response.json({inventory:[]});
  if(ordersRead++===0) return new Response(new ReadableStream({start(controller){finishOld=()=>{controller.enqueue(new TextEncoder().encode(JSON.stringify({orders:[{id:'old-order',customer_name:'Earlier snapshot',status:'pending'}]})));controller.close();};}}));
  return Response.json({orders:[{id:'new-order',customer_name:'Latest snapshot',status:'pending'}]});
 });
 const first=render(<KitchenView/>);await waitFor(()=>expect(finishOld).toBeDefined());first.unmount();render(<KitchenView/>);await screen.findByText(/Latest snapshot/);
 const key='omnisolo_onboarding_owned_v1:'+encodeURIComponent(JSON.stringify(['a','ta']))+':kitchen-orders-v1';
 const latest=localStorage.getItem(key);expect(latest).toContain('Latest snapshot');
 await act(async()=>finishOld());expect(localStorage.getItem(key)).toBe(latest);
});
