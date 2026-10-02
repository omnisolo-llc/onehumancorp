import {act,fireEvent,render,screen} from '@testing-library/react';
import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import Editor from './page';
import Reader from '../bio/[tenant]/page';
vi.mock('next/navigation',()=>({useRouter:()=>({back:vi.fn()}),useParams:()=>({tenant:'tenant-a'})}));
vi.mock('../components/PoweredByOmniSolo',()=>({PoweredByOmniSolo:()=>null}));
const unsafe=['javascript:alert(1)','JaVaScRiPt:alert(1)','data:text/html,<script>alert(1)</script>','blob:https://example.test/id','mailto:a@example.test','tel:+12025550123','/relative','//example.test','https:example.test','https://',' https://example.test','https://exa\nmple.test','https://example.test/white space','https://example.test/\u0000x','https:\\example.test'];
const profile={store_name:'Actual private business',bio:'Actual private description',theme:'light',links:[{title:'Stored destination',url:'https://example.test/original'}],remove_branding:false};
let link=profile.links[0];
beforeEach(()=>{link=profile.links[0];vi.stubGlobal('fetch',vi.fn(async(url,init)=>url==='/api/v1/auth/session-identity'?Response.json({userId:'member-a',tenantId:'tenant-a',expiresAt:Date.now()+60000}):init?.method==='POST'?new Response('',{status:200}):Response.json({...profile,links:[link]})));});
afterEach(()=>{vi.unstubAllGlobals();});
it.each(unsafe)('stored %s is never an active link in either private renderer',async url=>{
 link={title:'Stored destination',url};render(<><Editor/><Reader/></>);await screen.findByDisplayValue(profile.store_name);await act(async()=>{});
 expect(screen.queryByRole('link',{name:'Stored destination'})).not.toBeInTheDocument();
 expect(screen.getAllByText('Stored destination (unavailable)')).toHaveLength(2);
 expect(screen.getByLabelText('Link 1 URL')).toHaveAttribute('value',url);
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Save private configuration'})));
 expect(vi.mocked(fetch).mock.calls.filter(([,init])=>init?.method==='POST')).toHaveLength(0);
});
it('a valid quoted Unicode web destination is preserved and its title stays text',async()=>{
 const url='https://example.test/路径?q=\'"&x=%26#✓';const title='<img src=x onerror="throw 1">';link={title,url};const {container}=render(<><Editor/><Reader/></>);await screen.findByDisplayValue(profile.store_name);await act(async()=>{});
 const anchors=screen.getAllByRole('link',{name:title});expect(anchors).toHaveLength(2);for(const anchor of anchors)expect(anchor).toHaveAttribute('href',url);expect(container.querySelector('img')).toBeNull();
});
it('an invalid edited URL stays editable and cannot dispatch a save',async()=>{
 render(<Editor/>);await screen.findByDisplayValue(profile.store_name);fireEvent.change(screen.getByLabelText('Link 1 URL'),{target:{value:'javascript:alert(1)'}});
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Save private configuration'})));
 expect(vi.mocked(fetch).mock.calls.filter(([,init])=>init?.method==='POST')).toHaveLength(0);
 expect(screen.getByLabelText('Link 1 URL')).toBeEnabled();expect(screen.getByRole('status',{name:'Private profile status'})).toHaveTextContent(/absolute HTTP or HTTPS/);
 fireEvent.change(screen.getByLabelText('Link 1 URL'),{target:{value:'https://example.test/corrected'}});await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Save private configuration'})));
 expect(vi.mocked(fetch).mock.calls.filter(([,init])=>init?.method==='POST')).toHaveLength(1);
});
