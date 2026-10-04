import { act, cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import Page from './page';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installOnboardingLocks } from '../onboarding/testLocks';

const firstOwner = { userId: 'protocol-owner-a', tenantId: 'protocol-tenant-a' };
const id = 'aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa';
let owner = { ...firstOwner };
let configured = true;
function row(status = 'queued') {
  return { id, tenant_id: owner.tenantId, actor_id: owner.userId, request_id: 'bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb', name: 'Owner analysis', task: 'Review supplied business notes', workflow: 'expert_task', model: 'owned-model', provider: 'ollama', status, command: '', created_at: '2026-10-03T00:00:00Z', output: status === 'completed' ? 'Actual recorded analysis output' : null, error: null };
}
let rows: unknown[];
let detail: () => Promise<Response>;
let submit: () => Promise<Response>;
let cancel: () => Promise<Response>;
let runtimeRead: () => Promise<Response>;
const mutations = () => vi.mocked(fetch).mock.calls.filter(([, options]) => options?.method === 'POST');
const runtimeCalls = () => vi.mocked(fetch).mock.calls.filter(([url]) => String(url).includes('/agents/protocol'));
const panel = () => within(screen.getByRole('region', { name: 'Recorded text analysis' }));
const analysisRegion = () => screen.getByRole('region', { name: 'Recorded text analysis' });
beforeEach(() => {
  cleanup(); localStorage.clear(); notifyQueueIdentityChange(); installOnboardingLocks();
  owner = { ...firstOwner }; configured = true; rows = [];
  detail = async () => Response.json({ workflow: row() });
  submit = async () => Response.json({ id: `agent-${id.replaceAll('-', '')}`, agent_id: `agent-${id.replaceAll('-', '')}`, workflow_id: id, status: 'queued' }, { status: 201 });
  cancel = async () => { detail = async () => Response.json({ workflow: row('cancelled') }); return Response.json({ workflow: row('cancelled') }); };
  runtimeRead = async () => Response.json({ error: 'Agent runtime is not configured; no work was dispatched' }, { status: 503 });
  vi.stubGlobal('fetch', vi.fn(async (url, options) => {
    const path = String(url);
    if (path.endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    if (path === '/api/v1/agents/execution-policy') return Response.json({ available: configured, mode: 'text_analysis', workspace_access: false, tools: [], policy: configured ? { provider: 'ollama', model: 'owned-model', max_output_tokens: 2048 } : null });
    if (path === '/api/v1/agents/workflows') return Response.json({ workflows: rows });
    if (path === `/api/v1/agents/workflows/${id}`) return detail();
    if (path === `/api/v1/agents/workflows/${id}/cancel` && options?.method === 'POST') return cancel();
    if (path === '/api/v1/agents/hire') return submit();
    if (path.includes('/api/v1/agents/protocol')) return runtimeRead();
    return Response.json({ error: 'unexpected test route' }, { status: 404 });
  }));
});
afterEach(() => { cleanup(); notifyQueueIdentityChange(); vi.unstubAllGlobals(); });
async function open() { const view = render(<Page />); await screen.findByRole('heading', { name: 'Text analysis only' }); return view; }
async function fillTask() {
  fireEvent.change(panel().getByLabelText('Analysis name'), { target: { value: 'My requested analysis' } });
  fireEvent.change(panel().getByLabelText('Analysis task'), { target: { value: 'Analyze only these supplied notes.' } });
  await waitFor(() => expect(panel().getByRole('button', { name: 'Start text analysis' })).toBeEnabled());
}
it('loads actual owned receipt history without invoking a workspace runtime', async () => {
  configured = false; rows = [row('completed')]; detail = async () => Response.json({ workflow: row('completed') });
  await open();
  fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' }));
  expect(await panel().findByLabelText('Recorded analysis output')).toHaveTextContent('Actual recorded analysis output');
  expect(panel().getByLabelText('Recorded analysis status')).toHaveTextContent('completed');
  expect(runtimeCalls()).toHaveLength(0); expect(mutations()).toHaveLength(0);
});
it('submits only explicit text via the existing owner-bound funded path and displays the real queued receipt', async () => {
  await open(); await fillTask(); fireEvent.click(panel().getByRole('button', { name: 'Start text analysis' }));
  await waitFor(() => expect(mutations()).toHaveLength(1));
  const [url, options] = mutations()[0]; expect(url).toBe('/api/v1/agents/hire');
  expect(JSON.parse(String(options!.body))).toEqual({ name: 'My requested analysis', role: 'Text analyst', task: 'Analyze only these supplied notes.', model: 'owned-model', providerType: 'builtin' });
  expect(new Headers(options!.headers).get('x-ohc-expected-user')).toBe(firstOwner.userId);
  expect(new Headers(options!.headers).get('x-ohc-expected-tenant')).toBe(firstOwner.tenantId);
  expect(new Headers(options!.headers).get('idempotency-key')).toMatch(/^[a-f0-9-]{36}$/);
  expect(await panel().findByLabelText('Recorded analysis status')).toHaveTextContent('queued');
  expect(panel().queryByLabelText('Recorded analysis output')).toBeNull(); expect(runtimeCalls()).toHaveLength(0);
});
it('cannot submit without a real configured execution policy', async () => {
  configured = false; await open();
  expect(await panel().findByText(/Text analysis is not configured/)).toBeVisible();
  expect(panel().getByRole('button', { name: 'Start text analysis' })).toBeDisabled(); expect(mutations()).toHaveLength(0);
});
it('holds a lost acknowledgement across reload without duplicate dispatch', async () => {
  submit = async () => { throw new TypeError('connection ended'); };
  const view = await open(); await fillTask(); fireEvent.click(panel().getByRole('button', { name: 'Start text analysis' }));
  expect(await panel().findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  view.unmount(); await open();
  expect(await panel().findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  expect(panel().getByRole('button', { name: 'Start text analysis' })).toBeDisabled(); expect(mutations()).toHaveLength(1);
});
it('clears owned history and rejects a late private detail after an account change', async () => {
  rows = [row()]; let finish!: (response: Response) => void; detail = () => new Promise(resolve => { finish = resolve; });
  await open(); fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' })); await waitFor(() => expect(finish).toBeDefined());
  await act(async () => { owner = { userId: 'protocol-owner-b', tenantId: 'protocol-tenant-b' }; rows = []; notifyQueueIdentityChange(); });
  await act(async () => { finish(Response.json({ workflow: { ...row('completed'), actor_id: firstOwner.userId, output: 'Late private owner A output' } })); });
  await waitFor(() => expect(panel().queryByRole('button', { name: 'Open analysis Owner analysis' })).toBeNull());
  expect(screen.queryByText('Late private owner A output')).toBeNull();
});
it('cancels only the selected owned receipt and reads back actual cancellation', async () => {
  rows = [row()]; await open(); fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' }));
  fireEvent.click(await panel().findByRole('button', { name: 'Cancel text analysis' }));
  expect(await panel().findByLabelText('Recorded analysis status')).toHaveTextContent('cancelled');
  expect(mutations()).toHaveLength(1); expect(mutations()[0][0]).toBe(`/api/v1/agents/workflows/${id}/cancel`);
  expect(new Headers(mutations()[0][1]!.headers).get('x-ohc-expected-user')).toBe(firstOwner.userId);
  expect(panel().queryByLabelText('Recorded analysis output')).toBeNull();
});
it('does not infer cancellation from a failed request and requires status reconciliation', async () => {
  rows = [row()]; cancel = async () => Response.json({ error: 'unavailable' }, { status: 503 });
  await open(); fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' }));
  fireEvent.click(await panel().findByRole('button', { name: 'Cancel text analysis' }));
  expect(await panel().findByText(/Cancellation could not be confirmed/)).toBeVisible();
  expect(panel().getByRole('button', { name: 'Cancel text analysis' })).toBeDisabled();
  expect(panel().getByLabelText('Recorded analysis status')).toHaveTextContent('queued');
  expect(mutations()).toHaveLength(1);
});
it('requires an explicit workspace-runtime load and never substitutes an empty task list for failure', async () => {
  await open(); expect(runtimeCalls()).toHaveLength(0);
  fireEvent.click(screen.getByRole('button', { name: 'Load workspace runtime tasks' }));
  expect(await screen.findByText('Workspace runtime task history could not be verified.')).toBeVisible();
  expect(runtimeCalls()).toHaveLength(1); expect(screen.queryByText('No tasks found.')).toBeNull();
});
it('rejects a contradictory completed receipt without showing a completion or fabricated output', async () => {
  rows = [row()]; detail = async () => Response.json({ workflow: { ...row('completed'), output: null } });
  await open(); fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' }));
  expect(await panel().findByText(/receipt could not be verified/)).toBeVisible();
  expect(panel().queryByLabelText('Recorded analysis output')).toBeNull();
  expect(panel().queryByLabelText('Recorded analysis status')).toBeNull();
});

it.each([
  { actor_id: 'another-owner' }, { tenant_id: 'another-tenant' }, { id: 'cccccccc-cccc-4ccc-8ccc-cccccccccccc' },
  { output: ['must not render'] }, { output: { text: 'must not render' } }, { error: 'contradictory failure' },
])('rejects foreign or malformed completed receipt fields %j', async fields => {
  rows = [row()]; detail = async () => Response.json({ workflow: { ...row('completed'), ...fields } });
  await open(); fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' }));
  expect(await panel().findByText(/receipt could not be verified/)).toBeVisible();
  expect(panel().queryByLabelText('Recorded analysis output')).toBeNull(); expect(mutations()).toHaveLength(0);
});
it('does not turn an unavailable history into an empty or successful history', async () => {
  const original = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url, options) => url === '/api/v1/agents/workflows' ? Promise.resolve(Response.json({ error: 'database unavailable' }, { status: 503 })) : original(url, options));
  await open(); expect(await panel().findByText('Recorded analysis history could not be verified.')).toBeVisible();
  expect(panel().queryByText('No recorded text analyses on this page.')).toBeNull(); expect(mutations()).toHaveLength(0);
});
it('holds repeated cancellation clicks and does not infer cancellation from a success-shaped ACK', async () => {
  rows = [row()]; let finish!: (response: Response) => void; cancel = () => new Promise(resolve => { finish = resolve; });
  await open(); fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' }));
  const button = await panel().findByRole('button', { name: 'Cancel text analysis' });
  fireEvent.click(button); fireEvent.click(button); await waitFor(() => expect(finish).toBeDefined());
  expect(mutations()).toHaveLength(1);
  await act(async () => finish(Response.json({ success: true })));
  await waitFor(() => expect(button).toBeEnabled());
  expect(panel().getByLabelText('Recorded analysis status')).toHaveTextContent('queued');
  expect(panel().queryByText('cancelled', { exact: true })).toBeNull();
});
it('reconciles a failed cancellation only through a fresh owned status read', async () => {
  rows = [row()]; cancel = async () => { throw new TypeError('lost acknowledgement'); };
  await open(); fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' }));
  fireEvent.click(await panel().findByRole('button', { name: 'Cancel text analysis' }));
  expect(await panel().findByText(/Cancellation could not be confirmed/)).toBeVisible();
  detail = async () => Response.json({ workflow: row('outcome_unknown') });
  fireEvent.click(panel().getByRole('button', { name: 'Refresh analysis status' }));
  await waitFor(() => expect(panel().getByLabelText('Recorded analysis status')).toHaveTextContent('outcome_unknown'));
  expect(panel().queryByRole('button', { name: 'Cancel text analysis' })).toBeNull(); expect(mutations()).toHaveLength(1);
});
it('fences a late cancellation response after an account change', async () => {
  rows = [row()]; let finish!: (response: Response) => void; cancel = () => new Promise(resolve => { finish = resolve; });
  await open(); fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' }));
  fireEvent.click(await panel().findByRole('button', { name: 'Cancel text analysis' })); await waitFor(() => expect(finish).toBeDefined());
  await act(async () => { owner = { userId: 'other-owner', tenantId: 'other-tenant' }; rows = []; notifyQueueIdentityChange(); });
  await act(async () => finish(Response.json({ workflow: { ...row('completed'), output: 'Late cancellation data' } })));
  expect(panel().queryByLabelText('Recorded analysis status')).toBeNull(); expect(screen.queryByText('Late cancellation data')).toBeNull();
  expect(mutations()).toHaveLength(1);
});
it('rejects contradictory success envelopes even when a valid receipt is nested inside', async () => {
  rows = [row()]; detail = async () => Response.json({ success: false, error: 'not authorized', workflow: row('completed') });
  await open(); fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' }));
  expect(await panel().findByText(/receipt could not be verified/)).toBeVisible();
  expect(panel().queryByLabelText('Recorded analysis output')).toBeNull();
});
it('allows manual status refresh from queued to the actual completed output without dispatching again', async () => {
  rows = [row()]; await open(); fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' }));
  await panel().findByLabelText('Recorded analysis status'); detail = async () => Response.json({ workflow: row('completed') });
  fireEvent.click(panel().getByRole('button', { name: 'Refresh analysis status' }));
  expect(await panel().findByLabelText('Recorded analysis output')).toHaveTextContent('Actual recorded analysis output');
  expect(mutations()).toHaveLength(0); expect(runtimeCalls()).toHaveLength(0);
});

it('recovers a lost acceptance from the actual request-id receipt after reload without a new model dispatch', async () => {
  let acceptedRequestId = '';
  const original = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation(async (url, options) => {
    if (url === '/api/v1/agents/hire') { acceptedRequestId = new Headers(options?.headers).get('idempotency-key')!; throw new TypeError('lost ACK after acceptance'); }
    if (url === `/api/v1/agents/workflows/by-request/${acceptedRequestId}`) return Response.json({ workflow: { ...row('completed'), request_id: acceptedRequestId, agent_id: `agent-${id.replaceAll('-', '')}` } });
    return original(url, options);
  });
  detail = async () => Response.json({ workflow: row('completed') });
  const view = await open(); await fillTask(); fireEvent.click(panel().getByRole('button', { name: 'Start text analysis' }));
  expect(await panel().findByText(/Could not confirm whether this task was accepted/)).toBeVisible();
  view.unmount(); await open();
  expect(await panel().findByText(/Previously accepted text analysis/)).toBeVisible();
  expect(await panel().findByLabelText('Recorded analysis output')).toHaveTextContent('Actual recorded analysis output');
  expect(mutations()).toHaveLength(1);
});
it('retains explicit input and shows a hard budget rejection without claiming a queued receipt', async () => {
  submit = async () => Response.json({ id: '', agent_id: '', workflow_id: '', status: 'budget_rejected' }, { status: 409 });
  await open(); await fillTask(); fireEvent.click(panel().getByRole('button', { name: 'Start text analysis' }));
  expect(await panel().findByText(/usage budget cannot cover/)).toBeVisible();
  expect(panel().getByLabelText('Analysis task')).toHaveValue('Analyze only these supplied notes.');
  expect(panel().queryByLabelText('Recorded analysis status')).toBeNull();
  expect(panel().getByRole('button', { name: 'Start text analysis' })).toBeEnabled();
  expect(mutations()).toHaveLength(1);
});
it('polls only a pending selected receipt and displays its real terminal output', async () => {
  rows = [row()]; let poll!: () => void;
  const schedule = window.setInterval.bind(window);
  const timer = vi.spyOn(window, 'setInterval').mockImplementation(((callback: TimerHandler, ms?: number, ...args: unknown[]) => {
    if (typeof callback === 'function' && ms === 5000) poll = () => callback();
    return schedule(callback, ms, ...args);
  }) as typeof window.setInterval);
  try {
    await open(); fireEvent.click(await panel().findByRole('button', { name: 'Open analysis Owner analysis' }));
    await waitFor(() => expect(poll).toBeDefined()); detail = async () => Response.json({ workflow: row('completed') });
    await act(async () => poll());
    expect(await panel().findByLabelText('Recorded analysis output')).toHaveTextContent('Actual recorded analysis output');
    expect(mutations()).toHaveLength(0); expect(runtimeCalls()).toHaveLength(0);
  } finally { timer.mockRestore(); }
});
it.each(['foreign-tenant', 'contradictory-envelope'])('never retires a lost-ACK hold from %s recovery data', async failure => {
  let requestId = '';
  const original = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation(async (url, options) => {
    if (url === '/api/v1/agents/hire') { requestId = new Headers(options?.headers).get('idempotency-key')!; throw new TypeError('lost ACK'); }
    if (url === `/api/v1/agents/workflows/by-request/${requestId}`) return Response.json({
      ...(failure === 'contradictory-envelope' ? { success: false, error: 'unavailable' } : {}),
      workflow: { ...row('completed'), tenant_id: failure === 'foreign-tenant' ? 'another-tenant' : owner.tenantId, request_id: requestId, agent_id: `agent-${id.replaceAll('-', '')}` },
    });
    return original(url, options);
  });
  const view = await open(); await fillTask(); fireEvent.click(panel().getByRole('button', { name: 'Start text analysis' }));
  expect(await panel().findByText(/Could not confirm whether this task was accepted/)).toBeVisible(); view.unmount(); await open();
  await waitFor(() => expect(vi.mocked(fetch).mock.calls.some(([url]) => url === `/api/v1/agents/workflows/by-request/${requestId}`)).toBe(true));
  await act(async () => {});
  expect(panel().queryByText(/Previously accepted text analysis/)).toBeNull();
  expect(panel().queryByRole('button', { name: 'Prepare another text analysis' })).toBeNull();
  expect(panel().getByRole('button', { name: 'Start text analysis' })).toBeDisabled(); expect(mutations()).toHaveLength(1);
});
it('validates tenant-wide history and renders only the current actor without rejecting legitimate teammate receipts', async () => {
  rows = [row(), { ...row('completed'), id: 'cccccccc-cccc-4ccc-8ccc-cccccccccccc', actor_id: 'teammate', name: 'Teammate analysis', output: 'Teammate private output' }];
  await open(); expect(await panel().findByRole('button', { name: 'Open analysis Owner analysis' })).toBeVisible();
  expect(panel().queryByRole('button', { name: 'Open analysis Teammate analysis' })).toBeNull();
  expect(panel().queryByText('Recorded analysis history could not be verified.')).toBeNull();
  expect(screen.queryByText('Teammate private output')).toBeNull(); expect(mutations()).toHaveLength(0);
});
it('preserves paging through a same-tenant page containing only teammates to reach the current actor receipt', async () => {
  const cursor = '1800000000:cccccccc-cccc-4ccc-8ccc-cccccccccccc';
  const original = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url, options) => {
    if (url === '/api/v1/agents/workflows') return Promise.resolve(Response.json({ workflows: [{ ...row(), actor_id: 'teammate', name: 'Teammate analysis' }], next_cursor: cursor }));
    if (url === `/api/v1/agents/workflows?before=${encodeURIComponent(cursor)}`) return Promise.resolve(Response.json({ workflows: [row()], next_cursor: null }));
    return original(url, options);
  });
  await open(); expect(await panel().findByText('No recorded text analyses on this page.')).toBeVisible();
  fireEvent.click(panel().getByRole('button', { name: 'Older analyses' }));
  expect(await panel().findByRole('button', { name: 'Open analysis Owner analysis' })).toBeVisible();
  expect(panel().queryByRole('button', { name: 'Open analysis Teammate analysis' })).toBeNull();
  expect(panel().getByRole('button', { name: 'Latest analyses' })).toBeVisible();
  expect(mutations()).toHaveLength(0);
});
it('rejects an entire history response containing a foreign tenant rather than filtering away an authority mismatch', async () => {
  rows = [row(), { ...row(), id: 'cccccccc-cccc-4ccc-8ccc-cccccccccccc', tenant_id: 'foreign-tenant', actor_id: 'teammate', name: 'Foreign tenant analysis' }];
  await open(); expect(await panel().findByText('Recorded analysis history could not be verified.')).toBeVisible();
  expect(panel().queryByRole('button', { name: 'Open analysis Owner analysis' })).toBeNull();
  expect(panel().queryByRole('button', { name: 'Open analysis Foreign tenant analysis' })).toBeNull();
});

it('keeps analysis busy through owner-policy verification and the initial history read', async () => {
  let finishPolicy!: (response: Response) => void;
  let finishHistory!: (response: Response) => void;
  const original = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url, options) => {
    if (url === '/api/v1/agents/execution-policy') return new Promise(resolve => { finishPolicy = resolve; });
    if (url === '/api/v1/agents/workflows') return new Promise(resolve => { finishHistory = resolve; });
    return original(url, options);
  });
  await open();
  expect(analysisRegion()).toHaveAttribute('aria-busy', 'true');
  expect(panel().getByRole('button', { name: 'Refresh analysis history' })).toBeDisabled();
  await waitFor(() => expect(finishPolicy).toBeDefined());
  await act(async () => finishPolicy(Response.json({ available: false, mode: 'text_analysis', workspace_access: false, tools: [], policy: null })));
  await waitFor(() => expect(finishHistory).toBeDefined());
  expect(analysisRegion()).toHaveAttribute('aria-busy', 'true');
  expect(panel().getByRole('button', { name: 'Refresh analysis history' })).toBeDisabled();
  await act(async () => finishHistory(Response.json({ workflows: [] })));
  expect(analysisRegion()).toHaveAttribute('aria-busy', 'false');
  expect(panel().getByRole('button', { name: 'Refresh analysis history' })).toBeEnabled();
  expect(panel().getByRole('button', { name: 'Start text analysis' })).toBeDisabled();
  expect(mutations()).toHaveLength(0);
});

it.each(['session', 'policy', 'revoked-policy'])('settles analysis readiness after unavailable %s verification without enabling execution', async failure => {
  const original = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url, options) => {
    if (failure === 'session' && String(url).endsWith('/session-identity')) return Promise.resolve(Response.json({}, { status: 401 }));
    if (failure !== 'session' && url === '/api/v1/agents/execution-policy') return Promise.resolve(Response.json({ error: 'unavailable' }, { status: failure === 'revoked-policy' ? 401 : 503 }));
    return original(url, options);
  });
  await open();
  await waitFor(() => expect(analysisRegion()).toHaveAttribute('aria-busy', 'false'));
  expect(panel().getByRole('button', { name: 'Refresh analysis history' })).toBeDisabled();
  expect(panel().getByRole('button', { name: 'Start text analysis' })).toBeDisabled();
  expect(panel().getByText('Recorded analysis history could not be verified.')).toBeVisible();
  expect(mutations()).toHaveLength(0);
});

it('settles a failed history refresh while keeping the real refresh control available', async () => {
  await open();
  await waitFor(() => expect(panel().getByRole('button', { name: 'Refresh analysis history' })).toBeEnabled());
  let finish!: (response: Response) => void;
  const original = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url, options) => url === '/api/v1/agents/workflows' ? new Promise(resolve => { finish = resolve; }) : original(url, options));
  fireEvent.click(panel().getByRole('button', { name: 'Refresh analysis history' }));
  await waitFor(() => expect(finish).toBeDefined());
  expect(analysisRegion()).toHaveAttribute('aria-busy', 'true');
  expect(panel().getByRole('button', { name: 'Refresh analysis history' })).toBeDisabled();
  await act(async () => finish(Response.json({ error: 'database unavailable' }, { status: 503 })));
  expect(analysisRegion()).toHaveAttribute('aria-busy', 'false');
  expect(panel().getByRole('button', { name: 'Refresh analysis history' })).toBeEnabled();
  expect(panel().getByText('Recorded analysis history could not be verified.')).toBeVisible();
  expect(mutations()).toHaveLength(0);
});

it('does not let an old owner verification settle the new owner readiness', async () => {
  const finishes: ((response: Response) => void)[] = [];
  const original = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((url, options) => url === '/api/v1/agents/execution-policy' ? new Promise(resolve => { finishes.push(resolve); }) : original(url, options));
  await open(); await waitFor(() => expect(finishes).toHaveLength(1));
  await act(async () => { owner = { userId: 'next-owner', tenantId: 'next-tenant' }; notifyQueueIdentityChange(); });
  await waitFor(() => expect(finishes).toHaveLength(2));
  const unavailablePolicy = () => Response.json({ available: false, mode: 'text_analysis', workspace_access: false, tools: [], policy: null });
  await act(async () => finishes[0](unavailablePolicy()));
  expect(analysisRegion()).toHaveAttribute('aria-busy', 'true');
  expect(panel().getByRole('button', { name: 'Refresh analysis history' })).toBeDisabled();
  await act(async () => finishes[1](unavailablePolicy()));
  await waitFor(() => expect(analysisRegion()).toHaveAttribute('aria-busy', 'false'));
  expect(panel().getByRole('button', { name: 'Refresh analysis history' })).toBeEnabled();
  expect(mutations()).toHaveLength(0);
});
