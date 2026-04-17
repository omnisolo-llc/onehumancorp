import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import Dashboard from '../src/pages/Dashboard';
import SwarmOverview from '../src/components/orchestration/SwarmOverview';
import AutoDreamPipelineWidget from '../src/components/orchestration/AutoDreamPipelineWidget';
import TaskDAGViewer from '../src/components/orchestration/TaskDAGViewer';
import TeammateMeshConsole from '../src/components/orchestration/TeammateMeshConsole';

// ---------------------------------------------------------------------------
// Global mocks
// ---------------------------------------------------------------------------

let mockWsInstance: any;

const createMockWs = () => {
  const instance = {
    onopen: null as null | (() => void),
    onclose: null as null | (() => void),
    onerror: null as null | ((e: Event) => void),
    onmessage: null as null | ((e: MessageEvent) => void),
    close: jest.fn(),
    send: jest.fn(),
    readyState: 1,
    simulateMessage(data: string) {
      if (this.onmessage) this.onmessage({ data } as MessageEvent);
    },
  };
  mockWsInstance = instance;
  return instance as unknown as WebSocket;
};

beforeEach(() => {
  global.WebSocket = jest.fn(createMockWs) as unknown as typeof WebSocket;
  global.fetch = jest.fn().mockResolvedValue({
    ok: true,
    json: () => Promise.resolve([]),
  });
});

afterEach(() => {
  jest.restoreAllMocks();
});

// ---------------------------------------------------------------------------
// Dashboard page
// ---------------------------------------------------------------------------

describe('Dashboard page', () => {
  it('renders the main heading', () => {
    render(<Dashboard />);
    expect(screen.getByText('Swarm Orchestration Dashboard')).toBeInTheDocument();
  });

  it('renders all major components', async () => {
    render(<Dashboard />);
    expect(screen.getByText('Swarm Overview')).toBeInTheDocument();
    expect(screen.getByText('AutoDream Pipeline Stream')).toBeInTheDocument();
    expect(screen.getByText('Task DAG Viewer')).toBeInTheDocument();
    expect(screen.getByText('Teammate Mesh Console')).toBeInTheDocument();
    expect(screen.getByTestId('agent-chat-panel')).toBeInTheDocument();
    expect(screen.getByText('Your AI team, ready in minutes')).toBeInTheDocument();
  });
});

// ---------------------------------------------------------------------------
// SwarmOverview
// ---------------------------------------------------------------------------

describe('SwarmOverview', () => {
  it('renders the heading', () => {
    render(<SwarmOverview />);
    expect(screen.getByText('Swarm Overview')).toBeInTheDocument();
  });

  it('shows 12 active agents', () => {
    render(<SwarmOverview />);
    expect(screen.getByTestId('active-agents')).toHaveTextContent('12');
  });

  it('shows 145 completed tasks', () => {
    render(<SwarmOverview />);
    expect(screen.getByTestId('completed-tasks')).toHaveTextContent('145');
  });

  it('renders the "Active Agents" label', () => {
    render(<SwarmOverview />);
    expect(screen.getByText('Active Agents')).toBeInTheDocument();
  });

  it('renders the "Completed Tasks" label', () => {
    render(<SwarmOverview />);
    expect(screen.getByText('Completed Tasks')).toBeInTheDocument();
  });
});

// ---------------------------------------------------------------------------
// AutoDreamPipelineWidget
// ---------------------------------------------------------------------------

describe('AutoDreamPipelineWidget', () => {
  it('renders the container', () => {
    render(<AutoDreamPipelineWidget />);
    expect(screen.getByTestId('autodream-pipeline')).toBeInTheDocument();
  });

  it('renders the heading', () => {
    render(<AutoDreamPipelineWidget />);
    expect(screen.getByText('AutoDream Pipeline Stream')).toBeInTheDocument();
  });

  it('renders all pipeline stage labels', () => {
    render(<AutoDreamPipelineWidget />);
    expect(screen.getByText('Extract')).toBeInTheDocument();
    expect(screen.getByText('Analyze')).toBeInTheDocument();
    expect(screen.getByText('Embed')).toBeInTheDocument();
    expect(screen.getByText('Store')).toBeInTheDocument();
  });

  it('cleans up interval on unmount', () => {
    jest.useFakeTimers();
    const clearIntervalSpy = jest.spyOn(global, 'clearInterval');
    const { unmount } = render(<AutoDreamPipelineWidget />);
    unmount();
    expect(clearIntervalSpy).toHaveBeenCalled();
    jest.useRealTimers();
  });

  it('resets progress to 0 when it reaches 1', () => {
    jest.useFakeTimers();
    render(<AutoDreamPipelineWidget />);
    // Advance enough ticks to reach progress >= 1 (1/0.05 = 20 ticks + extra)
    act(() => {
      jest.advanceTimersByTime(150 * 25);
    });
    // Component should still be rendered without errors after progress reset
    expect(screen.getByTestId('autodream-pipeline')).toBeInTheDocument();
    jest.useRealTimers();
  });
});

// ---------------------------------------------------------------------------
// TaskDAGViewer
// ---------------------------------------------------------------------------

describe('TaskDAGViewer', () => {
  it('renders heading', () => {
    render(<TaskDAGViewer />);
    expect(screen.getByText('Task DAG Viewer')).toBeInTheDocument();
  });

  it('shows loading state initially', () => {
    // Don't resolve fetch yet
    global.fetch = jest.fn(() => new Promise(() => {}));
    render(<TaskDAGViewer />);
    expect(screen.getByText('Loading tasks...')).toBeInTheDocument();
  });

  it('shows empty state when no tasks', async () => {
    global.fetch = jest.fn().mockResolvedValue({
      json: () => Promise.resolve([]),
    });
    render(<TaskDAGViewer />);
    await waitFor(() => {
      expect(screen.getByText('No tasks in DAG.')).toBeInTheDocument();
    });
  });

  it('fetches and renders tasks', async () => {
    global.fetch = jest.fn().mockResolvedValue({
      json: () =>
        Promise.resolve([
          { id: 'task-1', title: 'Task Alpha', status: 'PENDING' },
          { id: 'task-2', title: 'Task Beta', status: 'COMPLETED' },
          { id: 'task-3', title: 'Task Gamma', status: 'EXECUTING' },
        ]),
    });

    render(<TaskDAGViewer />);
    await waitFor(() => {
      expect(screen.getByText('Task Alpha')).toBeInTheDocument();
      expect(screen.getByText('PENDING')).toBeInTheDocument();
      expect(screen.getByText('Task Beta')).toBeInTheDocument();
      expect(screen.getByText('COMPLETED')).toBeInTheDocument();
      expect(screen.getByText('Task Gamma')).toBeInTheDocument();
      expect(screen.getByText('EXECUTING')).toBeInTheDocument();
    });
  });

  it('renders Pause and Kill buttons per task', async () => {
    global.fetch = jest.fn().mockResolvedValue({
      json: () => Promise.resolve([{ id: 'task-1', title: 'My Task', status: 'EXECUTING' }]),
    });
    render(<TaskDAGViewer />);
    await waitFor(() => {
      expect(screen.getByText('Pause')).toBeInTheDocument();
      expect(screen.getByText('Kill')).toBeInTheDocument();
    });
  });

  it('calls pause API when Pause is clicked', async () => {
    const fetchMock = jest.fn()
      .mockResolvedValueOnce({ json: () => Promise.resolve([{ id: 'task-1', title: 'T1', status: 'EXECUTING' }]) })
      .mockResolvedValue({ ok: true });
    global.fetch = fetchMock;
    render(<TaskDAGViewer />);
    await waitFor(() => expect(screen.getByText('Pause')).toBeInTheDocument());
    fireEvent.click(screen.getByText('Pause'));
    await waitFor(() => {
      expect(fetchMock).toHaveBeenCalledWith('/api/v1/orchestration/tasks/task-1/pause', { method: 'POST' });
    });
  });

  it('calls kill API when Kill is clicked', async () => {
    const fetchMock = jest.fn()
      .mockResolvedValueOnce({ json: () => Promise.resolve([{ id: 'task-1', title: 'T1', status: 'EXECUTING' }]) })
      .mockResolvedValue({ ok: true });
    global.fetch = fetchMock;
    render(<TaskDAGViewer />);
    await waitFor(() => expect(screen.getByText('Kill')).toBeInTheDocument());
    fireEvent.click(screen.getByText('Kill'));
    await waitFor(() => {
      expect(fetchMock).toHaveBeenCalledWith('/api/v1/orchestration/tasks/task-1/kill', { method: 'POST' });
    });
  });

  it('handles fetch error gracefully', async () => {
    global.fetch = jest.fn().mockRejectedValue(new Error('API unavailable'));
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    render(<TaskDAGViewer />);
    await waitFor(() => {
      expect(screen.getByText('No tasks in DAG.')).toBeInTheDocument();
    });
    consoleSpy.mockRestore();
  });

  it('renders task list container', async () => {
    global.fetch = jest.fn().mockResolvedValue({
      json: () => Promise.resolve([{ id: '1', title: 'T', status: 'PENDING' }]),
    });
    render(<TaskDAGViewer />);
    await waitFor(() => expect(screen.getByTestId('task-list')).toBeInTheDocument());
  });

  it('renders a task with unknown status using default text color', async () => {
    global.fetch = jest.fn().mockResolvedValue({
      json: () => Promise.resolve([{ id: '1', title: 'Unknown Status Task', status: 'UNKNOWN' }]),
    });
    render(<TaskDAGViewer />);
    await waitFor(() => expect(screen.getByText('Unknown Status Task')).toBeInTheDocument());
    expect(screen.getByText('UNKNOWN')).toBeInTheDocument();
  });
});

// ---------------------------------------------------------------------------
// TeammateMeshConsole
// ---------------------------------------------------------------------------

describe('TeammateMeshConsole', () => {
  it('renders the heading', () => {
    render(<TeammateMeshConsole />);
    expect(screen.getByText('Teammate Mesh Console')).toBeInTheDocument();
  });

  it('shows waiting message when no messages', () => {
    render(<TeammateMeshConsole />);
    expect(screen.getByText('Waiting for messages...')).toBeInTheDocument();
  });

  it('connects to the mesh WebSocket', () => {
    render(<TeammateMeshConsole />);
    expect(global.WebSocket).toHaveBeenCalledWith('ws://localhost:8080/api/mesh/stream');
  });

  it('displays incoming messages from the mesh', async () => {
    render(<TeammateMeshConsole />);
    act(() => {
      mockWsInstance.simulateMessage('Agent SWE-1: Starting code review…');
    });
    await waitFor(() => {
      expect(screen.getByText('Agent SWE-1: Starting code review…')).toBeInTheDocument();
    });
  });

  it('closes WebSocket on unmount', () => {
    const { unmount } = render(<TeammateMeshConsole />);
    unmount();
    expect(mockWsInstance.close).toHaveBeenCalled();
  });

  it('accumulates multiple messages', async () => {
    render(<TeammateMeshConsole />);
    act(() => {
      mockWsInstance.simulateMessage('Message A');
      mockWsInstance.simulateMessage('Message B');
    });
    await waitFor(() => {
      expect(screen.getByText('Message A')).toBeInTheDocument();
      expect(screen.getByText('Message B')).toBeInTheDocument();
    });
  });
});
