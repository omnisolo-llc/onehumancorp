import { render, screen, fireEvent, waitFor, within, act } from '@testing-library/react';
import { expect, test, vi, beforeEach } from 'vitest';
import AgentsPage from './page';

const mockFetch = vi.fn();
const eventSources: MockEventSource[] = [];

class MockEventSource {
  onmessage: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: Event) => void) | null = null;
  url: string;

  constructor(url: string) {
    this.url = url;
    eventSources.push(this);
  }

  close() {}

  emit(data: unknown) {
    this.onmessage?.({ data: JSON.stringify(data) } as MessageEvent);
  }
}

beforeEach(() => {
  vi.clearAllMocks();
  eventSources.length = 0;
  global.fetch = mockFetch;
  (globalThis as any).EventSource = MockEventSource;
  mockFetch.mockImplementation((url: string) => {
    if (url.includes('/api/agents/workflows')) {
      return Promise.resolve({ ok: true, json: async () => ({ workflows: [] }) });
    }
    if (url.includes('/api/agents/approvals')) {
      return Promise.resolve({ ok: true, json: async () => ({ pending_approvals: [], next_cursor: null }) });
    }
    if (url.includes('/api/agents/hire')) {
      return Promise.resolve({
        ok: true,
        status: 201,
        json: async () => ({
          id: 'agent-growth',
          status: 'running',
          agent_id: 'agent-growth',
          workflow_id: 'workflow-growth',
          message: 'Hired Growth Strategist',
        }),
      });
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
  });
});

test('replaces /agents with a Workbuddy-style Expert Center catalog', async () => {
  render(<AgentsPage />);

  expect(await screen.findByRole('heading', { name: 'Expert Center' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Browse experts' })).toBeDefined();
  expect(screen.getByRole('button', { name: 'Expert Teams' })).toBeDefined();
  expect(screen.getByText('Most used')).toBeDefined();
  expect(screen.getAllByText('Growth Strategist').length).toBeGreaterThan(0);
  expect(screen.getByText('Launch Team')).toBeDefined();
  expect(screen.getAllByText('Use cases').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Model').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Skills').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Connectors').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Memory').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Automations').length).toBeGreaterThan(0);
  expect(screen.getByRole('heading', { name: 'AI Departments' })).toBeDefined();
});

test('summons an expert into the task composer and starts a hire workflow', async () => {
  render(<AgentsPage />);

  const growthCard = await screen.findByTestId('expert-card-growth-strategist');
  fireEvent.click(within(growthCard).getByRole('button', { name: /Summon/i }));

  expect(screen.getByText('Growth Strategist is ready')).toBeDefined();
  expect(screen.getByDisplayValue('MiniMax-M3')).toBeDefined();
  expect(screen.getByText('Ask')).toBeDefined();
  expect(screen.getByText('Craft')).toBeDefined();
  expect(screen.getByText('Plan')).toBeDefined();

  fireEvent.change(screen.getByLabelText('Task prompt'), {
    target: { value: 'Create a launch plan for a weekend flash sale.' },
  });
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));

  await waitFor(() => {
    expect(mockFetch).toHaveBeenCalledWith(
      '/api/agents/hire',
      expect.objectContaining({
        method: 'POST',
        body: expect.stringContaining('Growth Strategist'),
      }),
    );
  });
  expect(await screen.findByText('workflow-growth')).toBeDefined();
});

test('shows result inspection and extension surfaces from Workbuddy', async () => {
  render(<AgentsPage />);

  expect(await screen.findAllByText('Artifacts')).toHaveLength(2);
  expect(screen.getAllByText('All files').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Diffs').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Preview').length).toBeGreaterThan(0);

  fireEvent.click(screen.getByRole('button', { name: 'Skills' }));
  expect(screen.getByText('Skill Market')).toBeDefined();
  expect(screen.getByText('Create skill from prompt')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Connectors' }));
  expect(screen.getByText('Connector Center')).toBeDefined();
  expect(screen.getByText('QQ Mail')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Automations' }));
  expect(screen.getByText('Scheduled Tasks')).toBeDefined();
  expect(screen.getByText('Weekly business review')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Memory' }));
  expect(screen.getByText('Conversation Memory')).toBeDefined();
  expect(screen.getByText('Import from ChatGPT or Claude')).toBeDefined();
});

test('covers every Workbuddy efficient-tip feature surface', async () => {
  render(<AgentsPage />);

  expect(await screen.findByLabelText('Context references')).toBeDefined();
  expect(screen.getByLabelText('Attachments')).toBeDefined();
  expect(screen.getByText('Screenshot')).toBeDefined();
  expect(screen.getByText('Output format')).toBeDefined();
  expect(screen.getByText('Task constraints')).toBeDefined();
  expect(screen.getByText('Custom provider')).toBeDefined();
  expect(screen.getAllByText('Local Ollama').length).toBeGreaterThan(0);
  expect(screen.getByText('Vision')).toBeDefined();
  expect(screen.getByText('Tool use')).toBeDefined();
  expect(screen.getByText('Long context')).toBeDefined();
  expect(screen.getByText('Work directory')).toBeDefined();
  expect(screen.getByText('Parallel tasks')).toBeDefined();

  const growthCard = screen.getByTestId('expert-card-growth-strategist');
  fireEvent.click(within(growthCard).getByRole('button', { name: 'Details' }));
  expect(screen.getByText('Expert detail')).toBeDefined();
  expect(screen.getByText('Summon into chat')).toBeDefined();
  expect(screen.getByText('Favorite')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Skills' }));
  expect(screen.getByText('Search installed skills')).toBeDefined();
  expect(screen.getByText('Disable skill')).toBeDefined();
  expect(screen.getByText('Uninstall skill')).toBeDefined();
  expect(screen.getByText('Bulk uninstall')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Connectors' }));
  expect(screen.getByText('Create custom connector')).toBeDefined();
  expect(screen.getByText('MCP endpoint')).toBeDefined();
  expect(screen.getByText('Notification channel')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Automations' }));
  expect(screen.getByText('Schedule rule')).toBeDefined();
  expect(screen.getByText('Execution history')).toBeDefined();
  expect(screen.getByText('Push notification')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Memory' }));
  expect(screen.getByText('Nightly summary')).toBeDefined();
  expect(screen.getByText('Edit memory')).toBeDefined();
  expect(screen.getByText('Forget selected')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Results' }));
  expect(screen.getAllByText('Share result').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Download file').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Copy to workspace').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Archive task').length).toBeGreaterThan(0);
  expect(screen.getAllByText('Unarchive').length).toBeGreaterThan(0);

  fireEvent.click(screen.getByRole('button', { name: 'Remote control' }));
  expect(screen.getByText('/summon Growth Strategist')).toBeDefined();
  expect(screen.getByText('Slack')).toBeDefined();
  expect(screen.getByText('Feishu')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: 'Data management' }));
  expect(screen.getByText('Shared files')).toBeDefined();
  expect(screen.getByText('Unshare queue')).toBeDefined();
});

test('preserves approvals and activity feed operations', async () => {
  render(<AgentsPage />);

  await waitFor(() => {
    expect(eventSources[0]?.url).toBe('/api/agents/events');
  });

  fireEvent.click(screen.getByRole('button', { name: 'Activity Feed' }));
  act(() => {
    eventSources[0].emit({
      id: 'evt-1',
      department: 'sales',
      description: 'Draft quote for priority lead',
      status: 'Draft',
    });
  });
  expect(await screen.findByText('Draft quote for priority lead')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: /Needs Approval/i }));
  expect(screen.getByText('Draft quote for priority lead')).toBeDefined();
  expect(screen.getByText('Approve & Send')).toBeDefined();
});

test('sends Workbuddy context, attachment, model, and output controls in the hire payload', async () => {
  render(<AgentsPage />);

  fireEvent.change(await screen.findByLabelText('Context references'), {
    target: { value: '@orders @inventory @launch-plan' },
  });
  fireEvent.change(screen.getByLabelText('Attachments'), {
    target: { value: 'launch-screenshot.png, revenue.csv' },
  });
  fireEvent.change(screen.getByLabelText('Custom provider'), {
    target: { value: 'https://llm.example.com/v1' },
  });
  fireEvent.change(screen.getByLabelText('Work directory'), {
    target: { value: '/workspace/launch-room' },
  });
  fireEvent.change(screen.getByLabelText('Output format'), {
    target: { value: 'Spreadsheet' },
  });
  fireEvent.change(screen.getByLabelText('Task constraints'), {
    target: { value: 'Budget under $500; draft before sending' },
  });
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));

  await waitFor(() => {
    expect(mockFetch).toHaveBeenCalledWith('/api/agents/hire', expect.objectContaining({ method: 'POST' }));
  });

  const hireCall = mockFetch.mock.calls.find(([url]) => url === '/api/agents/hire');
  expect(hireCall).toBeDefined();
  const payload = JSON.parse(hireCall![1].body);
  expect(payload).toMatchObject({
    contextReferences: '@orders @inventory @launch-plan',
    attachments: 'launch-screenshot.png, revenue.csv',
    customProvider: 'https://llm.example.com/v1',
    workDirectory: '/workspace/launch-room',
    outputFormat: 'Spreadsheet',
    taskConstraints: 'Budget under $500; draft before sending',
  });
});
