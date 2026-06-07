import { render, screen, fireEvent, waitFor, within, act } from '@testing-library/react';
import { expect, test, vi, beforeEach } from 'vitest';
import AgentsPage from './page';
import * as TooltipContext from '@/components/TooltipContext';

vi.mock('@/components/TooltipContext', () => ({
  useTooltipContext: vi.fn(() => ({
    advance: vi.fn(),
  })),
}));

const mockFetch = vi.fn();
const eventSources: MockEventSource[] = [];

class MockEventSource {
  static CLOSED = 2;
  onmessage: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: Event) => void) | null = null;
  url: string;
  closed = false;

  constructor(url: string) {
    this.url = url;
    eventSources.push(this);
  }

  close() {
    this.closed = true;
  }

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
    if (url.includes('/api/agents/feed')) {
      return Promise.resolve({ ok: true, json: async () => ({ feed: [], next_cursor: null }) });
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

  fireEvent.click(await screen.findByRole('button', { name: /Hire Growth Strategist/i }));

  expect(screen.getByRole('heading', { name: 'Hire Growth Strategist' })).toBeDefined();
  expect(screen.getByText('Assign a specific goal to this expert.')).toBeDefined();

  fireEvent.change(screen.getByPlaceholderText('e.g., Increase repeat purchases by 15% this quarter'), {
    target: { value: 'Create a summer marketing campaign' },
  });

  fireEvent.click(screen.getByRole('button', { name: 'Hire Expert' }));

  expect(await screen.findByText('Hired Growth Strategist')).toBeDefined();
  expect(mockFetch).toHaveBeenCalledWith('/api/agents/hire', expect.objectContaining({
    method: 'POST',
    body: expect.stringContaining('Create a summer marketing campaign'),
  }));
});

test('pushes agent approval events into the activity feed in real time', async () => {
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

  fireEvent.click(await screen.findByRole('button', { name: /Hire Growth Strategist/i }));

  fireEvent.change(screen.getByLabelText('Context references'), {
    target: { value: 'https://example.com/guide' },
  });

  fireEvent.change(screen.getByLabelText('Model preference'), {
    target: { value: 'gpt-4o' },
  });

  const file = new File(['hello'], 'hello.png', { type: 'image/png' });
  const input = screen.getByLabelText('Attach files');
  Object.defineProperty(input, 'files', {
    value: [file]
  });
  fireEvent.change(input);

  fireEvent.click(screen.getByRole('button', { name: 'Hire Expert' }));

  await waitFor(() => {
    expect(mockFetch).toHaveBeenCalledWith('/api/agents/hire', expect.objectContaining({
      method: 'POST',
      body: expect.stringContaining('gpt-4o'),
    }));
  });
});
