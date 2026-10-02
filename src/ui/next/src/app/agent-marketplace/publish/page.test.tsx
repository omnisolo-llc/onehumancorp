import { afterEach,beforeEach,expect,it,vi } from 'vitest';
import { act,cleanup,fireEvent,render,screen,waitFor } from '@testing-library/react';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { fields,marketplaceBackend } from '../marketplace.test-support';
import Page from './page';
vi.mock('next/navigation',()=>({useRouter:()=>({push:vi.fn()})}));
let backend:ReturnType<typeof marketplaceBackend>;
beforeEach(()=>{backend=marketplaceBackend();});afterEach(()=>{cleanup();notifyQueueIdentityChange();vi.unstubAllGlobals();});
async function ready(){await waitFor(()=>expect(screen.getByLabelText('Agent Name')).toBeEnabled());}
it('renders the existing form and verifies access before enabling edits',async()=>{
 render(<Page/>);expect(screen.getByRole('heading',{name:'Publish New Agent'})).toBeVisible();expect(screen.getByLabelText('Agent Name')).toBeDisabled();await ready();
 for(const name of['Agent Name','Description','Role','System Prompt'])expect(screen.getByLabelText(name)).toBeVisible();expect(backend.posts()).toHaveLength(0);
});
it('retains every entered field during review without sending a publication',async()=>{
 render(<Page/>);await ready();for(const[label,value]of Object.entries({'Agent Name':fields.name,Description:fields.description,Role:fields.role,'System Prompt':fields.system_prompt}))fireEvent.change(screen.getByLabelText(label),{target:{value}});
 fireEvent.submit(screen.getByRole('button',{name:'Review Publication'}).closest('form')!);expect(backend.posts()).toHaveLength(0);
 fireEvent.click(screen.getByRole('button',{name:'Back to Editing'}));expect(screen.getByLabelText('System Prompt')).toHaveValue(fields.system_prompt);
});
it('prevents default navigation for programmatic submission without publishing',async()=>{
 render(<Page/>);await ready();const event=new Event('submit',{bubbles:true,cancelable:true});act(()=>screen.getByRole('button',{name:'Review Publication'}).closest('form')!.dispatchEvent(event));
 expect(event.defaultPrevented).toBe(true);expect(backend.posts()).toHaveLength(0);
});
it('does not send private edits during rendering and restores them only for the verified owner',async()=>{
 let view=render(<Page/>);await ready();fireEvent.change(screen.getByLabelText('System Prompt'),{target:{value:'Unsubmitted private draft'}});expect(backend.posts()).toHaveLength(0);
 view.unmount();await act(async()=>{});view=render(<Page/>);await ready();expect(screen.getByLabelText('System Prompt')).toHaveValue('Unsubmitted private draft');expect(backend.posts()).toHaveLength(0);view.unmount();
});
