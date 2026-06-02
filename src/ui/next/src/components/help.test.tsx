import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { HelpWidget, WalkthroughProvider, useWalkthrough } from './help';
import { TooltipProvider } from './TooltipRegistry';
import { useRouter } from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
}));

const mockPush = vi.fn();

describe('HelpWidget', () => {
  beforeEach(() => {
    (useRouter as any).mockReturnValue({ push: mockPush });
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/help') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([{ title: "Test Article", desc: "A test article", link: "/test" }])
        });
      }
      if (url === '/api/videos') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([{ id: 1, title: "Test Video", duration: "1:00" }])
        });
      }
      if (url === '/api/tooltips') {
         return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({"changelog-nav-tooltip": "Test tooltip"})
        });
      }
      if (url === '/api/chat') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({reply: "This is a chat reply", link: {url: "http://example.com", title: "example"}})
        });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders the floating help button', async () => {
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );
    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });
  });

  it('opens the help widget and displays tabs', async () => {
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));

    expect(screen.getByText('Help Center')).toBeInTheDocument();

    // Check for fetched data
    await waitFor(() => {
      expect(screen.getByText('Test Article')).toBeInTheDocument();
    });
  });

  it('switches to videos tab and plays video', async () => {
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));

    fireEvent.click(screen.getByText('Videos'));

    await waitFor(() => {
      expect(screen.getByText('Test Video')).toBeInTheDocument();
    });

    // Play video
    const videoCard = screen.getByText('Test Video').closest('div')?.parentElement;
    if (videoCard) {
      fireEvent.click(videoCard);
    }

    await waitFor(() => {
      expect(screen.getAllByText('Test Video').length).toBeGreaterThan(1);
    });
  });

  it('switches to chat tab', async () => {
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));

    fireEvent.click(screen.getByText('Ask AI'));

    await waitFor(() => {
      expect(screen.getByText("Hi! I'm your AI Support Agent. How can I help you grow your business today?")).toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText('Ask anything...');
    fireEvent.change(input, { target: { value: 'Hello' } });
    fireEvent.click(screen.getByRole('button', { name: /Send message/i }));

    await waitFor(() => {
      expect(screen.getByText("This is a chat reply")).toBeInTheDocument();
    });
  });

  it('switches to whatsnew tab', async () => {
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));

    fireEvent.click(screen.getByText('New'));

    await waitFor(() => {
      expect(screen.getByText("What's New")).toBeInTheDocument();
    });
  });

  it('triggers interactive tours', async () => {
    const { container } = render(
      <TooltipProvider>
        <WalkthroughProvider>
          <div id="bio-input"></div>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));

    fireEvent.click(screen.getByText('Tour: Set up your store'));
  });

  it('triggers kairos tour', async () => {
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));

    fireEvent.click(screen.getByText('Tour: KAIROS AI OS Orchestration'));
    expect(mockPush).toHaveBeenCalledWith('/kairos?walkthrough=true');
  });
});

describe('HelpWidget extended tests', () => {
  beforeEach(() => {
    (useRouter as any).mockReturnValue({ push: mockPush });
  });
  afterEach(() => {
    vi.clearAllMocks();
  });

  it('covers missing targetId for WalkthroughProvider step', async () => {
    // Tests el == null branch
    const TestComponent = () => {
      const { startWalkthrough } = useWalkthrough();
      return (
        <div>
          <button onClick={() => startWalkthrough([{ targetId: "test-missing-id", message: "message" }])}>Start missing</button>
        </div>
      );
    };

    render(
      <WalkthroughProvider>
        <TestComponent />
      </WalkthroughProvider>
    );

    fireEvent.click(screen.getByText('Start missing'));
  });

  it('throws error when useWalkthrough is used outside WalkthroughProvider', () => {
    // Suppress console.error
    const consoleSpy = vi.spyOn(console, 'error');
    consoleSpy.mockImplementation(() => {});

    const TestComponent = () => {
      useWalkthrough();
      return <div>Test</div>;
    };

    expect(() => render(<TestComponent />)).toThrow('useWalkthrough must be used within WalkthroughProvider');

    consoleSpy.mockRestore();
  });

  it('handles empty chat submit', async () => {
    global.fetch = vi.fn().mockImplementation((url) => Promise.resolve({ ok: true, json: () => Promise.resolve([]) }));
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));
    fireEvent.click(screen.getByText('Ask AI'));

    await waitFor(() => {
      expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText('Ask anything...');
    fireEvent.change(input, { target: { value: '   ' } });
    fireEvent.click(screen.getByRole('button', { name: /Send message/i }));

    expect(screen.queryByText('   ')).not.toBeInTheDocument();
  });

  it('normalizes valid chat reply with completely valid link', async () => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/chat') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ reply: "Valid reply", link: { url: "https://example.com", title: "Valid Title" } })
        });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
    });

    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));
    fireEvent.click(screen.getByText('Ask AI'));

    await waitFor(() => {
      expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText('Ask anything...');
    fireEvent.change(input, { target: { value: 'Hello' } });
    fireEvent.click(screen.getByRole('button', { name: /Send message/i }));

    await waitFor(() => {
      expect(screen.getByText("Valid reply")).toBeInTheDocument();
      expect(screen.getByText("Valid Title")).toBeInTheDocument();
    });
  });

  it('normalizes chat reply when data.link is just not an object', async () => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/chat') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ reply: "Valid reply", link: "not-an-object" })
        });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
    });

    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));
    fireEvent.click(screen.getByText('Ask AI'));

    await waitFor(() => {
      expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText('Ask anything...');
    fireEvent.change(input, { target: { value: 'Hello' } });
    fireEvent.click(screen.getByRole('button', { name: /Send message/i }));

    await waitFor(() => {
      expect(screen.getByText("Valid reply")).toBeInTheDocument();
    });
  });

  it('handles empty string chat submit directly', async () => {
    global.fetch = vi.fn().mockImplementation((url) => Promise.resolve({ ok: true, json: () => Promise.resolve([]) }));
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));
    fireEvent.click(screen.getByText('Ask AI'));

    await waitFor(() => {
      expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument();
    });

    const form = screen.getByPlaceholderText('Ask anything...').closest('form');
    if (form) {
        fireEvent.submit(form);
    }
  });

  it('handles empty help tabs rendering and data arrays', async () => {
    global.fetch = vi.fn().mockImplementation((url) => {
      return Promise.resolve({
          ok: true,
          json: () => Promise.resolve("not-an-array")
      });
    });

    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));
    expect(screen.getByText('Help Center')).toBeInTheDocument();
  });

  it('handles error in fetching chat replies', async () => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/chat') {
        return Promise.resolve({ ok: false });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
    });

    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Help/i })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /Help/i }));
    fireEvent.click(screen.getByText('Ask AI'));

    await waitFor(() => {
      expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText('Ask anything...');
    fireEvent.change(input, { target: { value: 'Trigger error' } });
    fireEvent.click(screen.getByRole('button', { name: /Send message/i }));

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('handles WalkthroughProvider steps internally', async () => {
    const TestComponent = () => {
      const { startWalkthrough, nextStep, endWalkthrough } = useWalkthrough();
      return (
        <div>
          <button onClick={() => startWalkthrough([{ targetId: "test-id", message: "message 1" }, { targetId: "test-id", message: "message 2" }])}>Start</button>
          <button onClick={() => nextStep()}>Next</button>
          <button onClick={() => endWalkthrough()}>End</button>
        </div>
      );
    };

    render(
      <WalkthroughProvider>
        <div id="test-id"></div>
        <TestComponent />
      </WalkthroughProvider>
    );

    fireEvent.click(screen.getByText('Start'));
    fireEvent.click(screen.getByText('Next'));
    fireEvent.click(screen.getByText('Next'));
    fireEvent.click(screen.getByText('End'));
  });
});
