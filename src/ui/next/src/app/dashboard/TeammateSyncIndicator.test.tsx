import React from 'react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, act } from '@testing-library/react';
import { TeammateSyncIndicator } from './TeammateSyncIndicator';
import '@testing-library/jest-dom';

describe('TeammateSyncIndicator', () => {
  let mockWebSocket: any;

  beforeEach(() => {
    mockWebSocket = {
      onmessage: null,
      onerror: null,
      onclose: null,
      close: vi.fn(),
    };
    (global as any).WebSocket = function() { return mockWebSocket; };
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('does not render when there are no messages', () => {
    const { container } = render(<TeammateSyncIndicator />);
    expect(container).toBeEmptyDOMElement();
  });

  it('renders messages received via WebSocket', () => {
    render(<TeammateSyncIndicator />);

    act(() => {
      mockWebSocket.onmessage({
        data: JSON.stringify({
          action: 'task_completed',
          agent_id: 'Marketing Agent'
        })
      });
    });

    expect(screen.getByText('TEAMMATE SYNC')).toBeInTheDocument();
    expect(screen.getByText('The Promoter is briefing The Manager')).toBeInTheDocument();
  });

  it('renders generic message if agent is unknown', () => {
    render(<TeammateSyncIndicator />);

    act(() => {
      mockWebSocket.onmessage({
        data: JSON.stringify({
          action: 'some_action',
          agent_id: 'Unknown Agent'
        })
      });
    });

    expect(screen.getByText('Agent Unknown Agent completed an action: some_action')).toBeInTheDocument();
  });
});
