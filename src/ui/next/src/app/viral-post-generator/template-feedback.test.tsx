import {act,fireEvent,render,screen} from '@testing-library/react';
import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import Page from './page';
vi.mock('next/navigation',()=>({useRouter:()=>({push:vi.fn()})}));
vi.mock('../components/useProPlan',()=>({useProPlan:()=>({hasPro:false,claimTrial:vi.fn().mockResolvedValue(false),claimError:null})}));
const deferred=()=>{let resolve!:()=>void;let reject!:(error:Error)=>void;const promise=new Promise<void>((yes,no)=>{resolve=yes;reject=no;});return{promise,resolve,reject};};
beforeEach(()=>{localStorage.clear();localStorage.setItem('business_display_name','unverified-local-business');Object.defineProperty(navigator,'clipboard',{configurable:true,value:{writeText:vi.fn().mockResolvedValue(undefined)}});});
afterEach(()=>vi.restoreAllMocks());
function details(product='Owner product'){fireEvent.change(screen.getByPlaceholderText('e.g. Signature Coffee Blend'),{target:{value:product}});fireEvent.change(screen.getByPlaceholderText('e.g. a bold start to your morning'),{target:{value:'Owner benefit'}});}
function generate(){fireEvent.click(screen.getByRole('button',{name:'Generate Post'}));}
it('makes a local template without inventing a storefront or claiming AI execution',()=>{
 render(<Page/>);details();generate();const post=screen.getByText(/Introducing the new Owner product/).textContent!;expect(post).toContain('Owner benefit');expect(post).not.toMatch(/cloud\.omnisolo\.co|unverified-local-business/);expect(screen.getByRole('heading',{name:/Social Post Template/})).toBeInTheDocument();expect(screen.queryByText(/AI promoter agent do the heavy lifting/)).not.toBeInTheDocument();
});
it('includes only the explicitly entered supported web destination without rewriting it',()=>{
 render(<Page/>);details();const url='https://example.test/路径?q=\'&v=%26';fireEvent.change(screen.getByLabelText('Destination link (optional)'),{target:{value:url}});generate();expect(screen.getByText(/Introducing the new Owner product/)).toHaveTextContent(url);
});
it('keeps an unsupported destination visibly ineligible',()=>{
 render(<Page/>);details();fireEvent.change(screen.getByLabelText('Destination link (optional)'),{target:{value:'javascript:alert(1)'}});expect(screen.getByRole('button',{name:'Generate Post'})).toBeDisabled();expect(screen.getByRole('status',{name:'Post requirements'})).toHaveTextContent(/HTTP or HTTPS/);expect(screen.queryByRole('button',{name:'Copy to Clipboard'})).not.toBeInTheDocument();
});
it('waits for the actual clipboard promise before reporting copied',async()=>{
 const pending=deferred();vi.mocked(navigator.clipboard.writeText).mockReturnValue(pending.promise);render(<Page/>);details();generate();fireEvent.click(screen.getByRole('button',{name:'Copy to Clipboard'}));expect(screen.queryByRole('button',{name:'Copied!'})).not.toBeInTheDocument();expect(screen.getByRole('status',{name:'Post clipboard'})).toHaveTextContent('Copying');await act(async()=>pending.resolve());expect(screen.getByRole('button',{name:'Copied!'})).toBeInTheDocument();expect(navigator.clipboard.writeText).toHaveBeenCalledWith(screen.getByText(/Introducing the new Owner product/).textContent);
});
it('clipboard failure is visible without a copied claim',async()=>{
 const pending=deferred();vi.mocked(navigator.clipboard.writeText).mockReturnValue(pending.promise);render(<Page/>);details();generate();fireEvent.click(screen.getByRole('button',{name:'Copy to Clipboard'}));await act(async()=>pending.reject(new Error('clipboard denied')));expect(screen.getByRole('alert',{name:'Post clipboard'})).toHaveTextContent('Copy failed');expect(screen.queryByRole('button',{name:'Copied!'})).not.toBeInTheDocument();
});
it('a late copy completion cannot certify a newly generated template',async()=>{
 const pending=deferred();vi.mocked(navigator.clipboard.writeText).mockReturnValue(pending.promise);render(<Page/>);details();generate();fireEvent.click(screen.getByRole('button',{name:'Copy to Clipboard'}));details('New owner product');generate();await act(async()=>pending.resolve());expect(screen.getByText(/Introducing the new New owner product/)).toBeInTheDocument();expect(screen.queryByRole('button',{name:'Copied!'})).not.toBeInTheDocument();
});
