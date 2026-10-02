import {act,fireEvent,render,screen,waitFor} from '@testing-library/react';
import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
import {installOnboardingLocks} from './testLocks';
import {initializeOnboardingDraft,useOnboardingStore} from './store';
import Page from './page';
vi.mock('@/lib/sync/queueIdentity',async original=>({...await original<typeof import('@/lib/sync/queueIdentity')>(),readQueueOwner:vi.fn(async()=>({userId:'owner-a',tenantId:'tenant-a'}))}));
beforeEach(async()=>{localStorage.clear();notifyQueueIdentityChange();installOnboardingLocks();await initializeOnboardingDraft();useOnboardingStore.setState({step:0,bio:'',isLoading:false,error:''});vi.stubGlobal('fetch',vi.fn(async(url:string)=>Response.json(url.endsWith('/chat')?{reply:'Please review the next detail',is_complete:false}:{})));});
afterEach(()=>vi.unstubAllGlobals());
it.each(['',' \n '])('holds empty onboarding chat input %j',async value=>{
 render(<Page/>);const input=await screen.findByPlaceholderText('Type a message...');fireEvent.change(input,{target:{value}});const send=screen.getByRole('button',{name:'Send'});expect(send).toBeDisabled();fireEvent.click(send);expect(vi.mocked(fetch).mock.calls.filter(([url])=>String(url).endsWith('/chat'))).toHaveLength(0);
});
it('enables actual chat text and sends the existing reviewed chat request',async()=>{
 render(<Page/>);const input=await screen.findByPlaceholderText('Type a message...');fireEvent.change(input,{target:{value:'I repair bicycles'}});const send=screen.getByRole('button',{name:'Send'});expect(send).toBeEnabled();await act(async()=>fireEvent.click(send));await screen.findByText('Please review the next detail');await waitFor(()=>expect(vi.mocked(fetch).mock.calls.filter(([url])=>String(url).endsWith('/chat'))).toHaveLength(1));expect(send).toBeDisabled();
});

it('keeps the existing image-only request eligible without inventing chat text',async()=>{
 vi.stubGlobal('prompt',vi.fn(()=> 'https://example.test/owner-image.png'));render(<Page/>);await screen.findByPlaceholderText('Type a message...');fireEvent.click(screen.getByRole('button',{name:'Upload Image'}));const send=screen.getByRole('button',{name:'Send'});expect(send).toBeEnabled();await act(async()=>fireEvent.click(send));await screen.findByText('Please review the next detail');const call=vi.mocked(fetch).mock.calls.find(([url])=>String(url).endsWith('/chat'));expect(String(call?.[1]?.body)).toContain('https://example.test/owner-image.png');expect(send).toBeDisabled();
});
