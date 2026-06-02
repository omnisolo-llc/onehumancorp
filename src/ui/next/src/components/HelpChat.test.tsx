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


describe('HelpChat Helper Error Cases & Links', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders a link when provided', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({
        reply: "Here is your mocked response with link",
        link: { url: "/help", title: "Read more" }
      })
    }));

    render(<HelpChat />);

    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'How do I add a product?' } });

    const submitBtn = input.closest('form')!.querySelector('button[type="submit"]');
    fireEvent.click(submitBtn!);

    await waitFor(() => {
      expect(screen.getByText('Read more')).toBeInTheDocument();
      expect(screen.getByText('Read more').closest('a')).toHaveAttribute('href', '/help');
    });
  });

  it('handles invalid chat response (null)', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => null
    }));

    render(<HelpChat />);

    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Crash' } });

    const submitBtn = input.closest('form')!.querySelector('button[type="submit"]');
    fireEvent.click(submitBtn!);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('handles invalid chat reply (missing reply)', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({ link: { url: "/foo", title: "bar" } })
    }));

    render(<HelpChat />);

    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'Crash' } });

    const submitBtn = input.closest('form')!.querySelector('button[type="submit"]');
    fireEvent.click(submitBtn!);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('disables itself in E2E mode', () => {
    const originalEnv = process.env.NEXT_PUBLIC_E2E;
    process.env.NEXT_PUBLIC_E2E = 'true';
    const { container } = render(<HelpChat />);
    expect(container.firstChild).toBeNull();
    process.env.NEXT_PUBLIC_E2E = originalEnv;
  });

  it('handles scrollIntoView missing', async () => {
    const originalScrollIntoView = window.HTMLElement.prototype.scrollIntoView;
    const mockScroll = vi.fn();
    window.HTMLElement.prototype.scrollIntoView = mockScroll;

    render(<HelpChat />);
    const button = screen.getByText('Ask anything').closest('button');
    fireEvent.click(button!); // Need to open the chat to trigger the messagesEndRef render and scroll

    await waitFor(() => {
        expect(mockScroll).toHaveBeenCalled();
    });

    window.HTMLElement.prototype.scrollIntoView = originalScrollIntoView;
  });
});


describe('HelpChat UI interactions', () => {
  it('closes chat when close button is clicked', () => {
    render(<HelpChat />);
    // Open chat
    const openBtn = screen.getByText('Ask anything').closest('button');
    fireEvent.click(openBtn!);

    expect(screen.getByText('Ask AI Help')).toBeInTheDocument();

    // Close chat
    const closeBtn = screen.getByRole('button', { name: 'Close help chat' });
    fireEvent.click(closeBtn!);

    expect(screen.queryByText('Ask AI Help')).not.toBeInTheDocument();
  });
});


describe('HelpChat Edge Cases and Branches', () => {
  it('does nothing when sending empty message', () => {
    render(<HelpChat />);
    const openBtn = screen.getByText('Ask anything').closest('button');
    fireEvent.click(openBtn!);

    // empty message -> form submit does nothing
    const submitBtn = screen.getByRole('button', { name: 'Send message' });
    fireEvent.click(submitBtn!);

    // Still only 1 agent message
    const messages = screen.getAllByText(/Hi! I'm your AI Help Agent/);
    expect(messages.length).toBe(1);
  });

  it('handles link with http schema', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({
        reply: "http link",
        link: { url: "http://example.com", title: "HTTP" }
      })
    }));

    render(<HelpChat />);
    fireEvent.click(screen.getByText('Ask anything').closest('button')!);
    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'http' } });
    fireEvent.click(input.closest('form')!.querySelector('button[type="submit"]')!);

    await waitFor(() => {
      expect(screen.getByText('HTTP')).toBeInTheDocument();
    });
  });

  it('handles link with https schema', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({
        reply: "https link",
        link: { url: "https://example.com", title: "HTTPS" }
      })
    }));

    render(<HelpChat />);
    fireEvent.click(screen.getByText('Ask anything').closest('button')!);
    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'https' } });
    fireEvent.click(input.closest('form')!.querySelector('button[type="submit"]')!);

    await waitFor(() => {
      expect(screen.getByText('HTTPS')).toBeInTheDocument();
    });
  });

  it('ignores invalid link schemas', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({
        reply: "invalid link",
        link: { url: "javascript:alert(1)", title: "Hack" }
      })
    }));

    render(<HelpChat />);
    fireEvent.click(screen.getByText('Ask anything').closest('button')!);
    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'hack' } });
    fireEvent.click(input.closest('form')!.querySelector('button[type="submit"]')!);

    await waitFor(() => {
      expect(screen.getByText('invalid link')).toBeInTheDocument();
      expect(screen.queryByText('Hack')).not.toBeInTheDocument();
    });
  });

  it('ignores invalid link object structure', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({
        reply: "invalid link obj",
        link: "not an object" // link is not an object
      })
    }));

    render(<HelpChat />);
    fireEvent.click(screen.getByText('Ask anything').closest('button')!);
    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'hack2' } });
    fireEvent.click(input.closest('form')!.querySelector('button[type="submit"]')!);

    await waitFor(() => {
      expect(screen.getByText('invalid link obj')).toBeInTheDocument();
    });
  });
});


describe('HelpChat more edge cases', () => {
  it('handles valid chat reply with missing link key entirely', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({
        reply: "missing link key",
        // no 'link' key at all
      })
    }));

    render(<HelpChat />);
    fireEvent.click(screen.getByText('Ask anything').closest('button')!);
    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'missing' } });
    fireEvent.click(input.closest('form')!.querySelector('button[type="submit"]')!);

    await waitFor(() => {
      expect(screen.getByText('missing link key')).toBeInTheDocument();
    });
  });
});


describe('HelpChat empty message cases', () => {
  it('does nothing when sending empty message by pressing enter', () => {
    render(<HelpChat />);
    const openBtn = screen.getByText('Ask anything').closest('button');
    fireEvent.click(openBtn!);

    // empty message -> form submit does nothing
    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: '   ' } }); // Whitespace only
    const submitBtn = screen.getByRole('button', { name: 'Send message' });

    // Let's force a submit using the form itself since the button is disabled
    fireEvent.submit(input.closest('form')!);

    // Still only 1 agent message
    const messages = screen.getAllByText(/Hi! I'm your AI Help Agent/);
    expect(messages.length).toBe(1);
  });
});


describe('HelpChat Failed Response Case', () => {
  it('handles non-ok response from fetch', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: false,
      status: 500,
    }));

    render(<HelpChat />);
    fireEvent.click(screen.getByText('Ask anything').closest('button')!);
    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'failed fetch' } });
    fireEvent.submit(input.closest('form')!);

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });
});
