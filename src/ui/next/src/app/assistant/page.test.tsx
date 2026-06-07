import { render, screen, fireEvent, waitFor, within } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import AssistantPage from './page';

const tasksPayload = {
  tasks: [
    {
      id: 'task-weekly-brief',
      title: "Create this week's operating brief",
      workspace: 'Personal OS',
      status: 'running',
      currentStep: 'Drafting report',
      mode: 'Plan',
      model: 'Auto',
      provider: 'Auto',
      permissionProfile: 'Guarded',
      riskSummary: ['Guarded mode is active', 'External sends require approval'],
      artifacts: [
        { id: 'artifact-weekly', type: 'document', filename: 'weekly-brief.md', preview: 'Weekly brief draft with action items.' },
      ],
      changes: [
        { id: 'change-weekly', path: '/workspace/reports/weekly-brief.md', summary: 'Creates a markdown brief.', approvalStatus: 'pending' },
      ],
      messages: [
        { id: 'msg-1', role: 'user', content: 'Summarize this week and prepare a brief.' },
        { id: 'msg-2', role: 'assistant', content: 'I am gathering context and drafting the brief.' },
      ],
    },
    {
      id: 'task-downloads',
      title: 'Organize Downloads by file type',
      workspace: 'Files',
      status: 'blocked',
      currentStep: 'Waiting for folder permission',
      mode: 'Craft',
      model: 'MiniMax-M3',
      provider: 'Auto',
      permissionProfile: 'Guarded',
      riskSummary: ['Needs permission for Downloads'],
      artifacts: [],
      changes: [],
      messages: [
        { id: 'msg-3', role: 'assistant', content: 'I need permission to read Downloads before continuing.' },
      ],
    },
  ],
};

beforeEach(() => {
  vi.clearAllMocks();
  global.fetch = vi.fn(async (url: RequestInfo | URL, init?: RequestInit) => {
    const urlString = typeof url === 'string' ? url : url.toString();
    if (urlString.includes('/api/assistant/tasks') && init?.method === 'POST') {
      return new Response(JSON.stringify({
        task: {
          id: 'task-new',
          title: 'Build a Q3 planning deck',
          workspace: 'Launch Room',
          status: 'running',
          currentStep: 'Planning and preparing tools',
          mode: 'Plan',
          model: 'MiniMax-M3',
          provider: 'Auto',
          permissionProfile: 'Guarded',
          riskSummary: ['Guarded mode is active', 'External sends require approval'],
          artifacts: [
            { id: 'artifact-deck', type: 'presentation', filename: 'assistant-presentation.pptx', preview: 'Slide deck outline.' },
            { id: 'artifact-chart', type: 'chart', filename: 'assistant-chart.png', preview: 'Generated chart preview.' },
          ],
          changes: [
            { id: 'change-deck', path: '/workspace/launch/assistant-presentation.pptx', summary: 'Creates a deck.', approvalStatus: 'pending' },
          ],
          messages: [
            { id: 'msg-user', role: 'user', content: 'Build a Q3 planning deck' },
            { id: 'msg-assistant', role: 'assistant', content: 'Jarvis planned the task with Web Research.' },
          ],
        },
      }), { status: 201, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/assistant/tasks')) {
      return new Response(JSON.stringify(tasksPayload), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    return new Response(JSON.stringify({}), { status: 200, headers: { 'Content-Type': 'application/json' } });
  }) as any;
});

test('renders the primary Jarvis workstation and preserves Expert Center navigation', async () => {
  render(<AssistantPage />);

  expect(await screen.findByRole('heading', { name: 'Jarvis Assistant' })).toBeDefined();
  expect(screen.getByText('Personal OS')).toBeDefined();
  expect(screen.getByText('Files')).toBeDefined();
  expect(screen.getByText("Create this week's operating brief")).toBeDefined();
  expect(screen.getByText('Organize Downloads by file type')).toBeDefined();
  expect(screen.getByRole('link', { name: 'Expert Center' })).toHaveAttribute('href', '/agents');
  expect(screen.getByRole('button', { name: 'Remote Control' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Automations' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Memory' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Skills' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Connectors' })).toBeDefined();
});

test('shows conversation, artifacts, files, changes, and preview for the active task', async () => {
  render(<AssistantPage />);

  expect(await screen.findByText('I am gathering context and drafting the brief.')).toBeDefined();
  expect(screen.getByText('weekly-brief.md')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'All Files' }));
  expect(screen.getByText('/workspace/reports/weekly-brief.md')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Changes' }));
  expect(screen.getByText('Creates a markdown brief.')).toBeDefined();
  expect(screen.getByText('pending')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Preview' }));
  expect(screen.getByText('Weekly brief draft with action items.')).toBeDefined();
});

test('submits full WorkBuddy-style composer payload and selects the new task', async () => {
  render(<AssistantPage />);

  await screen.findByText("Create this week's operating brief");
  fireEvent.change(screen.getByLabelText('Task prompt'), {
    target: { value: 'Build a Q3 planning deck' },
  });
  fireEvent.change(screen.getByLabelText('Workspace'), {
    target: { value: 'Launch Room' },
  });
  fireEvent.change(screen.getByLabelText('Work directory'), {
    target: { value: '/workspace/launch' },
  });
  fireEvent.change(screen.getByLabelText('Output format'), {
    target: { value: 'Presentation' },
  });
  fireEvent.click(screen.getByRole('button', { name: 'Start Task' }));

  await waitFor(() => {
    expect(global.fetch).toHaveBeenCalledWith('/api/assistant/tasks', expect.objectContaining({ method: 'POST' }));
  });
  const taskCall = (global.fetch as any).mock.calls.find(
    ([url, init]: any[]) => url === '/api/assistant/tasks' && init?.method === 'POST',
  );
  const payload = JSON.parse(taskCall[1].body);
  expect(payload).toMatchObject({
    prompt: 'Build a Q3 planning deck',
    workspace: 'Launch Room',
    workDirectory: '/workspace/launch',
    outputFormat: 'Presentation',
    permissionProfile: 'Guarded',
  });
  expect(await screen.findByText('assistant-presentation.pptx')).toBeDefined();
});

test('opens feature panels for remote control, automations, memory, skills, and connectors', async () => {
  render(<AssistantPage />);

  await screen.findByRole('heading', { name: 'Jarvis Assistant' });
  fireEvent.click(screen.getByRole('button', { name: 'Remote Control' }));
  expect(screen.getByText('Slack')).toBeDefined();
  expect(screen.getByText('Telegram')).toBeDefined();
  expect(screen.getByText('Discord')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Automations' }));
  expect(screen.getByText('Weekly research brief')).toBeDefined();
  expect(screen.getByText('Execution history')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Memory' }));
  expect(screen.getByText('Prefer concise technical summaries with citations.')).toBeDefined();
  expect(screen.getByText('Import Memory')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Skills' }));
  expect(screen.getByText('Skill Marketplace')).toBeDefined();
  expect(screen.getByText('Web Research')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Connectors' }));
  const connectorPanel = screen.getByLabelText('Connector panel');
  expect(within(connectorPanel).getByText('Google Drive')).toBeDefined();
  expect(within(connectorPanel).getByText('MCP Endpoint')).toBeDefined();
});
