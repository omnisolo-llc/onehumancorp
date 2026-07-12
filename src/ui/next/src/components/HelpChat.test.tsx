import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { HelpChat } from './HelpChat';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('HelpChat Component', () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ reply: 'Hello from AI' }),
      })
    ) as unknown as typeof fetch;
  });

  afterEach(() => {
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it('renders the floating button by default', () => {
    render(<HelpChat />);
    expect(screen.getByRole('button', { name: 'Open help chat' })).toBeInTheDocument();
    expect(screen.queryByRole('heading', { name: 'Ask anything' })).not.toBeInTheDocument();
  });

  it('opens the chat interface when the button is clicked', () => {
    render(<HelpChat />);
    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    expect(screen.getByRole('heading', { name: 'Ask anything' })).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument();
  });

  it('closes the chat when the close button is clicked', () => {
    render(<HelpChat />);

    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));
    expect(screen.getByRole('heading', { name: 'Ask anything' })).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Close help chat' }));
    expect(screen.queryByRole('heading', { name: 'Ask anything' })).not.toBeInTheDocument();
  });

  it('sends a message and displays the response', async () => {
    render(<HelpChat />);
    const user = userEvent.setup({ delay: null });

    // Open chat
    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    // Type and submit message
    const input = screen.getByPlaceholderText('Ask anything...');
    const submitBtn = screen.getByRole('button', { name: 'Send message' });

    await act(async () => {
      await user.type(input, 'Test message');
    });

    fireEvent.click(submitBtn);

    // Check if user message is immediately displayed
    expect(screen.getByText('Test message')).toBeInTheDocument();

    // Check input is disabled during loading
    expect(input).toBeDisabled();

    // Fast-forward fetch
    await waitFor(() => {
      expect(screen.getByText('Hello from AI')).toBeInTheDocument();
    });

    expect(global.fetch).toHaveBeenCalledWith('/api/chat', expect.objectContaining({
      method: 'POST',
      body: JSON.stringify({ message: 'Test message' })
    }));
  });

  it('handles fetch errors gracefully', async () => {
    global.fetch = vi.fn(() => Promise.reject(new Error('Network error')));

    render(<HelpChat />);
    const user = userEvent.setup({ delay: null });

    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    const input = screen.getByPlaceholderText('Ask anything...');
    const submitBtn = screen.getByRole('button', { name: 'Send message' });

    await act(async () => {
      await user.type(input, 'Error msg');
    });

    fireEvent.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('handles timeout errors gracefully', async () => {
    const error = new Error('AbortError');
    error.name = 'AbortError';
    global.fetch = vi.fn(() => Promise.reject(error));

    render(<HelpChat />);
    const user = userEvent.setup({ delay: null });

    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    const input = screen.getByPlaceholderText('Ask anything...');
    const submitBtn = screen.getByRole('button', { name: 'Send message' });

    await act(async () => {
      await user.type(input, 'Timeout msg');
    });

    fireEvent.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByText("Sorry, the connection timed out. Please try again later or check your network connection.")).toBeInTheDocument();
    });
  });

  it('clears the chat when the clear button is clicked', async () => {
    render(<HelpChat />);
    const user = userEvent.setup({ delay: null });

    // Open chat
    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    // Send a message
    const input = screen.getByPlaceholderText('Ask anything...');
    const submitBtn = screen.getByRole('button', { name: 'Send message' });
    await act(async () => {
      await user.type(input, 'Test message');
    });
    fireEvent.click(submitBtn);

    await waitFor(() => expect(screen.getByText('Test message')).toBeInTheDocument());

    // Check message is displayed
    expect(screen.getByText('Test message')).toBeInTheDocument();

    // Check clear button appears and click it
    const clearBtn = screen.getByRole('button', { name: 'Clear chat' });
    expect(clearBtn).toBeInTheDocument();

    fireEvent.click(clearBtn);

    // Verify messages are cleared (back to initial)
    expect(screen.queryByText('Test message')).not.toBeInTheDocument();
    expect(screen.getByText("Hi! I'm your AI Help Agent. Need help setting up your store or understanding payments?")).toBeInTheDocument();


    // Clear button should disappear
    expect(screen.queryByRole('button', { name: 'Clear chat' })).not.toBeInTheDocument();
  });

  it('opens the chat interface when open-help-chat event is dispatched', () => {
    render(<HelpChat />);
    expect(screen.queryByRole('heading', { name: 'Ask anything' })).not.toBeInTheDocument();

    act(() => {
      window.dispatchEvent(new CustomEvent('open-help-chat'));
    });

    expect(screen.getByRole('heading', { name: 'Ask anything' })).toBeInTheDocument();
  });
});

describe('HelpChat accessibility', () => {
  it('has dialog role and aria-labelledby', () => {
    render(<HelpChat />);
    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));
    const dialog = screen.getByRole('dialog');
    expect(dialog).toBeInTheDocument();
    expect(dialog).toHaveAttribute('aria-labelledby', 'ai-chat-header-title');
    expect(dialog).toHaveAttribute('aria-modal', 'false');
  });
  it('has polite aria-live region for messages', () => {
    render(<HelpChat />);
    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));
    const log = screen.getByRole('log');
    expect(log).toBeInTheDocument();
    expect(log).toHaveAttribute('aria-live', 'polite');
  });
});

describe('HelpChat normalizeAgentReply and URL safety', () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
  });

  afterEach(() => {
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it('filters unsafe URLs and returns only text', async () => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({
          reply: 'Here is a link',
          link: { url: 'javascript:alert(1)', title: 'Click me' }
        }),
      })
    ) as unknown as typeof fetch;

    render(<HelpChat />);
    const user = userEvent.setup({ delay: null });

    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    const input = screen.getByPlaceholderText('Ask anything...');
    const submitBtn = screen.getByRole('button', { name: 'Send message' });

    await act(async () => {
      await user.type(input, 'Test unsafe link');
    });

    fireEvent.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByText('Here is a link')).toBeInTheDocument();
    });

    expect(screen.queryByText('Read the full article →')).not.toBeInTheDocument();
  });

  it('handles invalid chat responses by throwing an error that is caught', async () => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve(null),
      })
    ) as unknown as typeof fetch;

    render(<HelpChat />);
    const user = userEvent.setup({ delay: null });
    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    await act(async () => {
      await user.type(screen.getByPlaceholderText('Ask anything...'), 'Test null');
    });
    fireEvent.click(screen.getByRole('button', { name: 'Send message' }));

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });

    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ reply: '   ' }),
      })
    ) as unknown as typeof fetch;

    await act(async () => {
      await user.type(screen.getByPlaceholderText('Ask anything...'), 'Test empty reply');
    });
    fireEvent.click(screen.getByRole('button', { name: 'Send message' }));

    await waitFor(() => {
      expect(screen.getAllByText("Sorry, I'm having trouble connecting right now.").length).toBeGreaterThanOrEqual(1);
    });
  });

  it('scrolls to bottom when a new message is sent', async () => {
    const scrollIntoViewMock = vi.fn();
    window.HTMLElement.prototype.scrollIntoView = scrollIntoViewMock;

    render(<HelpChat />);
    const user = userEvent.setup({ delay: null });
    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    scrollIntoViewMock.mockClear();

    await act(async () => {
      await user.type(screen.getByPlaceholderText('Ask anything...'), 'Hello');
    });
    fireEvent.click(screen.getByRole('button', { name: 'Send message' }));

    expect(scrollIntoViewMock).toHaveBeenCalled();
  });
});

describe('HelpChat safe link handling', () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
  });

  afterEach(() => {
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it('renders a safe link when provided by the agent', async () => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({
          reply: 'Here is a link',
          link: { url: 'https://example.com/safe', title: 'Safe Link' }
        }),
      })
    ) as unknown as typeof fetch;

    render(<HelpChat />);
    const user = userEvent.setup({ delay: null });

    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    const input = screen.getByPlaceholderText('Ask anything...');
    const submitBtn = screen.getByRole('button', { name: 'Send message' });

    await act(async () => {
      await user.type(input, 'Test safe link');
    });

    fireEvent.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByText('Here is a link')).toBeInTheDocument();
    });

    const link = screen.getByText('Read the full article →');
    expect(link).toBeInTheDocument();
    expect(link).toHaveAttribute('href', 'https://example.com/safe');
  });
});

describe('HelpChat remaining branches', () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
  });
  afterEach(() => {
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it('handles forceChat in E2E environments', async () => {
    const originalEnv = process.env.NEXT_PUBLIC_E2E;
    process.env.NEXT_PUBLIC_E2E = 'true';

    const originalUrl = window.location.href;
    window.history.replaceState({}, '', '?test_chat=true');

    render(<HelpChat />);
    expect(screen.getByRole('button', { name: 'Open help chat' })).toBeInTheDocument();

    process.env.NEXT_PUBLIC_E2E = originalEnv;
    window.history.replaceState({}, '', originalUrl);
  });

  it('prevents submitting empty messages', async () => {
    render(<HelpChat />);
    const user = userEvent.setup({ delay: null });

    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    const submitBtn = screen.getByRole('button', { name: 'Send message' });

    expect(submitBtn).toBeDisabled();

    const input = screen.getByPlaceholderText('Ask anything...');
    await act(async () => {
      await user.type(input, '   ');
    });

    expect(submitBtn).toBeDisabled();

    fireEvent.submit(submitBtn.closest('form')!);

    expect(screen.getByText("Hi! I'm your AI Help Agent. Need help setting up your store or understanding payments?")).toBeInTheDocument();
  });
});
