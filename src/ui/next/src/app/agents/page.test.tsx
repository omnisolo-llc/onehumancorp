import { render, screen, fireEvent, waitFor, within, act } from '@testing-library/react';
import { expect, test, vi, beforeEach, afterEach } from 'vitest';
import AgentsPage from './page';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installOnboardingLocks } from '../onboarding/testLocks';
import { TooltipProvider } from '../../components/TooltipRegistry';

const mockFetch = vi.fn();
const eventSources: MockWebSocket[] = [];

class MockWebSocket {
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
  installOnboardingLocks(); localStorage.clear(); notifyQueueIdentityChange();
  eventSources.length = 0;
  global.fetch = mockFetch;
  vi.stubGlobal('WebSocket', MockWebSocket);
  vi.stubGlobal('EventSource', class { addEventListener() {} close() {} });
  mockFetch.mockImplementation((url: string) => {
    if (url.endsWith('/session-identity')) return Promise.resolve(Response.json({ userId: 'agents-owner', tenantId: 'agents-tenant', expiresAt: Date.now() + 60_000 }));
    if (url === '/api/v1/agents/execution-policy') return Promise.resolve(Response.json({ available: true, mode: 'text_analysis', workspace_access: false, tools: [], policy: { provider: 'ollama', model: 'configured-model', max_output_tokens: 2048 } }));
    if (url.includes('/api/v1/agents/workflows')) {
      return Promise.resolve({ ok: true, json: async () => ({ workflows: [] }) });
    }
    if (url.includes('/api/v1/agents/approvals')) {
      return Promise.resolve({
        ok: true,
        json: async () => ({
          pending_approvals: [{
            id: 'evt-1',
            department: 'sales',
            description: 'Draft quote for priority lead',
            status: 'Draft',
          }],
          next_cursor: null,
        }),
      });
    }
    if (url.includes('/api/v1/memory')) {
      return Promise.resolve({ ok: true, json: async () => ([]) });
    }
    if (url.includes('/api/v1/agents/hire')) {
      return Promise.resolve({
        ok: true,
        status: 201,
        json: async () => ({
          id: 'agent-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
          status: 'queued',
          agent_id: 'agent-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
          workflow_id: 'aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa',
          message: 'Hired Growth Strategist',
        }),
      });
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
  });
});

test('replaces /agents with a Workbuddy-style Expert Center catalog', async () => {
  await act(async () => { render(<TooltipProvider><AgentsPage /></TooltipProvider>); });

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
  await act(async () => { render(<TooltipProvider><AgentsPage /></TooltipProvider>); });

  const growthCard = await screen.findByTestId('expert-card-growth-strategist');
  fireEvent.click(within(growthCard).getByRole('button', { name: /Summon/i }));

  expect(screen.getByText('Growth Strategist is ready')).toBeDefined();
  expect(screen.getByLabelText('Model')).toHaveValue('Auto');
  expect(screen.getByText('Ask')).toBeDefined();
  expect(screen.getByText('Craft')).toBeDefined();
  expect(screen.getByText('Plan')).toBeDefined();

  fireEvent.change(screen.getByLabelText('Task prompt'), {
    target: { value: 'Create a launch plan for a weekend flash sale.' },
  });
  fireEvent.click(screen.getByRole('button', { name: 'Start task' }));

  await waitFor(() => {
    expect(mockFetch).toHaveBeenCalledWith(
      '/api/v1/agents/hire',
      expect.objectContaining({
        method: 'POST',
        body: expect.stringContaining('Growth Strategist'),
      }),
    );
  });
  expect(await screen.findByText(/Text analysis queued: aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa/)).toBeVisible();
});

test('shows result inspection and extension surfaces from Workbuddy', async () => {
  await act(async () => { render(<TooltipProvider><AgentsPage /></TooltipProvider>); });

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
  expect(screen.getByText(/Scheduled runs and execution history are unavailable/)).toBeDefined();

  await act(async () => {
    fireEvent.click(screen.getByRole('button', { name: 'Memory' }));
  });
  await waitFor(() => expect(screen.getByText('Consolidated Memory')).toBeDefined());
});

test('covers every Workbuddy efficient-tip feature surface', async () => {
  await act(async () => { render(<TooltipProvider><AgentsPage /></TooltipProvider>); });

  expect(await screen.findByLabelText('Context references')).toBeDefined();
  expect(screen.getByLabelText('Attachments')).toBeDefined();
  expect(screen.getByText('Screenshot')).toBeDefined();
  expect(screen.getByText('Output format')).toBeDefined();
  expect(screen.getByText('Task constraints')).toBeDefined();
  expect(screen.getByText('Custom provider')).toBeDefined();
  expect(screen.getAllByText('Local Ollama').length).toBeGreaterThan(0);
  expect(screen.getByText('Text analysis')).toBeVisible();
  expect(screen.getByText(/Workspace actions, expert teams, skills and connectors are not executable/)).toBeVisible();
  expect(screen.getByText('Work directory')).toBeDefined();
  expect(screen.queryByText('Parallel tasks')).not.toBeInTheDocument();

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

  await act(async () => {
    fireEvent.click(screen.getByRole('button', { name: 'Memory' }));
  });
  await waitFor(() => expect(screen.getByText('Consolidated Memory')).toBeDefined());

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

test('preserves approvals and activity feed operations without an unauthenticated socket', async () => {
  mockFetch.mockImplementation((url: string) => {
    if (url.endsWith('/session-identity')) return Promise.resolve(Response.json({ userId: 'agents-owner', tenantId: 'agents-tenant', expiresAt: Date.now() + 60_000 }));
    if (url === '/api/v1/agents/execution-policy') return Promise.resolve(Response.json({ available: true, mode: 'text_analysis', workspace_access: false, tools: [], policy: { provider: 'ollama', model: 'configured-model', max_output_tokens: 2048 } }));
    if (url.includes('/api/v1/agents/approvals/activity')) {
      return Promise.resolve({
        ok: true,
        json: async () => ({
          pending_approvals: [{
            id: 'evt-1',
            department: 'sales',
            description: 'Draft quote for priority lead',
            status: 'Draft',
          }],
        }),
      });
    }
    if (url.includes('/api/v1/agents/approvals')) {
      return Promise.resolve({
        ok: true,
        json: async () => ({
          pending_approvals: [{
            id: 'evt-1',
            department: 'sales',
            description: 'Draft quote for priority lead',
            status: 'Draft',
          }],
          next_cursor: null,
        }),
      });
    }
    if (url.includes('/api/v1/agents/workflows')) {
      return Promise.resolve({ ok: true, json: async () => ({ workflows: [] }) });
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
  });

  await act(async () => { render(<TooltipProvider><AgentsPage /></TooltipProvider>); });

  await waitFor(() => {
    expect(eventSources).toHaveLength(0);
  });

  fireEvent.click(screen.getByRole('button', { name: 'Activity Feed' }));
  expect(await screen.findByText('Draft quote for priority lead')).toBeDefined();

  fireEvent.click(screen.getByRole('button', { name: /Needs Approval/i }));
  expect(screen.getByText('Draft quote for priority lead')).toBeDefined();
  expect(screen.getByText('Approve & Send')).toBeDefined();
});

test('keeps unsupported context, attachment, endpoint and output selections visible without submitting them', async () => {
  await act(async () => { render(<TooltipProvider><AgentsPage /></TooltipProvider>); });

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

  expect(screen.getByRole('button', { name: 'Start task' })).toBeDisabled();
  expect(mockFetch.mock.calls.filter(([url]) => url === '/api/v1/agents/hire')).toHaveLength(0);
  expect(screen.getByLabelText('Attachments')).toHaveValue('launch-screenshot.png, revenue.csv');
  expect(screen.getByLabelText('Work directory')).toHaveValue('/workspace/launch-room');
  expect(screen.getByText(/Workspace, tools, connectors, attachments/)).toBeVisible();
});

test('supports interactive tab transitions and toggling grid extensions', async () => {
  await act(async () => { render(<TooltipProvider><AgentsPage /></TooltipProvider>); });

  // Initial tab is Browse experts. Go to Skills.
  fireEvent.click(screen.getByRole('button', { name: 'Skills' }));
  expect(screen.getByText('Skill Market')).toBeDefined();

  // Toggle skill inside grid to disable or enable it
  const skillButton = screen.getByRole('button', { name: /Web Research Installed/i });
  fireEvent.click(skillButton);
  expect(screen.getByRole('button', { name: /Web Research Enabled/i })).toBeDefined();
  fireEvent.click(screen.getByRole('button', { name: /Web Research Enabled/i }));
  expect(screen.getByRole('button', { name: /Web Research Installed/i })).toBeDefined();

  // Navigate to Connectors
  fireEvent.click(screen.getByRole('button', { name: 'Connectors' }));
  expect(screen.getByText('Connector Center')).toBeDefined();

  // Toggle connector
  const connectorButton = screen.getByRole('button', { name: /Stripe Connected/i });
  fireEvent.click(connectorButton);
  expect(screen.getByRole('button', { name: /Stripe Selected/i })).toBeDefined();
  fireEvent.click(screen.getByRole('button', { name: /Stripe Selected/i }));
  expect(screen.getByRole('button', { name: /Stripe Connected/i })).toBeDefined();
});

test('closes the paywall only after the trial API confirms activation', async () => {
  const openSpy = vi.fn();
  vi.stubGlobal('open', openSpy);

  await act(async () => { render(<TooltipProvider><AgentsPage /></TooltipProvider>); });

  // Click Toggle Pro Mode switch to trigger paywall
  fireEvent.click(screen.getByRole('button', { name: 'Toggle Pro Mode' }));

  // Paywall dialog should appear
  expect(screen.getByRole('heading', { name: 'Upgrade to Pro' })).toBeDefined();
  
  // Click share on X
  fireEvent.click(screen.getByText('Share on X to activate Pro'));
  await waitFor(() => {
    expect(mockFetch).toHaveBeenCalledWith('/api/v1/growth/trial-extension/claim', { method: 'POST' });
    expect(screen.queryByRole('heading', { name: 'Upgrade to Pro' })).toBeNull();
  });
  expect(openSpy).toHaveBeenCalled();
  vi.unstubAllGlobals();
});

test('operational department shortcuts expose their selection in the existing team panel', async () => {
  await act(async () => { render(<TooltipProvider><AgentsPage /></TooltipProvider>); });
  const manager = screen.getByRole('button', { name: 'The Manager' });
  const ambassador = screen.getByRole('button', { name: 'The Ambassador' });
  fireEvent.click(manager);
  expect(screen.getByRole('button', { name: 'My Team' })).toHaveAttribute('aria-pressed', 'true');
  expect(manager).toHaveAttribute('aria-pressed', 'true');
  expect(ambassador).toHaveAttribute('aria-pressed', 'false');
  const details = screen.getByRole('region', { name: 'Department details' });
  expect(within(details).getByRole('heading', { name: 'The Manager' })).toBeVisible();
  expect(within(details).queryByRole('heading', { name: 'The Ambassador' })).not.toBeInTheDocument();
  fireEvent.click(ambassador);
  expect(manager).toHaveAttribute('aria-pressed', 'false');
  expect(ambassador).toHaveAttribute('aria-pressed', 'true');
  expect(within(details).getByRole('heading', { name: 'The Ambassador' })).toBeVisible();
  expect(within(details).queryByRole('heading', { name: 'The Manager' })).not.toBeInTheDocument();
  fireEvent.click(within(details).getByRole('button', { name: 'Show all departments' }));
  expect(within(details).getByRole('heading', { name: 'The Manager' })).toBeVisible();
  expect(ambassador).toHaveAttribute('aria-pressed', 'false');
});

test('unsupported task media and result actions explain their unavailable state', async () => {
  await act(async () => { render(<TooltipProvider><AgentsPage /></TooltipProvider>); });
  for (const name of ['Voice Input', 'Refine Prompt', 'Screenshot']) {
    const button = name === 'Screenshot' ? screen.getByRole('button', { name }) : screen.getByTitle(name);
    expect(button).toBeDisabled();
    const reason = document.getElementById(button.getAttribute('aria-describedby')!);
    expect(reason).toBeVisible();
    expect(reason).toHaveTextContent(/not available|not configured/i);
  }
  fireEvent.click(screen.getByRole('button', { name: 'Results' }));
  for (const name of ['Share result', 'Download file', 'Copy to workspace', 'Archive task', 'Unarchive']) {
    for (const button of screen.getAllByRole('button', { name })) {
      expect(button).toBeDisabled();
      expect(document.getElementById(button.getAttribute('aria-describedby')!)).toHaveTextContent(/not available/i);
    }
  }
});

afterEach(() => vi.unstubAllGlobals());
