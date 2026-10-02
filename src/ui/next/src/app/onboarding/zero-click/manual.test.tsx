import { act,cleanup,fireEvent,render,screen,waitFor } from '@testing-library/react';
import { afterEach,beforeEach,expect,it,vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installOnboardingLocks } from '../testLocks';
import { initializeOnboardingDraft,useOnboardingStore } from '../store';
import { OnboardingChatAgent } from './components/OnboardingChatAgent';
const navigation=vi.hoisted(()=>({push:vi.fn()}));vi.mock('next/navigation',()=>({useRouter:()=>navigation}));
let owner={userId:'owner-a',tenantId:'tenant-a'};let chat:()=>Promise<Response>;
const unavailable={error:'onboarding_ai_unconfigured',message:'AI-assisted setup is unavailable because no model provider is configured. Review and enter your business details manually.'};
beforeEach(()=>{localStorage.clear();notifyQueueIdentityChange();installOnboardingLocks();owner={userId:'owner-a',tenantId:'tenant-a'};navigation.push.mockReset();chat=async()=>Response.json(unavailable,{status:503});vi.stubGlobal('fetch',vi.fn(async(url:string,init?:RequestInit)=>url.endsWith('/session-identity')?Response.json({...owner,expiresAt:Date.now()+60000}):url.endsWith('/chat')?chat():url.endsWith('/state')&&init?.method==='POST'?new Response(null,{status:204}):Response.json({})));});
afterEach(()=>{cleanup();notifyQueueIdentityChange();vi.unstubAllGlobals();});
const mutations=()=>vi.mocked(fetch).mock.calls.filter(([url,init])=>init?.method==='POST'&&(/\/(start|launch|start_zero_click)$/.test(String(url))));
async function send(){render(<OnboardingChatAgent onComplete={vi.fn()}/>);const input=await screen.findByPlaceholderText(/e.g. I am a home baker/);fireEvent.change(input,{target:{value:'I restore bicycles in Portland'}});fireEvent.click(screen.getByRole('button',{name:'Send message'}));}
it('offers explicit manual review only for the real unconfigured503 and preserves the prompt without invented offers',async()=>{
 await send();const manual=await screen.findByRole('button',{name:'Review Details Manually'});expect(navigation.push).not.toHaveBeenCalled();expect(mutations()).toHaveLength(0);
 await act(async()=>fireEvent.click(manual));await waitFor(()=>expect(navigation.push).toHaveBeenCalledWith('/onboarding'));
 expect(useOnboardingStore.getState()).toMatchObject({step:2,bio:'I restore bicycles in Portland',businessDescription:'I restore bicycles in Portland',firstProductName:'',firstProductPrice:''});expect(mutations()).toHaveLength(0);
});
it('keeps arbitrary503 errors out of manual-success handling',async()=>{
 chat=async()=>Response.json({error:'database_unavailable'},{status:503});await send();await screen.findByText('Failed to communicate with setup agent');expect(screen.queryByRole('button',{name:'Review Details Manually'})).toBeNull();expect(navigation.push).not.toHaveBeenCalled();expect(mutations()).toHaveLength(0);
});
it('keeps prior owner-reviewed local fields when adding the new manual prompt',async()=>{
 await initializeOnboardingDraft();useOnboardingStore.setState({businessName:'Owner workshop',businessDescription:'My reviewed description',firstProductName:'Repair appointment',firstProductPrice:'45.00'});
 await send();const manual=await screen.findByRole('button',{name:'Review Details Manually'});await act(async()=>fireEvent.click(manual));
 await waitFor(()=>expect(navigation.push).toHaveBeenCalledWith('/onboarding'));expect(useOnboardingStore.getState()).toMatchObject({businessName:'Owner workshop',firstProductName:'Repair appointment',firstProductPrice:'45.00'});expect(useOnboardingStore.getState().businessDescription).toContain('My reviewed description');expect(useOnboardingStore.getState().bio).toBe('I restore bicycles in Portland');
});
it('retires the manual action immediately on auth change',async()=>{
 await send();await screen.findByRole('button',{name:'Review Details Manually'});owner={userId:'owner-b',tenantId:'tenant-b'};act(()=>notifyQueueIdentityChange());expect(screen.queryByRole('button',{name:'Review Details Manually'})).toBeNull();await act(async()=>{});expect(navigation.push).not.toHaveBeenCalled();expect(mutations()).toHaveLength(0);
});
it('holds navigation when the owner-bound local handoff cannot be persisted',async()=>{
 await send();await screen.findByRole('button',{name:'Review Details Manually'});const original=vi.mocked(localStorage.setItem).getMockImplementation();vi.mocked(localStorage.setItem).mockImplementation(()=>{throw new Error('Storage unavailable');});
 try {await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Review Details Manually'})));await screen.findByText(/could not be saved on this device/);expect(navigation.push).not.toHaveBeenCalled();expect(screen.getByText('I restore bicycles in Portland',{selector:'div'})).toBeVisible();expect(mutations()).toHaveLength(0);}finally{vi.mocked(localStorage.setItem).mockImplementation(original!);}
});

it('allows an explicit edited manual description while retaining the original conversation',async()=>{
 await send();await screen.findByRole('button',{name:'Review Details Manually'});fireEvent.change(screen.getByLabelText('Description for manual setup'),{target:{value:'Reviewed bicycle restoration services'}});
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Review Details Manually'})));await waitFor(()=>expect(navigation.push).toHaveBeenCalledWith('/onboarding'));
 expect(useOnboardingStore.getState().bio).toBe('Reviewed bicycle restoration services');expect(screen.getByText('I restore bicycles in Portland',{selector:'div'})).toBeVisible();expect(mutations()).toHaveLength(0);
});
