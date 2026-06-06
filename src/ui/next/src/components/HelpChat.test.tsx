import '@testing-library/jest-dom';

import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { HelpChat } from './HelpChat';
import { describe, it, expect, vi } from 'vitest';

// Mock the fetch call
global.fetch = vi.fn() as any;

describe('HelpChat Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the floating button initially', () => {
    render(<HelpChat />);
    expect(screen.getByText('Ask anything')).toBeInTheDocument();
  });

  it('opens the chat interface when floating button is clicked and can close', () => {
    render(<HelpChat />);
    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    expect(screen.getByText('Ask AI Help')).toBeInTheDocument();

    // Test close button
    const closeBtn = screen.getByLabelText('Close help chat');
    fireEvent.click(closeBtn);
    expect(screen.queryByText('Ask AI Help')).not.toBeInTheDocument();
  });

  it('sends a message and displays user and agent reply', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({
        reply: "Here is your mocked response",
        link: { url: "https://example.com", title: "Example Link" }
      })
    }));

    render(<HelpChat />);

    // Open chat
    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    // Type message
    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'How do I add a product?' } });

    // Submit
    const submitBtn = input.closest('form')!.querySelector('button[type="submit"]');
    fireEvent.click(submitBtn!);

    // Check user message is displayed immediately
    expect(screen.getByText('How do I add a product?')).toBeInTheDocument();

    // Check input is cleared
    expect(input).toHaveValue('');

    // Wait for agent reply
    await waitFor(() => {
      expect(screen.getByText('Here is your mocked response')).toBeInTheDocument();
      expect(screen.getByText('Example Link')).toBeInTheDocument();
    });
  });

  it('does not send empty messages', () => {
    render(<HelpChat />);
    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    const submitBtn = input.closest('form')!.querySelector('button[type="submit"]');

    fireEvent.click(submitBtn!);
    expect(global.fetch).not.toHaveBeenCalled();
  });

  it('handles fetch errors gracefully', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.reject(new Error('Network error')));

    render(<HelpChat />);

    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Will this fail?' } });

    const submitBtn = input.closest('form')!.querySelector('button[type="submit"]');
    fireEvent.click(submitBtn!);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('handles non-ok response', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: false
    }));

    render(<HelpChat />);

    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Will this fail too?' } });

    const submitBtn = input.closest('form')!.querySelector('button[type="submit"]');
    fireEvent.click(submitBtn!);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('ignores malformed responses', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => "invalid string response"
    }));

    render(<HelpChat />);
    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Will this throw?' } });
    fireEvent.submit(input.closest('form')!);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('ignores empty replies', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({ reply: "   " })
    }));

    render(<HelpChat />);
    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Empty space?' } });
    fireEvent.submit(input.closest('form')!);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

});
