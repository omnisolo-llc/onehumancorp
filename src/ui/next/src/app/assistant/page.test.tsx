import { render, screen, fireEvent, waitFor, within } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import { TooltipProvider } from '../../components/TooltipRegistry';
import AssistantPage from './page';

vi.mock('next/navigation', () => ({
  usePathname: () => '/assistant',
  useRouter: () => ({
    push: vi.fn(),
    replace: vi.fn(),
    prefetch: vi.fn(),
    back: vi.fn(),
    forward: vi.fn(),
    refresh: vi.fn(),
  }),
}));

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
      riskSummary: ['Guarded mode is active'],
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
      model: 'MiniMax M2.5',
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
  capabilities: {
    outputFormats: ['Document', 'Presentation', 'PDF', 'Code App'],
    workModes: ['Ask', 'Agent', 'Plan', 'Coding'],
    modelProviders: ['Auto', 'Agent', 'MiniMax M2.5'],
  },
};

beforeEach(() => {
  vi.clearAllMocks();
  global.fetch = vi.fn(async (url: RequestInfo | URL, init?: RequestInit) => {
    const urlString = typeof url === 'string' ? url : url.toString();
    if (urlString.includes('/api/v1/assistant/tasks') && init?.method === 'POST') {
      return new Response(JSON.stringify({
        task: {
          id: 'task-new',
          title: 'Build a Q3 planning deck',
          workspace: 'Launch Room',
          status: 'running',
          currentStep: 'Planning and preparing tools',
          mode: 'Plan',
          model: 'Auto',
          provider: 'Auto',
          permissionProfile: 'Guarded',
          riskSummary: ['Guarded mode is active'],
          artifacts: [
            { id: 'artifact-deck', type: 'presentation', filename: 'assistant-presentation.pptx', preview: 'Slide deck outline.' },
          ],
          changes: [],
          messages: [
            { id: 'msg-user', role: 'user', content: 'Build a Q3 planning deck' },
            { id: 'msg-assistant', role: 'assistant', content: 'Agent planned the task.' },
          ],
        },
      }), { status: 201, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/share')) {
      return new Response(JSON.stringify({ share: { id: 'share-1', target: 'Share Link' } }), { status: 201, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/previews')) {
      return new Response(JSON.stringify({ preview: { artifactId: 'artifact-weekly' } }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/tasks')) {
      return new Response(JSON.stringify(tasksPayload), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }

    if (urlString.includes('/api/v1/assistant/connectors')) {
      return new Response(JSON.stringify({ connectors: [{ id: 'connector-github', name: 'GitHub' }, { id: 'connector-gitlab', name: 'GitLab' }, { id: 'connector-slack', name: 'Slack' }] }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/models')) {
      return new Response(JSON.stringify({ models: [{ id: 'model-1', customProtocol: true, capabilities: ['Model capabilities'] }] }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/settings')) {
      return new Response(JSON.stringify({ settings: { compactMode: true, autoInstallLowRiskSkills: true, preventSleep: true, profile: { name: 'Test' }, version: 'Test', supportTickets: [{ screenshot: 'feedback.png' }], paritySummary: { total: 212 } } }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/data')) {
      return new Response(JSON.stringify({ sharedFiles: [{ id: 'file-1', action: 'Copy Link' }, { id: 'file-2', action: 'Download' }, { id: 'file-3', action: 'Cancel Sharing' }], archivedTasks: [{ id: 'task-1', action: 'Unarchive Task' }] }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/automations')) {
      return new Response(JSON.stringify({ automations: [{ id: 'auto-1', schedule: 'Hourly' }, { id: 'auto-2', schedule: 'Daily' }, { id: 'auto-3', schedule: 'Weekly' }, { id: 'auto-4', schedule: 'One-time' }] }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/parity')) {
      return new Response(JSON.stringify({ summary: { total: '212', implemented: '212', remaining: '0' }, categories: [{name: 'Test', total: 1, implemented: 1}], gaps: [] }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }

    return new Response(JSON.stringify({}), { status: 200, headers: { 'Content-Type': 'application/json' } });
  }) as any;
});

function renderAssistantPage() {
  return render(
    <TooltipProvider>
      <AssistantPage />
    </TooltipProvider>,
  );
}

test('renders Task List as a real section page and keeps resource sections honest', async () => {
  renderAssistantPage();

  expect(await screen.findByRole('heading', { name: 'Agent Assistant' })).toBeDefined();
  const sectionMenu = screen.getByLabelText('Assistant section menu');
  expect(within(sectionMenu).getByRole('button', { name: 'Task List' })).toHaveAttribute('aria-pressed', 'true');
  expect(within(sectionMenu).getByRole('button', { name: 'New Task' })).toBeDefined();
  expect(within(sectionMenu).getByRole('button', { name: 'Conversation' })).toBeDefined();
  expect(within(sectionMenu).getByRole('button', { name: 'Results' })).toBeDefined();
  expect(within(sectionMenu).getByRole('button', { name: 'Connectors' })).toBeDefined();

  expect(screen.getByRole('heading', { name: 'Task List' })).toBeDefined();
  expect(await screen.findByText("Create this week's operating brief")).toBeDefined();
  expect(screen.getByText('Organize Downloads by file type')).toBeDefined();
  expect(screen.queryByText('Skill Marketplace')).toBeNull();
});

test('navigates between real sections without leaving fake buttons behind', async () => {
  renderAssistantPage();

  await screen.findByRole('heading', { name: 'Agent Assistant' });
  fireEvent.click(screen.getByRole('button', { name: 'Conversation' }));
  expect(screen.getByRole('heading', { name: "Create this week's operating brief" })).toBeDefined();
  expect(screen.getByText('I am gathering context and drafting the brief.')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Results' }));
  expect(screen.getByRole('heading', { name: 'Results' })).toBeDefined();
  expect(screen.getByText('weekly-brief.md')).toBeDefined();
  expect(screen.getByRole('button', { name: 'Share Link' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Open Preview' })).toBeDefined();
  expect(screen.queryByRole('button', { name: 'Export DOCX' })).toBeNull();
  expect(screen.queryByRole('button', { name: 'Install Office Suite' })).toBeNull();
});

test('filters tasks and selects the clicked task', async () => {
  renderAssistantPage();

  await screen.findByText("Create this week's operating brief");
  fireEvent.change(screen.getByLabelText('Search tasks'), { target: { value: 'downloads' } });
  expect(screen.queryByText("Create this week's operating brief")).toBeNull();
  expect(screen.getByText('Organize Downloads by file type')).toBeDefined();

  fireEvent.click(screen.getByText('Organize Downloads by file type'));
  fireEvent.click(screen.getByRole('button', { name: 'Conversation' }));
  expect(screen.getByRole('heading', { name: 'Organize Downloads by file type' })).toBeDefined();
});

test('submits a real task creation request and selects the returned task', async () => {
  renderAssistantPage();

  await screen.findByRole('heading', { name: 'Agent Assistant' });
  fireEvent.click(screen.getByRole('button', { name: 'New Task' }));
  fireEvent.change(screen.getByLabelText('Task prompt'), { target: { value: 'Build a Q3 planning deck' } });
  fireEvent.change(screen.getByLabelText('Workspace'), { target: { value: 'Launch Room' } });
  fireEvent.change(screen.getByLabelText('Work directory'), { target: { value: '/workspace/launch' } });
  fireEvent.change(screen.getByLabelText('Output format'), { target: { value: 'Presentation' } });
  fireEvent.click(screen.getByRole('button', { name: 'Start Task' }));

  await waitFor(() => {
    expect(global.fetch).toHaveBeenCalledWith('/api/v1/assistant/tasks', expect.objectContaining({ method: 'POST' }));
  });
  const taskCall = (global.fetch as any).mock.calls.find(
    ([url, init]: any[]) => url === '/api/v1/assistant/tasks' && init?.method === 'POST',
  );
  expect(JSON.parse(taskCall[1].body)).toMatchObject({
    prompt: 'Build a Q3 planning deck',
    workspace: 'Launch Room',
    workDirectory: '/workspace/launch',
    outputFormat: 'Presentation',
  });
  expect(await screen.findByText('assistant-presentation.pptx')).toBeDefined();
});

test('shows no fake result actions when no artifact exists', async () => {
  renderAssistantPage();

  await screen.findByText('Organize Downloads by file type');
  fireEvent.click(screen.getByText('Organize Downloads by file type'));
  fireEvent.click(screen.getByRole('button', { name: 'Results' }));
  expect(screen.getByText('No artifacts yet.')).toBeDefined();
  expect(screen.queryByRole('button', { name: 'Share Link' })).toBeNull();
  expect(screen.queryByRole('button', { name: 'Open Preview' })).toBeNull();
});


test('renders new parity gaps elements', async () => {
  renderAssistantPage();

  fireEvent.click(screen.getByRole('button', { name: 'Connectors' }));
  expect(await screen.findByText('GitHub')).toBeDefined();
  expect(screen.getByText('GitLab')).toBeDefined();
  expect(screen.getByText('Slack')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Models' }));
  expect(await screen.findByText('Custom Protocol')).toBeDefined();
  expect(screen.getAllByText('Capabilities').length).toBeGreaterThan(0);

  fireEvent.click(screen.getByRole('button', { name: 'System' }));
  expect(await screen.findByText(/Compact Mode/i, { exact: false })).toBeDefined();
  expect(screen.getByText(/Auto Install Low Risk Skills/i, { exact: false })).toBeDefined();
  expect(screen.getByText(/Prevent Sleep/i, { exact: false })).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Data' }));
  expect(await screen.findByText('Copy Link')).toBeDefined();
  expect(screen.getByText('Download')).toBeDefined();
  expect(screen.getByText('Cancel Sharing')).toBeDefined();
  expect(screen.getByText('Unarchive Task')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Automations' }));
  expect(await screen.findByText('Hourly')).toBeDefined();
  expect(screen.getByText('Daily')).toBeDefined();
  expect(screen.getByText('Weekly')).toBeDefined();
  expect(screen.getByText('One-time')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Parity Audit' }));
  const els = await screen.findAllByText(/212/);
  expect(els.length).toBeGreaterThan(0);
});

test('renders empty Assistant state without seeded demo records', async () => {
  renderAssistantPage();

  expect(await screen.findByRole('heading', { name: 'Agent Assistant' })).toBeDefined();
  expect(screen.queryByText("Create this week's operating brief")).toBeDefined();
});

test('shows resource error instead of connector demo records', async () => {
  global.fetch = vi.fn(async (url: RequestInfo | URL) => {
    const urlString = typeof url === 'string' ? url : url.toString();
    if (urlString.includes('/api/v1/assistant/tasks')) {
      return new Response(JSON.stringify({
        tasks: [],
        capabilities: {
          outputFormats: ['Document', 'Presentation', 'PDF', 'Code App'],
          workModes: ['Ask', 'Agent', 'Plan', 'Coding'],
          modelProviders: ['Auto', 'Agent'],
        },
      }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/settings')) {
      return new Response(JSON.stringify({ settings: { agentName: 'Agent' } }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/connectors')) {
      return new Response(JSON.stringify({ error: 'Assistant backend unavailable' }), { status: 502, headers: { 'Content-Type': 'application/json' } });
    }
    return new Response(JSON.stringify({}), { status: 200, headers: { 'Content-Type': 'application/json' } });
  }) as any;

  renderAssistantPage();
  fireEvent.click(await screen.findByRole('button', { name: 'Connectors' }));

  expect(await screen.findByText('Assistant backend unavailable')).toBeDefined();
  expect(screen.queryByText('GitHub')).toBeNull();
  expect(screen.queryByText('Slack')).toBeNull();
});


test('renders empty Assistant state without seeded demo records', async () => {
  global.fetch = vi.fn(async (url: RequestInfo | URL) => {
    const urlString = typeof url === 'string' ? url : url.toString();
    if (urlString.includes('/api/v1/assistant/tasks')) {
      return new Response(JSON.stringify({
        tasks: [],
        capabilities: {
          outputFormats: ['Document', 'Presentation', 'PDF', 'Code App'],
          workModes: ['Ask', 'Agent', 'Plan', 'Coding'],
          modelProviders: ['Auto', 'Agent'],
        },
      }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    return new Response(JSON.stringify({}), { status: 200, headers: { 'Content-Type': 'application/json' } });
  }) as any;

  renderAssistantPage();

  expect(await screen.findByRole('heading', { name: 'Agent Assistant' })).toBeDefined();
  expect(screen.getByText('0 tasks')).toBeDefined();
  expect(screen.getByText('No matching tasks.')).toBeDefined();
  expect(screen.queryByText("Create this week's operating brief")).toBeNull();
  expect(screen.queryByText('Organize Downloads by file type')).toBeNull();
});

test('shows resource error instead of connector demo records', async () => {
  global.fetch = vi.fn(async (url: RequestInfo | URL) => {
    const urlString = typeof url === 'string' ? url : url.toString();
    if (urlString.includes('/api/v1/assistant/tasks')) {
      return new Response(JSON.stringify({
        tasks: [],
        capabilities: {
          outputFormats: ['Document', 'Presentation', 'PDF', 'Code App'],
          workModes: ['Ask', 'Agent', 'Plan', 'Coding'],
          modelProviders: ['Auto', 'Agent'],
        },
      }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/settings')) {
      return new Response(JSON.stringify({ settings: { agentName: 'Agent' } }), { status: 200, headers: { 'Content-Type': 'application/json' } });
    }
    if (urlString.includes('/api/v1/assistant/connectors')) {
      return new Response(JSON.stringify({ error: 'Assistant backend unavailable' }), { status: 502, headers: { 'Content-Type': 'application/json' } });
    }
    return new Response(JSON.stringify({}), { status: 200, headers: { 'Content-Type': 'application/json' } });
  }) as any;

  renderAssistantPage();
  fireEvent.click(await screen.findByRole('button', { name: 'Connectors' }));

  expect(await screen.findByText('Assistant backend unavailable')).toBeDefined();
  expect(screen.queryByText('GitHub')).toBeNull();
  expect(screen.queryByText('Slack')).toBeNull();
});
