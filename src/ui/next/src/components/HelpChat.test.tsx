import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { HelpChat } from './HelpChat';

// Mock matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(), // deprecated
    removeListener: vi.fn(), // deprecated
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

// Mock DOMPurify as it's not needed for logical testing here
vi.mock('dompurify', () => ({
  default: {
    sanitize: (str: string) => str
  }
}));

describe('HelpChat', () => {
  beforeEach(() => {
    global.fetch = vi.fn();
  });

  it('renders the floating button initially', () => {
    render(<HelpChat />);
    expect(screen.getByLabelText('Open help chat')).toBeInTheDocument();
    expect(screen.queryByText('Always here to help')).not.toBeInTheDocument();
  });

  it('opens chat window when the floating button is clicked', () => {
    render(<HelpChat />);
    const openBtn = screen.getByLabelText('Open help chat');
    fireEvent.click(openBtn);
    expect(screen.getByText('Always here to help')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Ask me anything...')).toBeInTheDocument();
  });

  it('closes chat window when close button is clicked', () => {
    render(<HelpChat />);
    const openBtn = screen.getByLabelText('Open help chat');
    fireEvent.click(openBtn);
    const closeBtn = screen.getByLabelText('Close help chat');
    fireEvent.click(closeBtn);
    expect(screen.queryByText('Always here to help')).not.toBeInTheDocument();
  });

  it('sends message and appends reply', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ reply: "Mocked AI reply" })
    });

    render(<HelpChat />);
    const openBtn = screen.getByLabelText('Open help chat');
    fireEvent.click(openBtn);

    const input = screen.getByPlaceholderText('Ask me anything...');
    fireEvent.change(input, { target: { value: 'How do I add a product?' } });

    const form = input.closest('form');
    fireEvent.submit(form!);

    await waitFor(() => {
      expect(screen.getByText('How do I add a product?')).toBeInTheDocument();
      expect(screen.getByText('Mocked AI reply')).toBeInTheDocument();
    });
  });
});
