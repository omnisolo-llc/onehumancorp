import "@testing-library/jest-dom/vitest";
import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach,beforeEach,expect,test,vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installOnboardingLocks } from '../onboarding/testLocks';
import { readOwnedOnboardingItem, writeOwnedOnboardingItem } from '../onboarding/draftSession';
import AssistantPage from './page';
vi.mock('../components/AppShell',()=>({AppShell:({children}:{children:React.ReactNode})=><main>{children}</main>}));
vi.mock('../../components/Walkthrough',()=>({InteractiveWalkthrough:()=>null,WalkthroughTarget:({children}:{children:React.ReactNode})=><div>{children}</div>}));
const id='aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa';
let owner={userId:'assistant-owner',tenantId:'assistant-tenant'};
let submit:(init:RequestInit)=>Promise<Response>;
let byRequest:Record<string,unknown>|null;
const task=(key:string,status='queued',output:string|null=null)=>({id,title:'Analyze supplied notes',workspace:'Personal OS',status,currentStep:status,mode:'Ask',model:'configured-model',provider:'ollama',permissionProfile:'Text only',riskSummary:[],artifacts:[],changes:[],messages:[],archived:false,output,execution:{id,request_id:key,root_request_id:key,tenant_id:owner.tenantId,actor_id:owner.userId,phase:status === 'running' ? 'dispatching' : status,output}});
const posts=()=>vi.mocked(fetch).mock.calls.filter(([url,init])=>url==='/api/v1/assistant/tasks'&&init?.method==='POST');
async function open(){const view=render(<AssistantPage/>);await act(async()=>{await Promise.resolve();});fireEvent.click(screen.getByRole('button',{name:'New Task'}));return view;}
beforeEach(()=>{
  installOnboardingLocks();localStorage.clear();owner={userId:'assistant-owner',tenantId:'assistant-tenant'};byRequest=null;notifyQueueIdentityChange();
  submit=async init=>Response.json({task:task(new Headers(init.headers).get('Idempotency-Key')!)},{status:202});
  vi.stubGlobal('fetch',vi.fn(async(url:RequestInfo|URL,init?:RequestInit)=>{
    const path=String(url);
    if(path.endsWith('/session-identity'))return Response.json({...owner,expiresAt:Date.now()+60_000});
    if(path==='/api/v1/agents/execution-policy')return Response.json({available:true,mode:'text_analysis',workspace_access:false,tools:[],policy:{provider:'ollama',model:'configured-model',max_output_tokens:512}});
    if(path==='/api/v1/assistant/tasks'&&init?.method==='POST')return submit(init);
    if(path.includes('/tasks/by-request/'))return Response.json(byRequest?{task:byRequest}:{error:'Not found'},{status:byRequest?200:404});
    if(path==='/api/v1/assistant/tasks')return Response.json({tasks:[],nextCursor:null,capabilities:{outputFormats:['Text'],workModes:['Ask'],modelProviders:['Auto']}});
    if(path.includes('/walkthrough/'))return Response.json([]);
    return Response.json({settings:{}});
  }));
});
afterEach(()=>{cleanup();vi.unstubAllGlobals();});
test('failure keeps the prompt and never creates a synthetic running task',async()=>{
  submit=async()=>Response.json({error:'Runtime unavailable'},{status:503});await open();
  fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Analyze supplied notes'}});
  await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());
  fireEvent.click(screen.getByRole('button',{name:'Start Task'}));
  expect(await screen.findByText(/Acceptance is unconfirmed/)).toBeVisible();
  expect(screen.getByLabelText('Task prompt')).toHaveValue('Analyze supplied notes');
  expect(screen.queryByText('Drafting response')).toBeNull();expect(posts()).toHaveLength(1);
});
test('ambiguous transport survives reload and retry retains the exact identity and draft',async()=>{
  submit=async()=>{throw new TypeError('transport lost');};const view=await open();
  fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Analyze supplied notes'}});
  await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());
  fireEvent.click(screen.getByRole('button',{name:'Start Task'}));await screen.findByText(/Acceptance is unconfirmed/);
  const first=posts()[0][1]!;view.unmount();await open();await screen.findByText(/Acceptance is unconfirmed/);
  expect(posts()).toHaveLength(1);expect(screen.getByLabelText('Task prompt')).toHaveValue('Analyze supplied notes');
  fireEvent.click(screen.getByRole('button',{name:'Retry same request'}));await waitFor(()=>expect(posts()).toHaveLength(2));
  expect(new Headers(posts()[1][1]?.headers).get('Idempotency-Key')).toBe(new Headers(first.headers).get('Idempotency-Key'));
  expect(posts()[1][1]?.body).toBe(first.body);
});
test('only text capability is offered and accepted status comes from the receipt',async()=>{
  await open();fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Analyze supplied notes'}});
  expect(screen.queryByRole('option',{name:'Coding'})).toBeNull();expect(screen.queryByRole('option',{name:'PDF'})).toBeNull();
  expect(screen.getByLabelText('Work directory')).toBeDisabled();
  await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());fireEvent.click(screen.getByRole('button',{name:'Start Task'}));
  await waitFor(()=>expect(posts()).toHaveLength(1));
  const body=JSON.parse(String(posts()[0][1]?.body));expect(body).toMatchObject({mode:'Ask',outputFormat:'Text',workDirectory:''});
  expect(body.status).toBeUndefined();expect(await screen.findByText(/Execution receipt:/)).toBeVisible();
});
test('a different account cannot restore the old task draft',async()=>{
  submit=async()=>{throw new Error('lost');};const view=await open();fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Private old-owner input'}});
  await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());fireEvent.click(screen.getByRole('button',{name:'Start Task'}));await screen.findByText(/Acceptance is unconfirmed/);
  view.unmount();owner={userId:'new-owner',tenantId:'new-tenant'};notifyQueueIdentityChange();await open();
  await waitFor(()=>expect(screen.getByLabelText('Task prompt')).toHaveValue(''));
  expect(posts()).toHaveLength(1);
});
function deferred<T>() { let resolve!:(value:T)=>void; const promise=new Promise<T>(yes=>{resolve=yes;});return {promise,resolve}; }
test('a late history response cannot discard a newly admitted receipt',async()=>{
  const history=deferred<Response>();const original=vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url,init)=>String(url)==='/api/v1/assistant/tasks'&&(!init?.method||init.method==='GET')?history.promise:original(url,init));
  await open();fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Analyze supplied notes'}});
  await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());fireEvent.click(screen.getByRole('button',{name:'Start Task'}));
  await screen.findByText(/Execution receipt:/);await act(async()=>history.resolve(Response.json({tasks:[],nextCursor:null})));
  expect(screen.getByText(/Execution receipt:/)).toBeVisible();
});
test('an older refresh cannot replace completed output with a running snapshot',async()=>{
  let key='';submit=async init=>{key=new Headers(init.headers).get('Idempotency-Key')!;return Response.json({task:task(key,'running')},{status:202});};
  const first=deferred<Response>(),second=deferred<Response>();let reads=0;const original=vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url,init)=>String(url)===`/api/v1/assistant/tasks/${id}`?(++reads===1?first.promise:second.promise):original(url,init));
  await open();fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Analyze supplied notes'}});await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());fireEvent.click(screen.getByRole('button',{name:'Start Task'}));await screen.findByText(/Execution receipt:/);
  fireEvent.click(screen.getByRole('button',{name:'Refresh Task'}));await waitFor(()=>expect(reads).toBe(1));fireEvent.click(screen.getByRole('button',{name:'Refresh Task'}));await waitFor(()=>expect(reads).toBe(2));
  await act(async()=>second.resolve(Response.json({task:task(key,'completed','Durable final answer')})));expect(screen.getByLabelText('Text response')).toHaveTextContent('Durable final answer');
  await act(async()=>first.resolve(Response.json({task:task(key,'running')})));expect(screen.getByLabelText('Text response')).toHaveTextContent('Durable final answer');
});
test('stale readback never clears a newer owner-scoped request marker',async()=>{
  submit=async()=>{throw new Error('lost');};await open();fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Analyze supplied notes'}});await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());fireEvent.click(screen.getByRole('button',{name:'Start Task'}));await screen.findByText(/Acceptance is unconfirmed/);
  const saved=JSON.parse(readOwnedOnboardingItem('assistant-text-request-v1')!);const late=deferred<Response>();const original=vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url,init)=>String(url).includes('/tasks/by-request/')?late.promise:original(url,init));
  fireEvent.click(screen.getByRole('button',{name:'Check acceptance'}));await waitFor(()=>expect(vi.mocked(fetch).mock.calls.some(([url])=>String(url).includes('/tasks/by-request/'))).toBe(true));
  const newer={...saved,requestId:'dddddddd-dddd-4ddd-8ddd-dddddddddddd',body:JSON.stringify({prompt:'A newer submitted draft'})};writeOwnedOnboardingItem('assistant-text-request-v1',JSON.stringify(newer));
  await act(async()=>late.resolve(Response.json({task:task(saved.requestId,'completed','Earlier result')})));
  expect(JSON.parse(readOwnedOnboardingItem('assistant-text-request-v1')!)).toEqual(newer);expect(posts()).toHaveLength(1);
});
test('account change clears private workspace and ignores an old resource response',async()=>{
  const late=deferred<Response>();let connectorReads=0;const original=vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url,init)=>String(url)==='/api/v1/assistant/connectors'?(++connectorReads===1?late.promise:Promise.resolve(Response.json({connectors:[]}))):original(url,init));
  await open();fireEvent.change(screen.getByLabelText('Workspace'),{target:{value:'Private workspace label'}});fireEvent.click(screen.getByRole('button',{name:'Connectors'}));await waitFor(()=>expect(connectorReads).toBe(1));
  await act(async()=>{owner={userId:'second-owner',tenantId:'second-tenant'};notifyQueueIdentityChange();});
  await act(async()=>late.resolve(Response.json({connectors:[{id:'old-private',name:'Private old-owner connector'}]})));
  expect(screen.queryByText('Private old-owner connector')).toBeNull();fireEvent.click(screen.getByRole('button',{name:'New Task'}));expect(screen.getByLabelText('Workspace')).toHaveValue('Personal OS');
});
test('invalid oversized workspace is editable without sending or holding a request',async()=>{
  await open();fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Analyze supplied notes'}});fireEvent.change(screen.getByLabelText('Workspace'),{target:{value:'x'.repeat(81)}});await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());fireEvent.click(screen.getByRole('button',{name:'Start Task'}));
  expect(await screen.findByText(/No task was submitted/)).toBeVisible();expect(posts()).toHaveLength(0);expect(readOwnedOnboardingItem('assistant-text-request-v1')).toBeNull();
});
test('explicit pre-admission rejection releases only its own request marker',async()=>{
  submit=async()=>Response.json({error:'Unsupported text option',effect:'none',rejected_before_admission:true},{status:400});await open();fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Analyze supplied notes'}});await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());fireEvent.click(screen.getByRole('button',{name:'Start Task'}));
  expect(await screen.findByText('Unsupported text option')).toBeVisible();expect(readOwnedOnboardingItem('assistant-text-request-v1')).toBeFalsy();expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled();
});
test('recovery adopts a newer tab marker written before the read begins',async()=>{
  submit=async()=>{throw new Error('lost');};await open();fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Analyze supplied notes'}});await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());fireEvent.click(screen.getByRole('button',{name:'Start Task'}));await screen.findByText(/Acceptance is unconfirmed/);
  const saved=JSON.parse(readOwnedOnboardingItem('assistant-text-request-v1')!);const newer={...saved,requestId:'dddddddd-dddd-4ddd-8ddd-dddddddddddd',body:JSON.stringify({prompt:'Newer tab text'})};
  writeOwnedOnboardingItem('assistant-text-request-v1',JSON.stringify(newer));byRequest=task(newer.requestId,'completed','Newer tab saved output');
  fireEvent.click(screen.getByRole('button',{name:'Check acceptance'}));expect(await screen.findByLabelText('Text response')).toHaveTextContent('Newer tab saved output');
  expect(vi.mocked(fetch).mock.calls.some(([url])=>String(url).endsWith(`/tasks/by-request/${newer.requestId}`))).toBe(true);
  expect(readOwnedOnboardingItem('assistant-text-request-v1')).toBeFalsy();expect(posts()).toHaveLength(1);
});
test('an already-cleared tab marker still requires authoritative readback before releasing the local hold',async()=>{
  submit=async()=>{throw new Error('lost');};await open();fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Analyze supplied notes'}});await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());fireEvent.click(screen.getByRole('button',{name:'Start Task'}));await screen.findByText(/Acceptance is unconfirmed/);
  const saved=JSON.parse(readOwnedOnboardingItem('assistant-text-request-v1')!);writeOwnedOnboardingItem('assistant-text-request-v1','');
  fireEvent.click(screen.getByRole('button',{name:'Check acceptance'}));await waitFor(()=>expect(vi.mocked(fetch).mock.calls.some(([url])=>String(url).endsWith(`/tasks/by-request/${saved.requestId}`))).toBe(true));
  expect(screen.getByRole('button',{name:'Start Task'})).toBeDisabled();
  byRequest=task(saved.requestId,'completed','Previously acknowledged result');await waitFor(()=>expect(screen.getByRole('button',{name:'Check acceptance'})).toBeEnabled());fireEvent.click(screen.getByRole('button',{name:'Check acceptance'}));
  expect(await screen.findByLabelText('Text response')).toHaveTextContent('Previously acknowledged result');expect(screen.queryByRole('button',{name:'Check acceptance'})).toBeNull();expect(posts()).toHaveLength(1);
});
test('verified earlier attempt lookup reconciles even after the current receipt has advanced',async()=>{
  submit=async()=>{throw new Error('lost');};await open();fireEvent.change(screen.getByLabelText('Task prompt'),{target:{value:'Analyze supplied notes'}});await waitFor(()=>expect(screen.getByRole('button',{name:'Start Task'})).toBeEnabled());fireEvent.click(screen.getByRole('button',{name:'Start Task'}));await screen.findByText(/Acceptance is unconfirmed/);
  const saved=JSON.parse(readOwnedOnboardingItem('assistant-text-request-v1')!);const advanced=task('eeeeeeee-eeee-4eee-8eee-eeeeeeeeeeee','completed','Later attempt output');advanced.execution.root_request_id='ffffffff-ffff-4fff-8fff-ffffffffffff';byRequest={...advanced,matchedRequestId:saved.requestId};
  fireEvent.click(screen.getByRole('button',{name:'Check acceptance'}));expect(await screen.findByLabelText('Text response')).toHaveTextContent('Later attempt output');expect(readOwnedOnboardingItem('assistant-text-request-v1')).toBeFalsy();expect(posts()).toHaveLength(1);
});
