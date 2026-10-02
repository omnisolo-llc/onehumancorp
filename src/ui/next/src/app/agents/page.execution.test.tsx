import { act, cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, expect, test, vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installOnboardingLocks } from '../onboarding/testLocks';
import { readOwnedOnboardingItem } from '../onboarding/draftSession';
import { TooltipProvider } from '../../components/TooltipRegistry';
import AgentsPage from './page';

const firstOwner = { userId: 'analysis-owner-a', tenantId: 'analysis-tenant-a' };
let owner = { ...firstOwner };
let configured = true;
let workflowRows: unknown[] = [];
let submit: () => Promise<Response>;
const receipt = { id: 'agent-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa', agent_id: 'agent-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa', workflow_id: 'aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa', status: 'queued', message: 'Accepted text analysis' };
const mutations = () => vi.mocked(fetch).mock.calls.filter(([url]) => url === '/api/v1/agents/hire');
const open = async () => { const view = render(<TooltipProvider><AgentsPage /></TooltipProvider>); await act(async () => { await Promise.resolve(); }); return view; };
beforeEach(() => {
  installOnboardingLocks(); localStorage.clear(); owner = { ...firstOwner }; configured = true; workflowRows = []; notifyQueueIdentityChange();
  submit = async () => Response.json(receipt, { status: 201 });
  vi.stubGlobal('EventSource', class { addEventListener() {} close() {} });
  vi.stubGlobal('fetch', vi.fn(async (url: string) => {
    if (url.endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    if (url === '/api/v1/agents/execution-policy') return Response.json({ available: configured, mode: 'text_analysis', workspace_access: false, tools: [], policy: configured ? { provider: 'ollama', model: 'configured-model', max_output_tokens: 2048 } : null });
    if (url === '/api/v1/agents/hire') return submit();
    if (url === '/api/v1/agents/workflows') return Response.json({ workflows: workflowRows });
    if (url.includes('/api/v1/agents/approvals')) return Response.json({ pending_approvals: [] });
    return Response.json({});
  }));
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });

test('sends only the explicit text task through the verified owner boundary and reports queued, not completed', async () => {
  await open();
  await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.change(screen.getByLabelText('Task prompt'), { target: { value: 'Analyze this supplied plan.' } });
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  await waitFor(() => expect(mutations()).toHaveLength(1));
  const options = mutations()[0][1]!;
  expect(JSON.parse(String(options.body))).toEqual({ name: 'Growth Strategist', role: 'Business growth operator', providerType: 'builtin', model: 'configured-model', task: 'Analyze this supplied plan.' });
  expect(new Headers(options.headers).get('x-ohc-expected-tenant')).toBe(firstOwner.tenantId);
  expect(await screen.findByText(/Text analysis queued/)).toBeVisible();
  expect(screen.queryByText(/Task completed/)).not.toBeInTheDocument();
});
test('unconfigured execution cannot dispatch a task', async () => {
  configured = false; await open();
  expect(await screen.findByText(/Text analysis is not configured/)).toBeVisible();
  expect(screen.getByRole('button', { name: 'Start task' })).toBeDisabled();
  expect(mutations()).toHaveLength(0);
});
test('nonempty unsupported options prevent dispatch instead of being silently dropped', async () => {
  await open();
  await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.change(screen.getByLabelText('Work directory'), { target: { value: '/private/records' } });
  expect(screen.getByRole('button', { name: 'Start task' })).toBeDisabled();
  expect(screen.getByText(/Workspace, tools, connectors, attachments/)).toBeVisible();
  expect(mutations()).toHaveLength(0);
});
test('an ambiguous dispatch stays held across reload without a second request', async () => {
  submit = async () => { throw new TypeError('connection ended'); };
  const view = await open();
  await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  expect(await screen.findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  view.unmount(); await open();
  expect(await screen.findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  expect(screen.getByRole('button', { name: 'Start task' })).toBeDisabled();
  expect(mutations()).toHaveLength(1);
});

test.each(['invalid-json', 'missing-id', 'running-without-queue-receipt', 'server-error'])('holds %s without claiming acceptance', async failure => {
  submit = async () => failure === 'invalid-json' ? new Response('bad', { status: 201 }) : Response.json(failure === 'missing-id' ? { status: 'queued' } : { ...receipt, status: failure === 'running-without-queue-receipt' ? 'running' : 'error' }, { status: failure === 'server-error' ? 503 : 201 });
  await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  expect(await screen.findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  expect(screen.queryByText(/Text analysis queued/)).not.toBeInTheDocument();
  expect(screen.getByRole('button', { name: 'Start task' })).toBeDisabled();
  expect(mutations()).toHaveLength(1);
});
test('a definite validation rejection permits editing but never claims acceptance', async () => {
  submit = async () => Response.json({ id: '', agent_id: '', workflow_id: '', status: 'error', message: 'Invalid payload' }, { status: 400 });
  await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  expect(await screen.findByText(/rejected before acceptance/)).toBeVisible();
  expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled();
  expect(screen.queryByText(/Text analysis queued/)).not.toBeInTheDocument();
});
test('cannot dispatch when its durable marker cannot be written', async () => {
  await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  const write = localStorage.setItem;
  localStorage.setItem = (key, value) => { if (key.endsWith(':agent-analysis-request')) throw new DOMException('full', 'QuotaExceededError'); write.call(localStorage, key, value); };
  try {
    fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
    expect(await screen.findByText(/No task was sent/)).toBeVisible();
    expect(mutations()).toHaveLength(0);
  } finally { localStorage.setItem = write; }
});
test('an owner switch clears private input and fences the late response body', async () => {
  let finish!: () => void;
  submit = async () => new Response(new ReadableStream({ start(controller) { finish = () => { controller.enqueue(new TextEncoder().encode(JSON.stringify(receipt))); controller.close(); }; } }), { status: 201 });
  await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.change(screen.getByLabelText('Task prompt'), { target: { value: 'Private owner A plan' } });
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  await waitFor(() => expect(finish).toBeDefined());
  await act(async () => { owner = { userId: 'owner-b', tenantId: 'tenant-b' }; notifyQueueIdentityChange(); });
  await waitFor(() => expect(screen.getByLabelText('Task prompt')).toHaveValue(''));
  await act(async () => finish());
  expect(screen.queryByText(/Text analysis queued/)).not.toBeInTheDocument();
  expect(screen.queryByText(receipt.workflow_id)).not.toBeInTheDocument();
  await act(async () => { owner = { ...firstOwner }; notifyQueueIdentityChange(); });
  expect(await screen.findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  expect(mutations()).toHaveLength(1);
});
test('a stored acknowledgement is never used as a fabricated fresh acceptance receipt', async () => {
  const view = await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  expect(await screen.findByText(/Text analysis queued/)).toBeVisible();
  view.unmount(); await open();
  expect(await screen.findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  expect(screen.queryByText(/Text analysis queued/)).not.toBeInTheDocument();
  expect(mutations()).toHaveLength(1);
});
test('the workflow form never fabricates a running record when the transport fails', async () => {
  const original = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation(async (url, options) => url === '/api/v1/agents/workflows' && options?.method === 'POST' ? Response.json({ error: 'unavailable' }, { status: 503 }) : original(url, options));
  submit = async () => { throw new TypeError('connection ended'); };
  await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.click(screen.getByRole('button', { name: 'Workflows' }));
  fireEvent.change(screen.getByLabelText('Workflow Name'), { target: { value: 'Do not invent this workflow' } });
  fireEvent.change(screen.getByLabelText('Task', { exact: true }), { target: { value: 'Analyze the supplied text' } });
  const form = within(screen.getByLabelText('Task', { exact: true }).closest('form')!);
  fireEvent.click(form.getByRole('button', { name: 'Create & Run Workflow' }));
  await waitFor(() => expect(form.queryByText('Creating...')).not.toBeInTheDocument());
  expect(screen.queryByRole('heading', { name: 'Do not invent this workflow' })).not.toBeInTheDocument();
  expect(screen.queryByText('running', { exact: true })).not.toBeInTheDocument();
});
test('separate mounted views cannot submit the same unresolved task twice', async () => {
  const releases: Array<(response: Response) => void> = [];
  submit = () => new Promise(resolve => releases.push(resolve));
  const first = await open(), second = await open();
  const start1 = within(first.container).getByRole('button', { name: 'Start task' });
  const start2 = within(second.container).getByRole('button', { name: 'Start task' });
  await waitFor(() => { expect(start1).toBeEnabled(); expect(start2).toBeEnabled(); });
  fireEvent.click(start1); fireEvent.click(start2);
  await waitFor(() => expect(mutations()).toHaveLength(1));
  await act(async () => { await new Promise(resolve => setTimeout(resolve, 10)); });
  try { expect(mutations()).toHaveLength(1); }
  finally { await act(async () => releases.forEach(resolve => resolve(Response.json(receipt, { status: 201 })))); }
  await waitFor(() => expect(within(second.container).getByText(/Could not confirm whether this task was accepted/)).toBeVisible());
  expect(mutations()).toHaveLength(1);
});
test('a late identity reply after the submission deadline cannot dispatch', async () => {
  await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  const original = vi.mocked(fetch).getMockImplementation()!;
  let identity!: (response: Response) => void;
  vi.mocked(fetch).mockImplementation((url, options) => String(url).endsWith('/session-identity') ? new Promise(resolve => { identity = resolve; }) : original(url, options));
  let expire!: () => void;
  const schedule = window.setTimeout.bind(window);
  vi.spyOn(window, 'setTimeout').mockImplementation(((handler: TimerHandler, delay?: number, ...args: unknown[]) => {
    if (delay === 30_000 && typeof handler === 'function') { expire = () => handler(); return schedule(() => {}, 30_000); }
    return schedule(handler, delay, ...args);
  }) as typeof window.setTimeout);
  try {
    fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
    await waitFor(() => expect(identity).toBeDefined());
    await act(async () => expire());
    expect(await screen.findByText(/No task was sent/)).toBeVisible();
    await act(async () => identity(Response.json({ ...owner, expiresAt: Date.now() + 60_000 })));
    expect(mutations()).toHaveLength(0);
  } finally { vi.restoreAllMocks(); }
});

test('reload accepts a stored reference only after the authenticated workflow read confirms it', async () => {
  const view = await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  expect(await screen.findByText(/Text analysis queued/)).toBeVisible();
  workflowRows = [{ id: receipt.workflow_id, actor_id: firstOwner.userId, name: 'Actual analysis', task: 'Supplied task', workflow: 'expert_task', status: 'completed', output: 'Actual returned text' }];
  view.unmount(); await open();
  expect(await screen.findByText(/Previously accepted text analysis/)).toBeVisible();
  expect(screen.getAllByText('Actual returned text').length).toBeGreaterThan(0);
  expect(mutations()).toHaveLength(1);
  fireEvent.click(screen.getByRole('button', { name: 'Prepare another task' }));
  await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  expect(mutations()).toHaveLength(1);
});
test.each(['failure', 'malformed'])('a %s first workflow read is unavailable rather than a verified empty list', async kind => {
  const original = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url, options) => url === '/api/v1/agents/workflows' ? Promise.resolve(kind === 'failure' ? Response.json({ error: 'unavailable' }, { status: 503 }) : Response.json({ workflows: 'not-an-array' })) : original(url, options));
  await open(); fireEvent.click(screen.getByRole('button', { name: 'Workflows' }));
  expect(await screen.findByText(/Agent records are unavailable/)).toBeVisible();
  expect(screen.queryByText('No workflows yet.')).not.toBeInTheDocument();
});
test('a successful current-owner empty read can show an empty workflow list', async () => {
  await open(); fireEvent.click(screen.getByRole('button', { name: 'Workflows' }));
  expect(await screen.findByText('No workflows yet.')).toBeVisible();
  expect(screen.queryByText(/Agent records are unavailable/)).not.toBeInTheDocument();
});
test('the durable duplicate-prevention marker does not retain private task text', async () => {
  submit = async () => { throw new TypeError('connection ended'); };
  await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  const privateTask = 'Private account planning text that must stay out of the receipt marker';
  fireEvent.change(screen.getByLabelText('Task prompt'), { target: { value: privateTask } });
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  expect(await screen.findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  const stored = readOwnedOnboardingItem('agent-analysis-request');
  expect(stored).not.toContain(privateTask);
  expect(JSON.parse(stored!)).toEqual({ status: 'unknown', request_id: new Headers(mutations()[0][1]!.headers).get('idempotency-key') });
  expect(mutations()).toHaveLength(1);
});

test('sends a durable request UUID and retains only that identity after an ambiguous response', async () => {
  submit = async () => { throw new TypeError('connection ended'); };
  await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  expect(await screen.findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  const key = new Headers(mutations()[0][1]!.headers).get('idempotency-key');
  expect(key, 'a dispatched request must have a durable identity').not.toBeNull();
  expect(key).toMatch(/^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$/);
  expect(JSON.parse(readOwnedOnboardingItem('agent-analysis-request')!)).toEqual({ status: 'unknown', request_id: key });
});

test('recovers a request UUID only from its authenticated owner receipt after a lost acknowledgement', async () => {
  submit = async () => { throw new TypeError('connection ended'); };
  const view = await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  expect(await screen.findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  const requestId = new Headers(mutations()[0][1]!.headers).get('idempotency-key');
  workflowRows = [{ id: receipt.workflow_id, request_id: requestId, actor_id: firstOwner.userId, agent_id: receipt.agent_id, name: 'Actual accepted work', task: 'Submitted task', workflow: 'expert_task', status: 'completed', output: 'Actual returned text' }];
  view.unmount(); await open();
  expect(await screen.findByText(/Previously accepted text analysis/)).toBeVisible();
  expect(mutations()).toHaveLength(1);
  fireEvent.click(screen.getByRole('button', { name: 'Prepare another task' }));
  await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
});

test('a confirmed budget rejection permits a new explicit attempt without holding an unstarted request', async () => {
  submit = async () => Response.json({ id: '', agent_id: '', workflow_id: '', status: 'budget_rejected', message: 'Budget cannot cover this request' }, { status: 409 });
  await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  expect(await screen.findByText(/budget cannot cover this request/i)).toBeVisible();
  expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled();
  expect([null, '']).toContain(readOwnedOnboardingItem('agent-analysis-request'));
  expect(mutations()).toHaveLength(1);
});

test('a different actor receipt cannot clear an unresolved request marker', async () => {
  submit = async () => { throw new TypeError('connection ended'); };
  const view = await open(); await waitFor(() => expect(screen.getByRole('button', { name: 'Start task' })).toBeEnabled());
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));
  expect(await screen.findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  const requestId = new Headers(mutations()[0][1]!.headers).get('idempotency-key');
  workflowRows = [{ id: receipt.workflow_id, request_id: requestId, actor_id: 'different-owner', agent_id: receipt.agent_id, name: 'Other actor work', task: 'Other task', workflow: 'expert_task', status: 'completed' }];
  view.unmount(); await open();
  expect(await screen.findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  expect(screen.getByRole('button', { name: 'Start task' })).toBeDisabled();
  expect(screen.queryByRole('button', { name: 'Prepare another task' })).not.toBeInTheDocument();
  expect(mutations()).toHaveLength(1);
});
