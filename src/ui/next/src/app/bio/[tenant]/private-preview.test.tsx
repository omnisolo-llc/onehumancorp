import {act,render,screen} from '@testing-library/react';
import {beforeEach,afterEach,it,expect,vi} from 'vitest';
import Page from './page';
vi.mock('next/navigation',()=>({useParams:()=>({tenant:'owned / & tenant'})}));
const owner={userId:'member-a',tenantId:'owned / & tenant',expiresAt:Date.now()+60000};
const profile={store_name:'Private owned profile',bio:'Private details',theme:'light',links:[{title:'Actual link',url:'https://example.test/actual'}],remove_branding:false};
let read:()=>Promise<Response>;
beforeEach(()=>{read=async()=>Response.json(profile);vi.stubGlobal('fetch',vi.fn(async url=>url==='/api/v1/auth/session-identity'?Response.json(owner):read()));});
afterEach(()=>vi.unstubAllGlobals());
it('renders the actual own-tenant private configuration',async()=>{render(<Page/>);expect(await screen.findByRole('heading',{name:profile.store_name})).toBeVisible();expect(screen.getByRole('link',{name:'Actual link'})).toHaveAttribute('href','https://example.test/actual');});
it.each([401,403,404,503])('HTTP%s does not become an invented profile',async status=>{
 read=async()=>new Response('',{status});render(<Page/>);const message=await screen.findByRole('alert');
 expect(message).toHaveTextContent(status===404?/No saved private profile/:status===401||status===403?/owning business/:/unavailable/);
 expect(screen.queryByRole('heading',{name:'owned / & tenant'})).not.toBeInTheDocument();expect(screen.queryByRole('link',{name:'Visit Store'})).not.toBeInTheDocument();
});
it('account retirement removes previously loaded private content',async()=>{
 render(<Page/>);await screen.findByRole('heading',{name:profile.store_name});act(()=>window.dispatchEvent(new Event('omnisolo_auth_changed')));
 expect(screen.queryByRole('heading',{name:profile.store_name})).not.toBeInTheDocument();expect(screen.getByRole('alert')).toHaveTextContent(/session changed/i);
});
it('retirement rejects a late private response body',async()=>{
 let resolve!:(value:Response)=>void;read=()=>new Promise(r=>{resolve=r;});render(<Page/>);await act(async()=>{});
 act(()=>window.dispatchEvent(new Event('omnisolo_auth_changed')));await act(async()=>resolve(Response.json(profile)));
 expect(screen.queryByRole('heading',{name:profile.store_name})).not.toBeInTheDocument();
});
