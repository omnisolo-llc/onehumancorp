import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { HelpChat } from './HelpChat';
import { describe, it, expect, vi } from 'vitest';

// Mock the fetch call
global.fetch = vi.fn() as any;

describe('HelpChat Component', () => {
  const originalEnv = process.env;

  beforeEach(() => {
    vi.clearAllMocks();
    process.env = { ...originalEnv };
  });

  afterAll(() => {
    process.env = originalEnv;
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

  it('closes the chat interface when the close button is clicked', () => {
    render(<HelpChat />);

    // Open chat
    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    // Check it's open
    expect(screen.getByText('Help Agent')).toBeInTheDocument();

    // Click close button
    const closeBtn = screen.getByLabelText('Close help chat');
    fireEvent.click(closeBtn);

    // Check it's closed
    expect(screen.queryByText('Help Agent')).not.toBeInTheDocument();
  });

  it('sends a message and displays user and agent reply', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({
        reply: "Here is your mocked response",
        link: { url: "https://example.com", title: "Click me" }
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
    const submitBtn = screen.getByLabelText('Send message');
    fireEvent.click(submitBtn!);

    // Check user message is displayed immediately
    expect(screen.getByText('How do I add a product?')).toBeInTheDocument();

    // Check input is cleared
    expect(input).toHaveValue('');

    // Wait for agent reply
    await waitFor(() => {
      expect(screen.getByText('Here is your mocked response')).toBeInTheDocument();
      expect(screen.getByText('Click me')).toBeInTheDocument();
      expect(screen.getByText('Click me')).toHaveAttribute('href', 'https://example.com');
    });
  });

  it('handles invalid agent responses', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({
        invalid_format: true
      })
    }));

    render(<HelpChat />);

    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Is this invalid?' } });

    const submitBtn = screen.getByLabelText('Send message');
    fireEvent.click(submitBtn!);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('handles fetch errors gracefully', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.reject(new Error('Network error')));

    render(<HelpChat />);

    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Will this fail?' } });

    const submitBtn = screen.getByLabelText('Send message');
    fireEvent.click(submitBtn!);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('returns null when NEXT_PUBLIC_E2E is true', () => {
    process.env.NEXT_PUBLIC_E2E = 'true';
    const { container } = render(<HelpChat />);
    expect(container).toBeEmptyDOMElement();
  });

  it('closes on Escape key press', () => {
    render(<HelpChat />);

    // Open chat
    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    expect(screen.getByText('Help Agent')).toBeInTheDocument();

    const dialog = screen.getByRole('dialog', { name: 'Help Chat' });
    fireEvent.keyDown(dialog, { key: 'Escape', code: 'Escape' });

    expect(screen.queryByText('Help Agent')).not.toBeInTheDocument();
  });
});
