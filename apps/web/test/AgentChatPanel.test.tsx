import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import AgentChatPanel, { ChatMessage } from '../src/components/orchestration/AgentChatPanel';
import '@testing-library/jest-dom';

// ---------------------------------------------------------------------------
// WebSocket mock helpers
// ---------------------------------------------------------------------------

interface MockWebSocketInstance {
  onopen: (() => void) | null;
  onclose: (() => void) | null;
  onerror: ((e: Event) => void) | null;
  onmessage: ((e: MessageEvent) => void) | null;
  close: jest.Mock;
  send: jest.Mock;
  readyState: number;
  /** Simulate an incoming message from the "agent". */
  simulateMessage: (data: string) => void;
  /** Simulate a connection open event. */
  simulateOpen: () => void;
  /** Simulate a connection close event. */
  simulateClose: () => void;
}

let mockSocketInstance: MockWebSocketInstance;

const createMockWebSocket = () => {
  const instance: MockWebSocketInstance = {
    onopen: null,
    onclose: null,
    onerror: null,
    onmessage: null,
    close: jest.fn(),
    send: jest.fn(),
    readyState: WebSocket.CONNECTING,
    simulateMessage(data: string) {
      if (this.onmessage) {
        this.onmessage({ data } as MessageEvent);
      }
    },
    simulateOpen() {
      this.readyState = WebSocket.OPEN;
      if (this.onopen) this.onopen();
    },
    simulateClose() {
      this.readyState = WebSocket.CLOSED;
      if (this.onclose) this.onclose();
    },
  };
  mockSocketInstance = instance;
  return instance as unknown as WebSocket;
};

// ---------------------------------------------------------------------------
// Fetch mock helper
// ---------------------------------------------------------------------------

const mockFetchSuccess = () => {
  global.fetch = jest.fn().mockResolvedValue({
    ok: true,
    json: () => Promise.resolve({ status: 'queued' }),
  });
};

const mockFetchFailure = () => {
  global.fetch = jest.fn().mockRejectedValue(new Error('Network error'));
};

// ---------------------------------------------------------------------------
// Setup & teardown
// ---------------------------------------------------------------------------

beforeEach(() => {
  global.WebSocket = jest.fn(createMockWebSocket) as unknown as typeof WebSocket;
  mockFetchSuccess();
});

afterEach(() => {
  jest.restoreAllMocks();
});

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

describe('AgentChatPanel – rendering', () => {
  it('renders the chat panel with correct heading', () => {
    render(<AgentChatPanel />);
    expect(screen.getByTestId('agent-chat-panel')).toBeInTheDocument();
    expect(screen.getByText('Agent Chat')).toBeInTheDocument();
  });

  it('renders the chat input placeholder', () => {
    render(<AgentChatPanel />);
    expect(screen.getByTestId('chat-input')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Give your agents a task…')).toBeInTheDocument();
  });

  it('renders the send button', () => {
    render(<AgentChatPanel />);
    expect(screen.getByTestId('send-button')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /send task/i })).toBeInTheDocument();
  });

  it('shows empty state when no messages', () => {
    render(<AgentChatPanel />);
    expect(screen.getByText(/send a task to your agent team/i)).toBeInTheDocument();
  });

  it('renders ws-status indicator', () => {
    render(<AgentChatPanel />);
    expect(screen.getByTestId('ws-status')).toBeInTheDocument();
  });

  it('send button is disabled when input is empty', () => {
    render(<AgentChatPanel />);
    const btn = screen.getByTestId('send-button');
    expect(btn).toBeDisabled();
  });
});

// ---------------------------------------------------------------------------
// WebSocket lifecycle
// ---------------------------------------------------------------------------

describe('AgentChatPanel – WebSocket', () => {
  it('connects to the provided wsUrl on mount', () => {
    render(<AgentChatPanel wsUrl="ws://test-host:9999/ws" />);
    expect(global.WebSocket).toHaveBeenCalledWith('ws://test-host:9999/ws');
  });

  it('defaults to ws://localhost:8080/api/mesh/stream', () => {
    render(<AgentChatPanel />);
    expect(global.WebSocket).toHaveBeenCalledWith('ws://localhost:8080/api/mesh/stream');
  });

  it('shows "connected" status after WebSocket opens', async () => {
    render(<AgentChatPanel />);
    act(() => mockSocketInstance.simulateOpen());
    await waitFor(() => {
      expect(screen.getByTestId('ws-status')).toHaveTextContent('connected');
    });
  });

  it('shows "disconnected" status after WebSocket closes', async () => {
    render(<AgentChatPanel />);
    act(() => {
      mockSocketInstance.simulateOpen();
      mockSocketInstance.simulateClose();
    });
    await waitFor(() => {
      expect(screen.getByTestId('ws-status')).toHaveTextContent('disconnected');
    });
  });

  it('closes the WebSocket on unmount', () => {
    const { unmount } = render(<AgentChatPanel />);
    unmount();
    expect(mockSocketInstance.close).toHaveBeenCalled();
  });

  it('displays incoming text message from agent', async () => {
    render(<AgentChatPanel />);
    act(() => {
      mockSocketInstance.simulateOpen();
      mockSocketInstance.simulateMessage('Task accepted! Starting analysis…');
    });
    await waitFor(() => {
      expect(screen.getByText('Task accepted! Starting analysis…')).toBeInTheDocument();
    });
  });

  it('parses JSON message from agent and shows content field', async () => {
    render(<AgentChatPanel />);
    act(() => {
      mockSocketInstance.simulateOpen();
      mockSocketInstance.simulateMessage(JSON.stringify({ content: 'Analysis complete.' }));
    });
    await waitFor(() => {
      expect(screen.getByText('Analysis complete.')).toBeInTheDocument();
    });
  });

  it('parses JSON message with message field', async () => {
    render(<AgentChatPanel />);
    act(() => {
      mockSocketInstance.simulateOpen();
      mockSocketInstance.simulateMessage(JSON.stringify({ message: 'Done with task.' }));
    });
    await waitFor(() => {
      expect(screen.getByText('Done with task.')).toBeInTheDocument();
    });
  });

  it('handles malformed JSON gracefully by showing raw string', async () => {
    render(<AgentChatPanel />);
    act(() => {
      mockSocketInstance.simulateOpen();
      mockSocketInstance.simulateMessage('not-json-{{{');
    });
    await waitFor(() => {
      expect(screen.getByText('not-json-{{{')).toBeInTheDocument();
    });
  });

  it('multiple agent messages accumulate in order', async () => {
    render(<AgentChatPanel />);
    act(() => {
      mockSocketInstance.simulateOpen();
      mockSocketInstance.simulateMessage('Step 1 done.');
      mockSocketInstance.simulateMessage('Step 2 done.');
      mockSocketInstance.simulateMessage('Step 3 done.');
    });
    await waitFor(() => {
      expect(screen.getByText('Step 1 done.')).toBeInTheDocument();
      expect(screen.getByText('Step 2 done.')).toBeInTheDocument();
      expect(screen.getByText('Step 3 done.')).toBeInTheDocument();
    });
  });
});

// ---------------------------------------------------------------------------
// Sending messages (critical user journey)
// ---------------------------------------------------------------------------

describe('AgentChatPanel – sending messages (CUJ)', () => {
  it('sends the typed message and clears the input', async () => {
    render(<AgentChatPanel broadcastUrl="/api/test/broadcast" />);
    const input = screen.getByTestId('chat-input');

    fireEvent.change(input, { target: { value: 'Build a login page' } });
    expect(screen.getByTestId('send-button')).not.toBeDisabled();

    fireEvent.click(screen.getByTestId('send-button'));

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/test/broadcast', expect.objectContaining({
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ intent: 'Build a login page' }),
      }));
    });

    // Input should be cleared after sending
    expect(input).toHaveValue('');
  });

  it('displays the user message in the chat thread immediately', async () => {
    render(<AgentChatPanel />);
    const input = screen.getByTestId('chat-input');
    fireEvent.change(input, { target: { value: 'Analyze our Q3 metrics' } });
    fireEvent.click(screen.getByTestId('send-button'));

    await waitFor(() => {
      expect(screen.getByText('Analyze our Q3 metrics')).toBeInTheDocument();
    });

    // Verify it renders as a user message bubble
    const userBubbles = screen.getAllByTestId('message-user');
    expect(userBubbles.length).toBeGreaterThanOrEqual(1);
  });

  it('sends a message on Enter key press', async () => {
    render(<AgentChatPanel />);
    const input = screen.getByTestId('chat-input');
    fireEvent.change(input, { target: { value: 'Deploy to staging' } });
    fireEvent.keyDown(input, { key: 'Enter' });

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith(
        '/api/mesh/broadcast',
        expect.objectContaining({ body: JSON.stringify({ intent: 'Deploy to staging' }) }),
      );
    });
  });

  it('does not send on Shift+Enter (for multiline intent)', () => {
    render(<AgentChatPanel />);
    const input = screen.getByTestId('chat-input');
    fireEvent.change(input, { target: { value: 'line1' } });
    fireEvent.keyDown(input, { key: 'Enter', shiftKey: true });
    expect(global.fetch).not.toHaveBeenCalled();
  });

  it('does not send empty or whitespace-only messages', async () => {
    render(<AgentChatPanel />);
    const input = screen.getByTestId('chat-input');

    // Whitespace only
    fireEvent.change(input, { target: { value: '   ' } });
    expect(screen.getByTestId('send-button')).toBeDisabled();
    fireEvent.keyDown(input, { key: 'Enter' });
    expect(global.fetch).not.toHaveBeenCalled();
  });

  it('shows error message when broadcast API fails', async () => {
    mockFetchFailure();
    render(<AgentChatPanel />);
    const input = screen.getByTestId('chat-input');
    fireEvent.change(input, { target: { value: 'Run diagnostics' } });
    fireEvent.click(screen.getByTestId('send-button'));

    await waitFor(() => {
      expect(screen.getByText(/failed to send message/i)).toBeInTheDocument();
    });
  });

  it('full CUJ: user sends task, agent responds via WebSocket', async () => {
    render(<AgentChatPanel broadcastUrl="/api/mesh/broadcast" />);
    act(() => mockSocketInstance.simulateOpen());

    // 1. User types and sends a task
    const input = screen.getByTestId('chat-input');
    fireEvent.change(input, { target: { value: 'Write unit tests for the auth module' } });
    fireEvent.click(screen.getByTestId('send-button'));

    // 2. User message appears immediately
    await waitFor(() => {
      expect(screen.getByText('Write unit tests for the auth module')).toBeInTheDocument();
    });

    // 3. Verify the broadcast API was called with correct payload
    expect(global.fetch).toHaveBeenCalledWith('/api/mesh/broadcast', expect.objectContaining({
      body: JSON.stringify({ intent: 'Write unit tests for the auth module' }),
    }));

    // 4. Agent responds via WebSocket (mocked AI model response)
    act(() => {
      mockSocketInstance.simulateMessage(JSON.stringify({
        content: 'Understood! I will write unit tests for the auth module. Starting with JWT validation tests…',
        agentId: 'agent-swe-1',
      }));
    });

    // 5. Agent response appears in chat
    await waitFor(() => {
      expect(screen.getByText(/Understood! I will write unit tests/)).toBeInTheDocument();
    });

    // 6. Both message roles are shown
    expect(screen.getAllByTestId('message-user')).toHaveLength(1);
    expect(screen.getAllByTestId('message-agent')).toHaveLength(1);
  });

  it('second send button press sends another message', async () => {
    render(<AgentChatPanel />);
    const input = screen.getByTestId('chat-input');

    fireEvent.change(input, { target: { value: 'First task' } });
    fireEvent.click(screen.getByTestId('send-button'));
    await waitFor(() => expect(global.fetch).toHaveBeenCalledTimes(1));

    fireEvent.change(input, { target: { value: 'Second task' } });
    fireEvent.click(screen.getByTestId('send-button'));
    await waitFor(() => expect(global.fetch).toHaveBeenCalledTimes(2));

    expect(screen.getByText('First task')).toBeInTheDocument();
    expect(screen.getByText('Second task')).toBeInTheDocument();
  });
});

// ---------------------------------------------------------------------------
// Chat messages container
// ---------------------------------------------------------------------------

describe('AgentChatPanel – messages container', () => {
  it('chat-messages container is present', () => {
    render(<AgentChatPanel />);
    expect(screen.getByTestId('chat-messages')).toBeInTheDocument();
  });
});
