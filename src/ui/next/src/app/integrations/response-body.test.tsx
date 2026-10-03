import {act,cleanup,fireEvent,render,screen,waitFor,within} from '@testing-library/react';
import {afterEach,expect,it,vi} from 'vitest';
import Page from './page';
vi.mock('../components/AppShell',()=>({AppShell:({children}:{children:import('react').ReactNode})=>children}));
vi.mock('./ProviderConnections',()=>({default:()=>null}));
const navigation=vi.hoisted(()=>({push:vi.fn()}));vi.mock('next/navigation',()=>({useRouter:()=>navigation}));
afterEach(()=>{cleanup();vi.unstubAllGlobals();navigation.push.mockReset();});
async function open(){render(<Page/>);const card=screen.getByRole('heading',{name:'Twilio for WhatsApp'}).closest('div.rounded-2xl')!;fireEvent.click(within(card as HTMLElement).getByRole('button',{name:'Connect'}));fireEvent.change(screen.getByLabelText('Account SID'),{target:{value:'synthetic-account'}});fireEvent.change(screen.getByLabelText('Auth Token'),{target:{value:'synthetic-token'}});fireEvent.change(screen.getByLabelText('WhatsApp Phone Number'),{target:{value:'+12025550100'}});fireEvent.click(screen.getByRole('button',{name:'Save & Connect'}));}
it('consumes the finite501 body before reporting failure and keeps the integration disconnected',async()=>{
 let close!:()=>void;const response=new Response(new ReadableStream({start(controller){close=()=>{controller.enqueue(new TextEncoder().encode(JSON.stringify({success:false,status:'pending_verification',usable:false})));controller.close();};}}),{status:501,headers:{'content-type':'application/json'}});
 vi.stubGlobal('fetch',vi.fn(async(url:string)=>url.endsWith('/whatsapp/connect')?response:Response.json({success:true,integrations:[]})));
 await open();await waitFor(()=>expect(response.bodyUsed).toBe(true));expect(screen.queryByText('Failed to connect Twilio for WhatsApp.')).toBeNull();
 await act(async()=>close());expect(await screen.findByText('Failed to connect Twilio for WhatsApp.')).toBeVisible();expect(screen.getByRole('heading',{name:'Connect Twilio for WhatsApp API'})).toBeVisible();expect(screen.queryByText('Twilio for WhatsApp connected.')).toBeNull();expect(navigation.push).not.toHaveBeenCalled();
});
it('keeps an invalid finite response unconnected after body parsing fails',async()=>{
 const response=new Response('not-json',{status:501,headers:{'content-type':'application/json'}});vi.stubGlobal('fetch',vi.fn(async(url:string)=>url.endsWith('/whatsapp/connect')?response:Response.json({success:true,integrations:[]})));
 await open();expect(await screen.findByText('Failed to connect Twilio for WhatsApp.')).toBeVisible();expect(response.bodyUsed).toBe(true);expect(navigation.push).not.toHaveBeenCalled();
});
