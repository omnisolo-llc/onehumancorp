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
    ) as jest.Mock;
  });

  afterEach(() => {
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it('renders the floating button by default', () => {
    render(<HelpChat />);
    expect(screen.getByRole('button', { name: 'Open help chat' })).toBeInTheDocument();
    expect(screen.queryByRole('heading', { name: 'Ask AI Help' })).not.toBeInTheDocument();
  });

  it('opens the chat interface when the button is clicked', () => {
    render(<HelpChat />);
    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    expect(screen.getByRole('heading', { name: 'Ask AI Help' })).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Ask me anything...')).toBeInTheDocument();
  });

  it('closes the chat when the close button is clicked', () => {
    render(<HelpChat />);

    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));
    expect(screen.getByRole('heading', { name: 'Ask AI Help' })).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Close help chat' }));
    expect(screen.queryByRole('heading', { name: 'Ask AI Help' })).not.toBeInTheDocument();
  });

  it('sends a message and displays the response', async () => {
    render(<HelpChat />);
    const user = userEvent.setup({ delay: null });

    // Open chat
    fireEvent.click(screen.getByRole('button', { name: 'Open help chat' }));

    // Type and submit message
    const input = screen.getByPlaceholderText('Ask me anything...');
    const submitBtn = screen.getByRole('button', { name: 'Send message' });

    await user.type(input, 'Test message');

    act(() => {
      fireEvent.click(submitBtn);
    });

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

    const input = screen.getByPlaceholderText('Ask me anything...');
    const submitBtn = screen.getByRole('button', { name: 'Send message' });

    await user.type(input, 'Error msg');

    act(() => {
      fireEvent.click(submitBtn);
    });

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

    const input = screen.getByPlaceholderText('Ask me anything...');
    const submitBtn = screen.getByRole('button', { name: 'Send message' });

    await user.type(input, 'Timeout msg');

    act(() => {
      fireEvent.click(submitBtn);
    });

    await waitFor(() => {
      expect(screen.getByText("Sorry, the connection timed out. Please try again later or check your network connection.")).toBeInTheDocument();
    });
  });
});
