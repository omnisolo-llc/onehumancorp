import '@testing-library/jest-dom';

import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { HelpWidget, WalkthroughProvider, useWalkthrough } from './help';
import { TooltipProvider } from './TooltipRegistry';
import { describe, it, expect, vi } from 'vitest';

global.fetch = vi.fn() as any;

describe('HelpWidget System', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (global.fetch as any).mockImplementation(() => Promise.resolve({
      ok: true,
      json: async () => ({})
    }));

    window.HTMLElement.prototype.scrollIntoView = vi.fn();

    // Add dummy target elements for the walkthrough tests
    const dummyIds = ["bio-input", "generate-btn", "stripe-setup-btn", "help-widget-container", "foo"];
    dummyIds.forEach(id => {
      if (!document.getElementById(id)) {
        const el = document.createElement('div');
        el.id = id;
        el.getBoundingClientRect = vi.fn(() => ({
          top: 0, left: 0, width: 100, height: 100, right: 100, bottom: 100, x: 0, y: 0, toJSON: () => {}
        }));
        document.body.appendChild(el);
      }
    });
  });

  afterEach(() => {
    document.body.innerHTML = '';
  });

  const TestWrapper = ({ children }: { children: React.ReactNode }) => (
    <TooltipProvider>
      <WalkthroughProvider>
        {children}
      </WalkthroughProvider>
    </TooltipProvider>
  );

  it('renders help button initially', async () => {
    render(
      <TestWrapper>
        <HelpWidget />
      </TestWrapper>
    );
    await waitFor(() => {
      expect(screen.getByRole('button', { name: /help/i })).toBeInTheDocument();
    });
  });

  it('opens widget when button is clicked', async () => {
    render(
      <TestWrapper>
        <HelpWidget />
      </TestWrapper>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /help/i }));

    await waitFor(() => {
      expect(screen.getByText('Help Center')).toBeInTheDocument();
    });
  });

  it('switches tabs', async () => {
    render(
      <TestWrapper>
        <HelpWidget />
      </TestWrapper>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /help/i }));

    // Switch to Ask AI
    fireEvent.click(screen.getByText('Ask AI'));
    await waitFor(() => {
      expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument();
    });

    // Switch to Videos
    fireEvent.click(screen.getByText('Videos'));
    await waitFor(() => {
      expect(screen.getByText('Tutorials')).toBeInTheDocument();
    });

    // Switch to What's New
    fireEvent.click(screen.getByText("New"));
    await waitFor(() => {
      expect(screen.getByText("New AI Store Builder")).toBeInTheDocument();
    });
  });

  it('sends chat message and receives reply', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === "/api/chat") {
        return Promise.resolve({
          ok: true,
          json: async () => ({ reply: "Mocked AI Help Reply" })
        });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({})
      });
    });

    render(
      <TestWrapper>
        <HelpWidget />
      </TestWrapper>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /help/i }));
    fireEvent.click(screen.getByText('Ask AI'));

    const input = screen.getByPlaceholderText('Ask anything...');
    fireEvent.change(input, { target: { value: 'How to setup?' } });

    fireEvent.submit(input.closest('form')!);

    expect(screen.getByText('How to setup?')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Mocked AI Help Reply')).toBeInTheDocument();
    });
  });

  it('handles chat fetch error gracefully', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === "/api/chat") {
        return Promise.reject(new Error("Network Error"));
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({})
      });
    });

    render(
      <TestWrapper>
        <HelpWidget />
      </TestWrapper>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /help/i }));
    fireEvent.click(screen.getByText('Ask AI'));

    const input = screen.getByPlaceholderText('Ask anything...');
    fireEvent.change(input, { target: { value: 'Fail test' } });

    fireEvent.submit(input.closest('form')!);

    await waitFor(() => {
      // The error message from help.tsx is "Sorry, I'm having trouble connecting right now."
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('starts a walkthrough', async () => {
    const TestComponent = () => {
      const { startWalkthrough } = useWalkthrough();
      return (
        <button onClick={() => startWalkthrough([{ targetId: "foo", title: "Foo", content: "Bar" }])}>
          Start Walkthrough
        </button>
      );
    };

    render(
      <TestWrapper>
        <TestComponent />
      </TestWrapper>
    );

    await waitFor(() => {
      expect(screen.getByText('Start Walkthrough')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Start Walkthrough'));
  });

  it('plays a video', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === "/api/videos") {
        return Promise.resolve({
          ok: true,
          json: async () => ([
            { id: 1, title: "How to set up your first store easily", duration: "1:20" }
          ])
        });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({})
      });
    });

    render(
      <TestWrapper>
        <HelpWidget />
      </TestWrapper>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /help/i }));
    fireEvent.click(screen.getByText('Videos'));

    await waitFor(() => {
      expect(screen.getByText('How to set up your first store easily')).toBeInTheDocument();
    });

    const video = screen.getByText('How to set up your first store easily');
    fireEvent.click(video);

    expect(screen.getAllByText('How to set up your first store easily').length).toBeGreaterThan(0);

    // Close video
    const closeBtn = screen.getByLabelText('Close video');
    fireEvent.click(closeBtn);
  });
});
