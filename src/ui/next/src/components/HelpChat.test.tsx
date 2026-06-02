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

    expect(screen.getByText('Help Agent')).toBeInTheDocument();
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

  it('handles invalid API response missing reply', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({ some_other_key: "value" })
    }));

    render(<HelpChat />);
    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Break it' } });
    fireEvent.click(input.closest('form')!.querySelector('button[type="submit"]')!);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('handles invalid API response completely broken JSON', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => null
    }));

    render(<HelpChat />);
    fireEvent.click(screen.getByText('Ask anything').closest('button')!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Break it more' } });
    fireEvent.click(input.closest('form')!.querySelector('button[type="submit"]')!);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('can close the chat window', async () => {
    render(<HelpChat />);

    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);
    expect(screen.getByPlaceholderText('Ask me anything...')).toBeInTheDocument();

    const closeBtn = screen.getByLabelText('Close help chat');
    fireEvent.click(closeBtn);

    await waitFor(() => {
      expect(screen.queryByPlaceholderText('Ask me anything...')).not.toBeInTheDocument();
    });
  });

  it('ignores empty message submission', async () => {
    render(<HelpChat />);
    fireEvent.click(screen.getByText('Ask anything').closest('button')!);

    const submitBtn = screen.getByLabelText('Send message');
    expect(submitBtn).toBeDisabled();

    // Attempt submitting form directly
    fireEvent.submit(screen.getByPlaceholderText('Ask me anything...').closest('form')!);

    // Messages count shouldn't change (only 1 welcome message)
    expect(screen.getAllByText(/Hi! I'm your AI Help Agent/)).toHaveLength(1);
  });

  it('handles chat response with link successfully', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({
        reply: "Here is your link",
        link: { url: "/help", title: "Click me" }
      })
    }));

    render(<HelpChat />);
    fireEvent.click(screen.getByText('Ask anything').closest('button')!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Give me a link' } });
    fireEvent.click(input.closest('form')!.querySelector('button[type="submit"]')!);

    await waitFor(() => {
      expect(screen.getByText('Here is your link')).toBeInTheDocument();
      expect(screen.getByText('Click me')).toBeInTheDocument();
      expect(screen.getByText('Click me')).toHaveAttribute('href', '/help');
    });
  });

  it('returns null when NEXT_PUBLIC_E2E is true', () => {
    const originalEnv = process.env.NEXT_PUBLIC_E2E;
    process.env.NEXT_PUBLIC_E2E = 'true';
    render(<HelpChat />);
    expect(screen.queryByText('Ask anything')).not.toBeInTheDocument();
    process.env.NEXT_PUBLIC_E2E = originalEnv;
  });
});
