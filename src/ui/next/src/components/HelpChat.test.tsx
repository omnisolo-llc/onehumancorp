import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { HelpChat } from './HelpChat';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('HelpChat Component', () => {

  beforeEach(() => {
    global.fetch = vi.fn() as any;
    // Mock scrollIntoView
    Element.prototype.scrollIntoView = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders nothing in E2E mode', () => {
    process.env.NEXT_PUBLIC_E2E = 'true';
    const { container } = render(<HelpChat />);
    expect(container.firstChild).toBeNull();
    process.env.NEXT_PUBLIC_E2E = undefined;
  });

  it('opens chat when floating button is clicked', async () => {
    render(<HelpChat />);

    const button = screen.getByLabelText('Open help chat');
    fireEvent.click(button);

    await waitFor(() => {
        expect(screen.getByText('Help Agent')).toBeInTheDocument();
        expect(screen.getByPlaceholderText('Ask me anything...')).toBeInTheDocument();
    });
  });

  it('closes chat when close button is clicked', async () => {
    render(<HelpChat />);

    fireEvent.click(screen.getByLabelText('Open help chat'));

    await waitFor(() => {
        expect(screen.getByLabelText('Close help chat')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByLabelText('Close help chat'));

    await waitFor(() => {
        expect(screen.queryByText('Help Agent')).not.toBeInTheDocument();
    });
  });

  it('handles sending a valid message and receiving a valid response', async () => {
    (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => ({
            reply: 'This is an AI response',
            link: { url: 'https://example.com', title: 'Example' }
        })
    });

    render(<HelpChat />);

    fireEvent.click(screen.getByLabelText('Open help chat'));

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'How do I add a product?' } });

    fireEvent.click(screen.getByLabelText('Send message'));

    // Check user message
    expect(screen.getByText('How do I add a product?')).toBeInTheDocument();

    // Check bot response
    await waitFor(() => {
        expect(screen.getByText('This is an AI response')).toBeInTheDocument();
        expect(screen.getByText('Example')).toBeInTheDocument();
        expect(screen.getByText('Example').closest('a')).toHaveAttribute('href', 'https://example.com');
    });
  });

  it('handles sending empty message', async () => {
    render(<HelpChat />);

    fireEvent.click(screen.getByLabelText('Open help chat'));

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: '   ' } });

    fireEvent.submit(input.closest('form')!);

    // Fetch should not be called
    expect(global.fetch).not.toHaveBeenCalled();
  });

  it('handles fetch error (network failure)', async () => {
    (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));

    render(<HelpChat />);

    fireEvent.click(screen.getByLabelText('Open help chat'));

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Test message' } });

    fireEvent.click(screen.getByLabelText('Send message'));

    await waitFor(() => {
        expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('handles non-ok response', async () => {
    (global.fetch as any).mockResolvedValueOnce({ ok: false });

    render(<HelpChat />);

    fireEvent.click(screen.getByLabelText('Open help chat'));

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Test message' } });

    fireEvent.click(screen.getByLabelText('Send message'));

    await waitFor(() => {
        expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('handles invalid response format (not an object)', async () => {
    (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => ("invalid")
    });

    render(<HelpChat />);

    fireEvent.click(screen.getByLabelText('Open help chat'));

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Test message' } });

    fireEvent.click(screen.getByLabelText('Send message'));

    await waitFor(() => {
        expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('handles invalid response format (missing reply)', async () => {
    (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => ({ no_reply: 'here' })
    });

    render(<HelpChat />);

    fireEvent.click(screen.getByLabelText('Open help chat'));

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Test message' } });

    fireEvent.click(screen.getByLabelText('Send message'));

    await waitFor(() => {
        expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('handles invalid response format (reply is not string)', async () => {
    (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => ({ reply: 123 })
    });

    render(<HelpChat />);

    fireEvent.click(screen.getByLabelText('Open help chat'));

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Test message' } });

    fireEvent.click(screen.getByLabelText('Send message'));

    await waitFor(() => {
        expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('handles response with invalid link URL', async () => {
    (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => ({
            reply: 'Valid reply',
            link: { url: 'javascript:alert(1)', title: 'Click me' }
        })
    });

    render(<HelpChat />);

    fireEvent.click(screen.getByLabelText('Open help chat'));

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Test message' } });

    fireEvent.click(screen.getByLabelText('Send message'));

    await waitFor(() => {
        expect(screen.getByText('Valid reply')).toBeInTheDocument();
        // Link should NOT be rendered
        expect(screen.queryByText('Click me')).not.toBeInTheDocument();
    });
  });

  it('handles response with invalid link title', async () => {
    (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => ({
            reply: 'Valid reply',
            link: { url: 'https://example.com', title: '' }
        })
    });

    render(<HelpChat />);

    fireEvent.click(screen.getByLabelText('Open help chat'));

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Test message' } });

    fireEvent.click(screen.getByLabelText('Send message'));

    await waitFor(() => {
        expect(screen.getByText('Valid reply')).toBeInTheDocument();
        // Link should NOT be rendered
        const link = document.querySelector('a');
        expect(link).toBeNull();
    });
  });
});
