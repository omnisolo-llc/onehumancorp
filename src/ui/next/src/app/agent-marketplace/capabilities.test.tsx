import { act, cleanup, fireEvent, render, screen, within } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { fields, marketplaceBackend } from './marketplace.test-support';
import MarketplacePage from './page';
import PublishPage from './publish/page';
const navigation=vi.hoisted(()=>({push:vi.fn()}));vi.mock('next/navigation',()=>({useRouter:()=>navigation}));
let backend:ReturnType<typeof marketplaceBackend>;
beforeEach(()=>{backend=marketplaceBackend();navigation.push.mockReset();backend.fetch.mockImplementation(async(url,options)=>options?.method==='POST'?Response.json({id:'fabricated-success'}):backend.route(url,options));});
afterEach(()=>{cleanup();notifyQueueIdentityChange();vi.unstubAllGlobals();});
it('never presents a local toggle or incomplete receipt as a recorded installation',async()=>{
 render(<MarketplacePage/>);await screen.findByText('Senior Rust Developer');fireEvent.click(within(screen.getByRole('article',{name:'Senior Rust Developer'})).getByRole('button',{name:'Install Agent'}));
 expect(screen.queryByRole('button',{name:'Installed'})).toBeNull();expect(backend.posts()).toHaveLength(0);
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Confirm Inactive Installation'})));
 await screen.findByRole('button',{name:'Check Saved Status'});expect(screen.queryByRole('button',{name:'Installed'})).toBeNull();expect(screen.queryByText(/Agent installed successfully/)).toBeNull();
});
it('retains full publication fields without a false success when the actual receipt is incomplete',async()=>{
 render(<PublishPage/>);await screen.findByText('Verified marketplace access. Review all fields before submitting.');
 for(const[label,value]of Object.entries({'Agent Name':fields.name,Description:fields.description,Role:fields.role,'System Prompt':fields.system_prompt}))fireEvent.change(screen.getByLabelText(label),{target:{value}});
 fireEvent.click(screen.getByRole('button',{name:'Review Publication'}));expect(backend.posts()).toHaveLength(0);
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Publish Publicly'})));await screen.findByRole('button',{name:'Check Saved Status'});
 expect(navigation.push).not.toHaveBeenCalled();expect(screen.getByLabelText('System Prompt')).toHaveValue(fields.system_prompt);expect(backend.posts()).toHaveLength(1);
});
