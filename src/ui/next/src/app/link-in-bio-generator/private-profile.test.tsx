import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, afterEach, it, expect, vi } from 'vitest';
import Page from './page';
vi.mock('next/navigation', () => ({ useRouter: () => ({back:vi.fn()}) }));
vi.mock('../components/PoweredByOmniSolo', () => ({PoweredByOmniSolo:()=>null}));
const owner={userId:'member-a',tenantId:'tenant-a / &'};
const profile={store_name:'Private business A',bio:'Private draft',theme:'dark',links:[{id:'1',title:'Owned link',url:'https://example.test/owned'}],remove_branding:false};
const deferred=<T,>()=>{let resolve!:(v:T)=>void;const promise=new Promise<T>(r=>{resolve=r;});return{promise,resolve};};
let identity:()=>Promise<Response>;let read:()=>Promise<Response>;let save:()=>Promise<Response>;
const posts=()=>vi.mocked(fetch).mock.calls.filter(([,i])=>i?.method==='POST');
beforeEach(()=>{
 localStorage.clear();localStorage.setItem('business_display_name','foreign-display-name');
 identity=async()=>Response.json({...owner,expiresAt:Date.now()+60000});read=async()=>Response.json(profile);save=async()=>new Response('',{status:200});
 vi.stubGlobal('fetch',vi.fn(async(u,i)=>u==='/api/v1/auth/session-identity'?identity():i?.method==='POST'?save():read()));
 Object.defineProperty(navigator,'clipboard',{configurable:true,value:{writeText:vi.fn().mockResolvedValue(undefined)}});
});
afterEach(()=>{vi.unstubAllGlobals();vi.restoreAllMocks();});
async function ready(){await screen.findByDisplayValue(profile.store_name);}
it('holds private configuration controls until real identity and load complete',async()=>{
 const pending=deferred<Response>();identity=()=>pending.promise;const view=render(<Page/>);
 expect(screen.getByLabelText('Store / Creator Name Business name')).toBeDisabled();expect(posts()).toHaveLength(0);
 view.unmount();await act(async()=>pending.resolve(Response.json({...owner,expiresAt:Date.now()+60000})));
});
it('uses verified encoded tenant for private load and save, never the local display name',async()=>{
 render(<Page/>);await ready();
 expect(vi.mocked(fetch).mock.calls.some(([u])=>u===`/api/v1/growth/link-in-bio/${encodeURIComponent(owner.tenantId)}`)).toBe(true);
 const button=screen.getByRole('button',{name:/Save (?:private configuration|& Publish)/});await act(async()=>fireEvent.click(button));
 expect(posts()).toHaveLength(1);const init=posts()[0][1]!;const headers=new Headers(init.headers);
 expect(headers.get('x-ohc-expected-user')).toBe(owner.userId);expect(headers.get('x-ohc-expected-tenant')).toBe(owner.tenantId);
 expect(JSON.parse(init.body as string).tenant_id).toBe(owner.tenantId);
 expect(screen.getByRole('status',{name:'Private profile status'})).toHaveTextContent(/saved.*private/i);
});
it('does not label private persistence as publication',async()=>{
 render(<Page/>);await ready();expect(screen.queryByText('Save & Publish')).not.toBeInTheDocument();
 expect(screen.getByRole('status',{name:'Private profile status'})).toHaveTextContent('Public publication is not available');
});
it('a202 result is not a completed save',async()=>{
 save=async()=>new Response('',{status:202});render(<Page/>);await ready();
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:/Save (?:private configuration|& Publish)/})));
 expect(screen.queryByText('Saved! ✅')).not.toBeInTheDocument();
 expect(screen.getByRole('status',{name:'Private profile status'})).toHaveTextContent(/not be confirmed/i);
});
it('copy waits for actual clipboard success using the verified private URL',async()=>{
 const pending=deferred<void>();vi.mocked(navigator.clipboard.writeText).mockImplementation(()=>pending.promise);render(<Page/>);await ready();
 fireEvent.click(screen.getByRole('button',{name:/Copy (?:saved private preview link|Link)/}));expect(screen.queryByText('Copied URL!')).not.toBeInTheDocument();
 await act(async()=>pending.resolve());expect(navigator.clipboard.writeText).toHaveBeenCalledWith(`${window.location.origin}/bio/${encodeURIComponent(owner.tenantId)}`);
});
it.each(['auth','storage','pagehide'])('%s retirement hides private fields and prevents later writes',async kind=>{
 render(<Page/>);await ready();act(()=>window.dispatchEvent(kind==='auth'?new Event('omnisolo_auth_changed'):kind==='pagehide'?new Event('pagehide'):new StorageEvent('storage',{key:'omnisolo_queue_identity_epoch_v2'})));
 expect(screen.queryByDisplayValue(profile.store_name)).not.toBeInTheDocument();expect(screen.getByLabelText('Store / Creator Name Business name')).toBeDisabled();expect(posts()).toHaveLength(0);
});
it('a late private profile cannot refill a retired account view',async()=>{
 const pending=deferred<Response>();read=()=>pending.promise;render(<Page/>);await waitFor(()=>expect(vi.mocked(fetch).mock.calls.length).toBeGreaterThan(1));
 act(()=>window.dispatchEvent(new Event('omnisolo_auth_changed')));await act(async()=>pending.resolve(Response.json(profile)));
 expect(screen.queryByDisplayValue(profile.store_name)).not.toBeInTheDocument();
});
it('load failure holds save rather than overwriting unseen private configuration',async()=>{
 read=async()=>new Response('',{status:503});render(<Page/>);await act(async()=>{});
 expect(screen.getByRole('button',{name:/Save (?:private configuration|& Publish)/})).toBeDisabled();expect(posts()).toHaveLength(0);
});

it('clipboard denial is visible without a copied claim',async()=>{
 vi.mocked(navigator.clipboard.writeText).mockRejectedValue(new Error('denied'));render(<Page/>);await ready();
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Copy saved private preview link'})));
 expect(screen.getByRole('alert',{name:'Private profile clipboard'})).toHaveTextContent(/Copy failed/);
 expect(screen.queryByText('Copied private preview link')).not.toBeInTheDocument();
});
it('repeated same-view save clicks dispatch only one captured update',async()=>{
 const pending=deferred<Response>();save=()=>pending.promise;render(<Page/>);await ready();
 const button=screen.getByRole('button',{name:'Save private configuration'});
 await act(async()=>{fireEvent.click(button);fireEvent.click(button);});expect(posts()).toHaveLength(1);
 await act(async()=>pending.resolve(new Response('',{status:200})));
});
it('a rejected pre-save identity retires previous private fields',async()=>{
 render(<Page/>);await ready();identity=async()=>Response.json({error:'unauthenticated'},{status:401});
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Save private configuration'})));
 expect(posts()).toHaveLength(0);expect(screen.queryByDisplayValue(profile.store_name)).not.toBeInTheDocument();
});
it('an absent own profile allows explicit manual fields but no saved preview link',async()=>{
 read=async()=>new Response('',{status:404});render(<Page/>);await waitFor(()=>expect(screen.getByRole('button',{name:'Save private configuration'})).toBeEnabled());
 expect(screen.getByLabelText('Store / Creator Name Business name')).toHaveValue('');expect(screen.getByRole('button',{name:'Copy saved private preview link'})).toBeDisabled();
 fireEvent.change(screen.getByLabelText('Store / Creator Name Business name'),{target:{value:'Owner-entered business'}});
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Save private configuration'})));
 expect(JSON.parse(posts()[0][1]!.body as string).store_name).toBe('Owner-entered business');
 expect(screen.getByRole('button',{name:'Copy saved private preview link'})).toBeEnabled();
});

it.each([401,403])('a post-preflight %s clears private state before a stalled response body',async status=>{
 const text=vi.fn(()=>new Promise<string>(()=>{}));save=async()=>({status,text} as unknown as Response);render(<Page/>);await ready();
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Save private configuration'})));
 expect(posts()).toHaveLength(1);expect(screen.getByLabelText('Store / Creator Name Business name')).toHaveValue('');
 expect(screen.getByLabelText('Bio / Description Bio tagline')).toHaveValue('');expect(screen.queryByText('Owned link')).not.toBeInTheDocument();
 expect(screen.getByRole('button',{name:'Copy saved private preview link'})).toBeDisabled();expect(text).not.toHaveBeenCalled();
});
it.each(['queued owner does not match the current session','session_identity_changed'])('a post-preflight owner denial %s clears private fields',async error=>{
 save=async()=>Response.json({error},{status:409});render(<Page/>);await ready();
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Save private configuration'})));
 expect(posts()).toHaveLength(1);expect(screen.getByLabelText('Store / Creator Name Business name')).toHaveValue('');
 expect(screen.getByRole('status',{name:'Private profile status'})).toHaveTextContent('Your session changed');
});
it.each([500,409])('an ordinary %s holds the private draft without interpreting a business failure as an owner change',async status=>{
 save=async()=>Response.json({error:'revision_conflict'},{status});render(<Page/>);await ready();
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Save private configuration'})));
 expect(screen.getByLabelText('Store / Creator Name Business name')).toHaveValue(profile.store_name);
 expect(screen.getByRole('button',{name:'Save private configuration'})).toBeDisabled();
 expect(screen.getByRole('status',{name:'Private profile status'})).toHaveTextContent('could not be confirmed');expect(posts()).toHaveLength(1);
});
