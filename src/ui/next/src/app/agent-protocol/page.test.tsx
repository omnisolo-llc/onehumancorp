import React from 'react';
import { act, cleanup, render, screen, fireEvent, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, it, expect, vi } from 'vitest';
import AgentProtocolPage from './page';
import { AppShell } from '../components/AppShell';
import { resolveShellRoute } from '../components/shellRoutes';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installOnboardingLocks } from '../onboarding/testLocks';
import { invalidateOnboardingSession } from '../onboarding/draftSession';

vi.mock('@/components/VoiceAssistant', () => ({ VoiceAssistant: () => null }));
vi.mock('../components/Omnibox', () => ({ Omnibox: () => null }));
vi.mock('../components/LogoutButton', () => ({ LogoutButton: () => null }));
vi.mock('@/components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: React.ReactNode }) => children }));

const mockRuntime = vi.fn();
const task = { task_id: 'task-1', input: 'Write a poem' };
let tasks: typeof task[];
let steps: { step_id: string; status: string; input?: string; output?: string }[];
let checkpoints: { checkpoint_id: string; created_at: string }[];
async function loadRuntime() {
  render(<AgentProtocolPage />);
  fireEvent.click(screen.getByRole('button', { name: 'Load workspace runtime tasks' }));
  await waitFor(() => expect(mockRuntime).toHaveBeenCalledWith('/api/v1/agents/protocol?method=ap_list_tasks', undefined));
}
describe('Agent Protocol UI', () => {
  beforeEach(() => {
    cleanup(); localStorage.clear(); notifyQueueIdentityChange(); installOnboardingLocks(); tasks = []; steps = []; checkpoints = [{ checkpoint_id: 'cp-1', created_at: '2026-10-03' }];
    mockRuntime.mockReset().mockImplementation(async (url, options) => {
      if (options?.method === 'POST') {
        const body = JSON.parse(options.body);
        if (body.method === 'ap_create_task') { tasks = [task]; return Response.json(task); }
        if (body.method === 'ap_execute_step') { checkpoints = [{ checkpoint_id: 'cp-2', created_at: '2026-10-03T12:00:00Z' }]; steps = [{ step_id: 'step-1', status: 'completed', input: 'Line 1', output: 'Roses are red' }]; return Response.json(steps[0]); }
        if (body.method === 'ap_restore_checkpoint') { steps = []; return Response.json({ success: true }); }
      }
      if (String(url).includes('ap_list_steps')) return Response.json({ steps });
      if (String(url).includes('ap_list_checkpoints')) return Response.json({ checkpoints });
      return Response.json({ tasks });
    });
    vi.stubGlobal('fetch', vi.fn(async (url, options) => {
      if (String(url).endsWith('/session-identity')) return Response.json({ userId: 'owner', tenantId: 'tenant', expiresAt: Date.now() + 60_000 });
      if (url === '/api/v1/agents/execution-policy') return Response.json({ available: false, mode: 'text_analysis', workspace_access: false, tools: [], policy: null });
      if (url === '/api/v1/agents/workflows') return Response.json({ workflows: [] });
      return mockRuntime(url, options);
    }));
  });
  afterEach(() => { cleanup(); notifyQueueIdentityChange(); vi.unstubAllGlobals(); });
  it('uses the application shell as the only page heading and main landmark', async () => {
    const route = resolveShellRoute('/agent-protocol');
    expect(route.owner).toBe('guard');
    render(<AppShell title={route.title}><AgentProtocolPage /></AppShell>);
    expect(screen.getAllByRole('heading', { name: /^Agent Protocol UI$/ })).toHaveLength(1);
    expect(screen.getAllByRole('heading', { level: 1 })).toHaveLength(1);
    expect(screen.getAllByRole('main')).toHaveLength(1);
    expect(screen.getByRole('heading', { name: 'Workspace runtime', level: 2 })).toBeVisible();
    await screen.findByText('Text analysis is not configured. No task can be submitted.');
  });
  it('keeps runtime reads explicit and does not mistake an unloaded list for an empty one', async () => {
    render(<AgentProtocolPage />);
    expect(screen.getByRole('heading', { name: 'Workspace runtime' })).toBeInTheDocument();
    expect(screen.getByText('Tasks')).toBeInTheDocument();
    expect(mockRuntime).not.toHaveBeenCalled(); expect(screen.queryByText('No tasks found.')).toBeNull();
    fireEvent.click(screen.getByRole('button', { name: 'Load workspace runtime tasks' }));
    expect(await screen.findByText('No tasks found.')).toBeVisible();
  });
  it('preserves actual runtime creation and its method', async () => {
    await loadRuntime();
    fireEvent.change(screen.getByPlaceholderText('New Task Input...'), { target: { value: task.input } });
    fireEvent.click(screen.getByRole('button', { name: 'Create' }));
    expect(await screen.findByText(task.input)).toBeVisible();
    const [, options] = mockRuntime.mock.calls.find(([, options]) => options?.method === 'POST')!;
    expect(JSON.parse(options.body)).toEqual({ method: 'ap_create_task', params: { input: task.input } });
  });
  it.each([
    { status: 502, body: { error: 'Checkpointer not configured' } },
    { status: 200, body: { error: 'Checkpointer not configured' } },
    { status: 200, body: {} },
  ])('does not report unavailable checkpoint history as an empty saved list %#', async ({ status, body }) => {
    const runtime = mockRuntime.getMockImplementation()!;
    mockRuntime.mockImplementation((url, options) => String(url).includes('ap_list_checkpoints')
      ? Response.json(body, { status }) : runtime(url, options));
    tasks = [task]; await loadRuntime(); fireEvent.click(await screen.findByText(task.input));
    expect(await screen.findByRole('alert')).toHaveTextContent('Checkpoint history could not be verified.');
    expect(screen.queryByText('No checkpoints saved.')).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Restore Checkpoint' })).not.toBeInTheDocument();
  });
  it.each([
    { nextState: 'ready', oldStatus: 200 },
    { nextState: 'loading', oldStatus: 200 },
    { nextState: 'unavailable', oldStatus: 200 },
    { nextState: 'ready', oldStatus: 502 },
  ])('ignores a late checkpoint response after switching tasks: %j', async ({ nextState, oldStatus }) => {
    const otherTask = { task_id: 'task-2', input: 'Write a shopping list' };
    const runtime = mockRuntime.getMockImplementation()!;
    let finishOld!: (response: Response) => void;
    const oldReply = new Promise<Response>(resolve => { finishOld = resolve; });
    mockRuntime.mockImplementation((url, options) => {
      if (String(url).includes('ap_list_checkpoints')) {
        if (String(url).endsWith('task_id=task-1')) return oldReply;
        if (nextState === 'loading') return new Promise<Response>(() => {});
        return Response.json(nextState === 'ready' ? { checkpoints: [{ checkpoint_id: 'cp-b', created_at: '2026-10-03' }] } : { error: 'Unavailable' },
          { status: nextState === 'ready' ? 200 : 502 });
      }
      return runtime(url, options);
    });
    tasks = [task, otherTask]; await loadRuntime();
    fireEvent.click(await screen.findByText(task.input));
    await waitFor(() => expect(mockRuntime).toHaveBeenCalledWith('/api/v1/agents/protocol?method=ap_list_checkpoints&task_id=task-1', undefined));
    fireEvent.click(screen.getByText(otherTask.input));
    const assertCurrentState = async () => {
      if (nextState === 'ready') expect(await screen.findByText('cp-b')).toBeVisible();
      if (nextState === 'loading') expect(await screen.findByText('Loading checkpoints…')).toBeVisible();
      if (nextState === 'unavailable') expect(await screen.findByRole('alert')).toHaveTextContent('Checkpoint history could not be verified.');
      expect(screen.queryByText('cp-a')).not.toBeInTheDocument();
      expect(screen.queryByText('No checkpoints saved.')).not.toBeInTheDocument();
    };
    await assertCurrentState();
    await act(async () => finishOld(Response.json(oldStatus === 200
      ? { checkpoints: [{ checkpoint_id: 'cp-a', created_at: '2026-10-03' }] } : { error: 'Old request unavailable' }, { status: oldStatus })));
    await assertCurrentState();
  });
  it('does not restore a selected task checkpoint after session retirement clears the selection', async () => {
    const runtime = mockRuntime.getMockImplementation()!;
    let finish!: (response: Response) => void;
    mockRuntime.mockImplementation((url, options) => String(url).includes('ap_list_checkpoints')
      ? new Promise<Response>(resolve => { finish = resolve; }) : runtime(url, options));
    tasks = [task]; await loadRuntime(); fireEvent.click(await screen.findByText(task.input));
    await waitFor(() => expect(finish).toBeDefined());
    act(() => invalidateOnboardingSession());
    expect(await screen.findByText('Select a task to view its steps.')).toBeVisible();
    await act(async () => finish(Response.json({ checkpoints: [{ checkpoint_id: 'retired-cp', created_at: '2026-10-03' }] })));
    expect(screen.queryByText('retired-cp')).not.toBeInTheDocument();
    expect(screen.queryByRole('heading', { name: 'State Checkpoints' })).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Restore Checkpoint' })).not.toBeInTheDocument();
  });
  it.each([
    { nextState: 'ready', oldStatus: 200 },
    { nextState: 'loading', oldStatus: 200 },
    { nextState: 'unavailable', oldStatus: 200 },
    { nextState: 'ready', oldStatus: 502 },
  ])('ignores delayed step history from the previous task: %j', async ({ nextState, oldStatus }) => {
    const otherTask = { task_id: 'task-2', input: 'Write a shopping list' };
    const runtime = mockRuntime.getMockImplementation()!;
    let finish!: (response: Response) => void;
    const oldReply = new Promise<Response>(resolve => { finish = resolve; });
    mockRuntime.mockImplementation((url, options) => {
      if (String(url).includes('ap_list_steps')) {
        if (String(url).endsWith('task_id=task-1')) return oldReply;
        if (nextState === 'loading') return new Promise<Response>(() => {});
        return Response.json(nextState === 'ready' ? { steps: [{ step_id: 'step-b', status: 'completed', output: 'Task B output' }] } : { error: 'Unavailable' },
          { status: nextState === 'ready' ? 200 : 502 });
      }
      return runtime(url, options);
    });
    tasks = [task, otherTask]; await loadRuntime(); fireEvent.click(await screen.findByText(task.input));
    await waitFor(() => expect(mockRuntime).toHaveBeenCalledWith('/api/v1/agents/protocol?method=ap_list_steps&task_id=task-1', undefined));
    fireEvent.click(screen.getByText(otherTask.input));
    const assertCurrentState = async () => {
      if (nextState === 'ready') expect(await screen.findByText('Task B output')).toBeVisible();
      if (nextState === 'loading') expect(await screen.findByText('Loading steps…')).toBeVisible();
      if (nextState === 'unavailable') expect(await screen.findByText('Step history could not be verified.')).toBeVisible();
      expect(screen.queryByText('Task A output')).not.toBeInTheDocument();
      expect(screen.queryByText('No steps executed yet.')).not.toBeInTheDocument();
      expect(screen.queryByText('Failed to fetch steps')).not.toBeInTheDocument();
    };
    await assertCurrentState();
    await act(async () => finish(Response.json(oldStatus === 200 ? { steps: [{ step_id: 'step-a', status: 'completed', output: 'Task A output' }] } : { error: 'Old read unavailable' }, { status: oldStatus })));
    await assertCurrentState();
  });
  it('clears already rendered steps as soon as another task is selected', async () => {
    const otherTask = { task_id: 'task-2', input: 'Write a shopping list' };
    const runtime = mockRuntime.getMockImplementation()!;
    mockRuntime.mockImplementation((url, options) => String(url).includes('ap_list_steps') && String(url).endsWith('task_id=task-2')
      ? new Promise<Response>(() => {}) : runtime(url, options));
    tasks = [task, otherTask]; steps = [{ step_id: 'step-a', status: 'completed', output: 'Old selected output' }];
    await loadRuntime(); fireEvent.click(await screen.findByText(task.input));
    expect(await screen.findByText('Old selected output')).toBeVisible();
    fireEvent.click(screen.getByText(otherTask.input));
    expect(screen.queryByText('Old selected output')).not.toBeInTheDocument();
    expect(screen.getByText('Loading steps…')).toBeVisible();
  });
  it.each([
    { method: 'ap_execute_step', status: 200 }, { method: 'ap_execute_step', status: 503 },
    { method: 'ap_restore_checkpoint', status: 200 }, { method: 'ap_restore_checkpoint', status: 503 },
  ])('keeps the newer task view and draft after an older action settles: %j', async ({ method, status }) => {
    const otherTask = { task_id: 'task-2', input: 'Write a shopping list' };
    const runtime = mockRuntime.getMockImplementation()!;
    let finish!: (response: Response) => void;
    mockRuntime.mockImplementation((url, options) => {
      if (options?.method === 'POST' && JSON.parse(options.body).method === method) return new Promise<Response>(resolve => { finish = resolve; });
      if (String(url).includes('ap_list_steps')) return Response.json({ steps: [{ step_id: 'actual-step', status: 'completed', output: String(url).endsWith('task_id=task-2') ? 'Task B current output' : 'Task A old output' }] });
      return runtime(url, options);
    });
    tasks = [task, otherTask]; await loadRuntime(); fireEvent.click(await screen.findByText(task.input));
    await screen.findByText('cp-1');
    fireEvent.change(screen.getByPlaceholderText('Optional Step Input...'), { target: { value: 'Task A input' } });
    fireEvent.click(screen.getByRole('button', { name: method === 'ap_execute_step' ? 'Execute Step' : 'Restore Checkpoint' }));
    await waitFor(() => expect(finish).toBeDefined());
    fireEvent.click(screen.getByText(otherTask.input));
    expect(await screen.findByText('Task B current output')).toBeVisible();
    fireEvent.change(screen.getByPlaceholderText('Optional Step Input...'), { target: { value: 'Task B newer input' } });
    await act(async () => finish(Response.json(status === 200 ? { success: true } : { error: 'Action outcome unknown' }, { status })));
    expect(screen.getByText('Task B current output')).toBeVisible();
    expect(screen.queryByText('Task A old output')).not.toBeInTheDocument();
    expect(screen.getByPlaceholderText('Optional Step Input...')).toHaveValue('Task B newer input');
    expect(screen.queryByText(/Failed to (execute step|restore checkpoint)/)).not.toBeInTheDocument();
    expect(mockRuntime.mock.calls.filter(([, options]) => options?.method === 'POST')).toHaveLength(1);
    expect(screen.getByRole('button', { name: 'Execute Step' })).toBeEnabled();
  });
  it('keeps a newer same-task draft when the submitted step succeeds', async () => {
    const runtime = mockRuntime.getMockImplementation()!;
    let finish!: (response: Response) => void;
    mockRuntime.mockImplementation((url, options) => options?.method === 'POST' && JSON.parse(options.body).method === 'ap_execute_step'
      ? new Promise<Response>(resolve => { finish = resolve; }) : runtime(url, options));
    tasks = [task]; await loadRuntime(); fireEvent.click(await screen.findByText(task.input));
    fireEvent.change(screen.getByPlaceholderText('Optional Step Input...'), { target: { value: 'Submitted step' } });
    fireEvent.click(screen.getByRole('button', { name: 'Execute Step' }));
    await waitFor(() => expect(finish).toBeDefined());
    fireEvent.change(screen.getByPlaceholderText('Optional Step Input...'), { target: { value: 'Next unsent step' } });
    await act(async () => finish(Response.json({ success: true })));
    expect(screen.getByPlaceholderText('Optional Step Input...')).toHaveValue('Next unsent step');
  });
  it('refreshes the actual checkpoint list after a step completes', async () => {
    tasks = [task]; await loadRuntime(); fireEvent.click(await screen.findByText(task.input));
    expect(await screen.findByText('cp-1')).toBeVisible();
    fireEvent.click(screen.getByRole('button', { name: 'Execute Step' }));
    expect(await screen.findByText('Roses are red')).toBeVisible();
    expect(await screen.findByText('cp-2')).toBeVisible();
    expect(screen.queryByText('cp-1')).not.toBeInTheDocument();
  });
  it('preserves actual step execution and checkpoint restoration without routing either to text analysis', async () => {
    tasks = [task]; await loadRuntime(); fireEvent.click(await screen.findByText(task.input));
    fireEvent.change(await screen.findByPlaceholderText('Optional Step Input...'), { target: { value: 'Line 1' } });
    fireEvent.click(screen.getByRole('button', { name: 'Execute Step' }));
    expect(await screen.findByText('Roses are red')).toBeVisible();
    expect(await screen.findByText('cp-2')).toBeVisible();
    fireEvent.click(await screen.findByRole('button', { name: 'Restore Checkpoint' }));
    await waitFor(() => expect(screen.queryByText('Roses are red')).toBeNull());
    expect(mockRuntime.mock.calls.filter(([, options]) => options?.method === 'POST').map(([, options]) => JSON.parse(options.body))).toEqual([
      { method: 'ap_execute_step', params: { task_id: 'task-1', input: 'Line 1' } },
      { method: 'ap_restore_checkpoint', params: { task_id: 'task-1', checkpoint_id: 'cp-2' } },
    ]);
    expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/agents/hire')).toBe(false);
  });
});
