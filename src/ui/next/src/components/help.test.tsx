import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { HelpWidget, WalkthroughProvider, useWalkthrough } from './help';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { TooltipProvider } from './TooltipRegistry';

// Mock useRouter
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

global.fetch = vi.fn() as any;

describe('HelpWidget Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();

    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/help') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([{ title: 'Getting Started', desc: 'Learn how', link: '/help/getting-started' }])
        });
      }
      if (url === '/api/videos') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([{ id: 1, title: 'Video 1', duration: '1:00' }])
        });
      }
      if (url === '/api/tooltips') {
         return Promise.resolve({
           ok: true,
           json: () => Promise.resolve({})
         });
      }
      return Promise.reject(new Error('not found'));
    });
  });

  const TestWrapper = ({ children }: { children: React.ReactNode }) => (
    <TooltipProvider>
      <WalkthroughProvider>
        {children}
      </WalkthroughProvider>
    </TooltipProvider>
  );

  it('renders help button and opens widget', async () => {
    render(<TestWrapper><HelpWidget /></TestWrapper>);
    const btn = screen.getByRole('button', { name: 'Help' });
    fireEvent.click(btn);

    await waitFor(() => {
      expect(screen.getByText('Help Center')).toBeInTheDocument();
    });
  });

  it('navigates between tabs', async () => {
    render(<TestWrapper><HelpWidget /></TestWrapper>);
    fireEvent.click(screen.getByRole('button', { name: 'Help' }));

    await waitFor(() => expect(screen.getByText('Help')).toBeInTheDocument());

    // Chat tab
    fireEvent.click(screen.getByText('Ask AI'));
    await waitFor(() => expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument());

    // Videos tab
    fireEvent.click(screen.getByText('Videos'));
    await waitFor(() => expect(screen.getByText('Tutorials')).toBeInTheDocument());
    await waitFor(() => expect(screen.getByText('Video 1')).toBeInTheDocument());

    // Whats New tab
    fireEvent.click(screen.getByText('New'));
    await waitFor(() => expect(screen.getByText("What's New")).toBeInTheDocument());
  });

  it('handles chat submission in Ask AI tab', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/chat') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ reply: 'I am the AI', link: { url: '/help', title: 'Read more' } })
        });
      }
      // other mocks
      return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
    });

    render(<TestWrapper><HelpWidget /></TestWrapper>);
    fireEvent.click(screen.getByRole('button', { name: 'Help' }));
    fireEvent.click(screen.getByText('Ask AI'));

    await waitFor(() => expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument());
    const input = screen.getByPlaceholderText('Ask anything...');
    fireEvent.change(input, { target: { value: 'Test message' } });

    const submitBtn = screen.getByRole('button', { name: 'Send message' });
    fireEvent.click(submitBtn);

    expect(screen.getByText('Test message')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('I am the AI')).toBeInTheDocument();
      expect(screen.getByText('Read more')).toBeInTheDocument();
    });
  });

  it('handles chat submission error', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/chat') return Promise.reject(new Error('fail'));
      return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
    });

    render(<TestWrapper><HelpWidget /></TestWrapper>);
    fireEvent.click(screen.getByRole('button', { name: 'Help' }));
    fireEvent.click(screen.getByText('Ask AI'));

    await waitFor(() => expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument());
    const input = screen.getByPlaceholderText('Ask anything...');
    fireEvent.change(input, { target: { value: 'Test message fail' } });

    fireEvent.click(screen.getByRole('button', { name: 'Send message' }));

    await waitFor(() => {
      expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

  it('can open and close a video', async () => {
    render(<TestWrapper><HelpWidget /></TestWrapper>);
    fireEvent.click(screen.getByRole('button', { name: 'Help' }));
    fireEvent.click(screen.getByText('Videos'));

    await waitFor(() => expect(screen.getByText('Video 1')).toBeInTheDocument());
    fireEvent.click(screen.getByText('Video 1'));

    await waitFor(() => {
        // video modal close button
        const closeBtns = screen.getAllByRole('button');
        fireEvent.click(closeBtns[closeBtns.length - 1]); // click a button in the modal to close
    });
  });
});

describe('WalkthroughProvider', () => {
    const TestComponent = () => {
        const { startWalkthrough, nextStep, endWalkthrough } = useWalkthrough();
        return (
            <div>
                <button onClick={() => startWalkthrough([{ targetId: 't1', message: 'step1' }, { targetId: 't2', message: 'step2' }])}>Start</button>
                <button onClick={nextStep}>Next</button>
                <button onClick={endWalkthrough}>End</button>
                <div id="t1">Target 1</div>
                <div id="t2">Target 2</div>
            </div>
        );
    };

    it('manages walkthrough steps', async () => {
        window.HTMLElement.prototype.scrollIntoView = vi.fn();
        render(
            <WalkthroughProvider>
                <TestComponent />
            </WalkthroughProvider>
        );

        fireEvent.click(screen.getByText('Start'));
        await waitFor(() => {
            expect(screen.getByText('step1')).toBeInTheDocument();
        });

        const nextBtns = screen.getAllByText('Next');
        fireEvent.click(nextBtns[nextBtns.length - 1]); // click the Walkthrough button, not the test component button
        await waitFor(() => {
            expect(screen.getByText('step2')).toBeInTheDocument();
        });

        fireEvent.click(screen.getByText('End'));
        await waitFor(() => {
            expect(screen.queryByText('step1')).not.toBeInTheDocument();
            expect(screen.queryByText('step2')).not.toBeInTheDocument();
        });
    });
});
