import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { HelpChat } from './HelpChat';
import userEvent from '@testing-library/user-event';

describe('HelpChat', () => {
  beforeEach(() => {
    // Reset process.env.NEXT_PUBLIC_E2E for consistent test behavior
    process.env.NEXT_PUBLIC_E2E = 'false';
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders closed chat button initially', () => {
    render(<HelpChat />);
    expect(screen.getByRole('button', { name: 'Open help chat' })).toBeVisible();
    expect(screen.queryByRole('button', { name: 'Close help chat' })).not.toBeInTheDocument();
  });

  it('opens chat interface on button click', async () => {
    const user = userEvent.setup();
    render(<HelpChat />);

    const openBtn = screen.getByRole('button', { name: 'Open help chat' });
    await user.click(openBtn);

    expect(screen.getByText('Ask AI Help')).toBeVisible();
    expect(screen.getByRole('button', { name: 'Close help chat' })).toBeVisible();
    expect(screen.queryByRole('button', { name: 'Open help chat' })).not.toBeInTheDocument();
  });

  it('sends message and receives response successfully', async () => {
    const mockReply = { reply: 'Here is how to do X.' };
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockReply),
    });

    const user = userEvent.setup();
    render(<HelpChat />);

    // Open chat
    await user.click(screen.getByRole('button', { name: 'Open help chat' }));

    // Type message
    const input = screen.getByPlaceholderText('Ask me anything...');
    await user.type(input, 'How to do X?');

    // Send
    const sendBtn = screen.getByRole('button', { name: 'Send message' });
    await user.click(sendBtn);

    // Verify user message appears immediately
    expect(screen.getByText('How to do X?')).toBeVisible();

    // Verify fetch was called correctly
    expect(global.fetch).toHaveBeenCalledWith('/api/chat', expect.objectContaining({
      method: 'POST',
      body: JSON.stringify({ message: 'How to do X?' }),
    }));

    // Wait for bot response
    await waitFor(() => {
      expect(screen.getByText('Here is how to do X.')).toBeVisible();
    });
  });

  it('handles network error and displays fallback message', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network failure'));

    const user = userEvent.setup();
    render(<HelpChat />);

    // Open chat
    await user.click(screen.getByRole('button', { name: 'Open help chat' }));

    // Type and send message
    await user.type(screen.getByPlaceholderText('Ask me anything...'), 'Hello');
    await user.click(screen.getByRole('button', { name: 'Send message' }));

    // Wait for fallback error message
    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeVisible();
    });
  });

  it('handles timeout error and displays timeout message', async () => {
    const abortError = new Error('AbortError');
    abortError.name = 'AbortError';
    global.fetch = vi.fn().mockRejectedValue(abortError);

    const user = userEvent.setup();
    render(<HelpChat />);

    await user.click(screen.getByRole('button', { name: 'Open help chat' }));
    await user.type(screen.getByPlaceholderText('Ask me anything...'), 'Hello');
    await user.click(screen.getByRole('button', { name: 'Send message' }));

    await waitFor(() => {
      expect(screen.getByText("Sorry, the connection timed out. Please try again later or check your network connection.")).toBeVisible();
    });
  });

  it('does not send empty messages', async () => {
    global.fetch = vi.fn();
    const user = userEvent.setup();
    render(<HelpChat />);

    await user.click(screen.getByRole('button', { name: 'Open help chat' }));
    await user.click(screen.getByRole('button', { name: 'Send message' }));

    expect(global.fetch).not.toHaveBeenCalled();
  });

  it('closes chat when close button is clicked', async () => {
    const user = userEvent.setup();
    render(<HelpChat />);

    await user.click(screen.getByRole('button', { name: 'Open help chat' }));
    expect(screen.getByText('Ask AI Help')).toBeVisible();

    await user.click(screen.getByRole('button', { name: 'Close help chat' }));
    expect(screen.queryByText('Ask AI Help')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Open help chat' })).toBeVisible();
  });
});
