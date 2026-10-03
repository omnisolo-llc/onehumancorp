import React from 'react';
import { cleanup, render, screen, fireEvent, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, it, expect, vi } from 'vitest';
import AgentProtocolPage from './page';
import { AppShell } from '../components/AppShell';
import { resolveShellRoute } from '../components/shellRoutes';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installOnboardingLocks } from '../onboarding/testLocks';

vi.mock('@/components/VoiceAssistant', () => ({ VoiceAssistant: () => null }));
vi.mock('../components/Omnibox', () => ({ Omnibox: () => null }));
vi.mock('../components/LogoutButton', () => ({ LogoutButton: () => null }));
vi.mock('@/components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: React.ReactNode }) => children }));

const mockRuntime = vi.fn();
const task = { task_id: 'task-1', input: 'Write a poem' };
let tasks: typeof task[];
let steps: { step_id: string; status: string; input?: string; output?: string }[];
async function loadRuntime() {
  render(<AgentProtocolPage />);
  fireEvent.click(screen.getByRole('button', { name: 'Load workspace runtime tasks' }));
  await waitFor(() => expect(mockRuntime).toHaveBeenCalledWith('/api/v1/agents/protocol?method=ap_list_tasks', undefined));
}
describe('Agent Protocol UI', () => {
  beforeEach(() => {
    cleanup(); localStorage.clear(); notifyQueueIdentityChange(); installOnboardingLocks(); tasks = []; steps = [];
    mockRuntime.mockReset().mockImplementation(async (url, options) => {
      if (options?.method === 'POST') {
        const body = JSON.parse(options.body);
        if (body.method === 'ap_create_task') { tasks = [task]; return Response.json(task); }
        if (body.method === 'ap_execute_step') { steps = [{ step_id: 'step-1', status: 'completed', input: 'Line 1', output: 'Roses are red' }]; return Response.json(steps[0]); }
        if (body.method === 'ap_restore_checkpoint') { steps = []; return Response.json({ success: true }); }
      }
      if (String(url).includes('ap_list_steps')) return Response.json({ steps });
      if (String(url).includes('ap_list_checkpoints')) return Response.json({ checkpoints: [{ checkpoint_id: 'cp-1', created_at: '2026-10-03' }] });
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
  it('preserves actual step execution and checkpoint restoration without routing either to text analysis', async () => {
    tasks = [task]; await loadRuntime(); fireEvent.click(await screen.findByText(task.input));
    fireEvent.change(await screen.findByPlaceholderText('Optional Step Input...'), { target: { value: 'Line 1' } });
    fireEvent.click(screen.getByRole('button', { name: 'Execute Step' }));
    expect(await screen.findByText('Roses are red')).toBeVisible();
    fireEvent.click(await screen.findByRole('button', { name: 'Restore Checkpoint' }));
    await waitFor(() => expect(screen.queryByText('Roses are red')).toBeNull());
    expect(mockRuntime.mock.calls.filter(([, options]) => options?.method === 'POST').map(([, options]) => JSON.parse(options.body))).toEqual([
      { method: 'ap_execute_step', params: { task_id: 'task-1', input: 'Line 1' } },
      { method: 'ap_restore_checkpoint', params: { task_id: 'task-1', checkpoint_id: 'cp-1' } },
    ]);
    expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/agents/hire')).toBe(false);
  });
});
