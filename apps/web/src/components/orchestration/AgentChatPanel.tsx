import React, { useEffect, useState, useRef, useCallback } from 'react';
import { theme } from '../../styles/theme';

export interface ChatMessage {
  id: string;
  role: 'user' | 'agent';
  content: string;
  timestamp: string;
  agentId?: string;
}

interface AgentChatPanelProps {
  /** WebSocket URL for receiving agent messages. */
  wsUrl?: string;
  /** REST endpoint for sending tasks to the agent team. */
  broadcastUrl?: string;
}

/**
 * AgentChatPanel – critical user journey component.
 *
 * Allows the user to type a task/intent in the input box and send it to the
 * agent team. Incoming agent responses arrive over a WebSocket and are
 * displayed in the conversation thread above the input area.
 *
 * Mock-friendly design:
 *  - `wsUrl` and `broadcastUrl` are props so tests can inject fakes.
 *  - WebSocket constructor is accessed via `window.WebSocket` so tests can
 *    replace it with a mock.
 */
const AgentChatPanel: React.FC<AgentChatPanelProps> = ({
  wsUrl = 'ws://localhost:8080/api/mesh/stream',
  broadcastUrl = '/api/mesh/broadcast',
}) => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [isSending, setIsSending] = useState(false);
  const [wsStatus, setWsStatus] = useState<'connecting' | 'connected' | 'disconnected'>('connecting');
  const scrollRef = useRef<HTMLDivElement>(null);
  const socketRef = useRef<WebSocket | null>(null);

  // Auto-scroll to the latest message
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [messages]);

  // Connect WebSocket for incoming agent messages
  useEffect(() => {
    const socket = new WebSocket(wsUrl);
    socketRef.current = socket;

    socket.onopen = () => setWsStatus('connected');
    socket.onclose = () => setWsStatus('disconnected');
    socket.onerror = () => setWsStatus('disconnected');

    socket.onmessage = (event: MessageEvent) => {
      let content: string;
      try {
        const parsed = JSON.parse(event.data);
        content = parsed.content ?? parsed.message ?? JSON.stringify(parsed);
      } catch {
        content = event.data;
      }
      setMessages((prev) => [
        ...prev,
        {
          id: `agent-${Date.now()}-${Math.random()}`,
          role: 'agent',
          content,
          timestamp: new Date().toISOString(),
        },
      ]);
    };

    return () => socket.close();
  }, [wsUrl]);

  const sendMessage = useCallback(async () => {
    const trimmed = inputValue.trim();
    if (!trimmed || isSending) return;

    const userMsg: ChatMessage = {
      id: `user-${Date.now()}`,
      role: 'user',
      content: trimmed,
      timestamp: new Date().toISOString(),
    };

    setMessages((prev) => [...prev, userMsg]);
    setInputValue('');
    setIsSending(true);

    try {
      await fetch(broadcastUrl, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ intent: trimmed }),
      });
    } catch {
      setMessages((prev) => [
        ...prev,
        {
          id: `error-${Date.now()}`,
          role: 'agent',
          content: 'Failed to send message. Please try again.',
          timestamp: new Date().toISOString(),
        },
      ]);
    } finally {
      setIsSending(false);
    }
  }, [inputValue, isSending, broadcastUrl]);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  const statusColor =
    wsStatus === 'connected' ? theme.colors.completed :
    wsStatus === 'connecting' ? theme.colors.pending :
    theme.colors.error;

  return (
    <div
      data-testid="agent-chat-panel"
      style={{
        ...theme.glassmorphism,
        ...theme.typography,
        padding: '24px',
        borderRadius: '16px',
        color: theme.colors.text,
        marginTop: '24px',
        display: 'flex',
        flexDirection: 'column',
        gap: '16px',
      }}
    >
      {/* Header */}
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <h2 style={{ margin: 0, fontWeight: 600 }}>Agent Chat</h2>
        <span
          data-testid="ws-status"
          style={{ fontSize: '12px', color: statusColor, display: 'flex', alignItems: 'center', gap: '6px' }}
        >
          <span
            style={{
              width: '8px',
              height: '8px',
              borderRadius: '50%',
              backgroundColor: statusColor,
              display: 'inline-block',
            }}
          />
          {wsStatus}
        </span>
      </div>

      {/* Message thread */}
      <div
        ref={scrollRef}
        data-testid="chat-messages"
        style={{
          height: '320px',
          overflowY: 'auto',
          background: 'rgba(0,0,0,0.3)',
          padding: '16px',
          borderRadius: '12px',
          border: '1px solid rgba(255,255,255,0.05)',
          display: 'flex',
          flexDirection: 'column',
          gap: '12px',
        }}
      >
        {messages.length === 0 ? (
          <div
            style={{
              display: 'flex',
              height: '100%',
              alignItems: 'center',
              justifyContent: 'center',
              color: 'rgba(255,255,255,0.4)',
              fontStyle: 'italic',
              fontSize: '14px',
            }}
          >
            Send a task to your agent team to get started.
          </div>
        ) : (
          messages.map((msg) => (
            <div
              key={msg.id}
              data-testid={`message-${msg.role}`}
              style={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: msg.role === 'user' ? 'flex-end' : 'flex-start',
              }}
            >
              <div
                style={{
                  maxWidth: '80%',
                  padding: '10px 14px',
                  borderRadius: msg.role === 'user' ? '16px 16px 4px 16px' : '16px 16px 16px 4px',
                  background:
                    msg.role === 'user'
                      ? 'rgba(52, 152, 219, 0.3)'
                      : 'rgba(255,255,255,0.08)',
                  border:
                    msg.role === 'user'
                      ? '1px solid rgba(52, 152, 219, 0.5)'
                      : '1px solid rgba(255,255,255,0.1)',
                  fontSize: '14px',
                  lineHeight: '1.5',
                }}
              >
                {msg.content}
              </div>
              <span
                style={{
                  fontSize: '11px',
                  color: 'rgba(255,255,255,0.3)',
                  marginTop: '4px',
                }}
              >
                {msg.role === 'user' ? 'You' : 'Agent'} · {new Date(msg.timestamp).toLocaleTimeString()}
              </span>
            </div>
          ))
        )}
      </div>

      {/* Input area */}
      <div style={{ display: 'flex', gap: '12px', alignItems: 'center' }}>
        <input
          data-testid="chat-input"
          type="text"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Give your agents a task…"
          disabled={isSending}
          aria-label="Task input"
          style={{
            flex: 1,
            padding: '12px 16px',
            background: 'rgba(255,255,255,0.05)',
            border: '1px solid rgba(255,255,255,0.1)',
            borderRadius: '12px',
            color: theme.colors.text,
            fontSize: '14px',
            outline: 'none',
          }}
        />
        <button
          data-testid="send-button"
          onClick={sendMessage}
          disabled={isSending || !inputValue.trim()}
          aria-label="Send task"
          style={{
            padding: '12px 20px',
            background: 'rgba(52, 152, 219, 0.3)',
            border: '1px solid rgba(52, 152, 219, 0.5)',
            borderRadius: '12px',
            color: theme.colors.text,
            fontSize: '14px',
            cursor: isSending || !inputValue.trim() ? 'not-allowed' : 'pointer',
            opacity: isSending || !inputValue.trim() ? 0.5 : 1,
            transition: 'opacity 0.2s',
          }}
        >
          {isSending ? 'Sending…' : 'Send'}
        </button>
      </div>
    </div>
  );
};

export default AgentChatPanel;
