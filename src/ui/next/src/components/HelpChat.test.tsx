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

  it('opens the chat interface when floating button is clicked', () => {
    render(<HelpChat />);
    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    expect(screen.getByText('Ask AI Help')).toBeInTheDocument();
    expect(screen.getByText("Hi! I'm your AI Help Agent. Need help setting up your store or understanding payments?")).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Ask me anything...')).toBeInTheDocument();
  });

  it('sends a message and displays user and agent reply', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({
        reply: "Here is your mocked response",
        link: null
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
    });
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
});
