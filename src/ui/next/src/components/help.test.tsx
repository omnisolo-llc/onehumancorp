import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';
import { HelpWidget, WalkthroughProvider, useWalkthrough } from './help';
import { TooltipProvider } from './TooltipRegistry';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
    replace: vi.fn(),
    prefetch: vi.fn(),
    back: vi.fn(),
    forward: vi.fn(),
    refresh: vi.fn(),
  }),
  usePathname: () => '/',
  useSearchParams: () => new URLSearchParams(),
}));

function TestContext() {
  useWalkthrough();
  return null;
}

describe('HelpWidget', () => {
  beforeEach(() => {
    mockPush.mockClear();
    global.fetch = vi.fn().mockImplementation((url: string, options: any) => {
      if (url === '/api/videos') {
        return Promise.resolve(new Response(JSON.stringify([
          { id: 1, title: 'Test Video 1', duration: '1:23' },
          { id: 2, title: 'Test Video 2', duration: '2:34' },
        ]), { status: 200 }));
      }
      if (url === '/api/chat' && options?.method === 'POST') {
        return Promise.resolve(new Response(JSON.stringify({ response: 'This is a mocked bot response' }), { status: 200 }));
      }
      return Promise.resolve(new Response(JSON.stringify({}), { status: 200 }));
    });

    vi.spyOn(document, 'getElementById').mockImplementation((id) => {
      if (['bio-input', 'generate-btn', 'stripe-setup-btn', 'help-widget-container', 'kairos-walkthrough-btn'].includes(id)) {
        const div = document.createElement('div');
        div.id = id;
        div.scrollIntoView = vi.fn();
        div.getBoundingClientRect = vi.fn().mockReturnValue({
          top: 100,
          left: 100,
          bottom: 120,
          right: 120,
          width: 20,
          height: 20
        });
        return div;
      }
      return null;
    });

    process.env.NEXT_PUBLIC_E2E = 'false';
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('throws error when useWalkthrough is used outside provider', () => {
     // disable console error for this test since we expect an error
     const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
     expect(() => render(<TestContext />)).toThrow('useWalkthrough must be used within WalkthroughProvider');
     consoleErrorSpy.mockRestore();
  });

  it('renders widget button and opens chat interface', async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    const openButton = screen.getByLabelText('Help');
    expect(openButton).toBeInTheDocument();

    await user.click(openButton);

    expect(screen.getByText('Help Center')).toBeInTheDocument();
  });

  it('navigates to videos tab and selects a video to play', async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await user.click(screen.getByLabelText('Help'));

    const videosTab = screen.getByRole('button', { name: /Videos/i });
    await user.click(videosTab);

    await waitFor(() => {
      expect(screen.getByText('Test Video 1')).toBeInTheDocument();
    });

    await user.click(screen.getByText('Test Video 1'));

    expect(screen.getByLabelText('Close video')).toBeInTheDocument();

    await user.click(screen.getByLabelText('Close video'));
    expect(screen.queryByLabelText('Close video')).not.toBeInTheDocument();
  });

  it('interacts with chat tab', async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await user.click(screen.getByLabelText('Help'));

    const chatTab = screen.getByRole('button', { name: /Ask AI/i });
    await user.click(chatTab);

    const input = screen.getByPlaceholderText('Ask anything...');
    await user.type(input, 'Hello agent');

    const sendBtn = screen.getByLabelText('Send message');
    await user.click(sendBtn);

    await waitFor(() => {
      expect(screen.getByText('Hello agent')).toBeInTheDocument();
    });
  });

  it('starts tours', async () => {
    const user = userEvent.setup();
    const { unmount } = render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    // Tour 1
    await user.click(screen.getByLabelText('Help'));
    await waitFor(() => expect(screen.getByText('Help Center')).toBeInTheDocument());
    await user.click(screen.getByText('Tour: Set up your store'));
    expect(await screen.findByText('Enter your business description.')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /Next/i }));
    expect(await screen.findByText('Click to generate!')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /Finish/i }));

    // Remount to test KAIROS Tour
    unmount();
    const { unmount: unmount2 } = render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    // Test accept payment tour
    await user.click(screen.getByLabelText('Help'));
    await waitFor(() => expect(screen.getByText('Help Center')).toBeInTheDocument());
    await user.click(screen.getByText('Tour: Accept your first payment'));
    expect(await screen.findByText('Click here to connect Stripe and start accepting payments.')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /Finish/i }));
    unmount2();

    const { unmount: unmount3 } = render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );
    // Test KAIROS tour
    await user.click(screen.getByLabelText('Help'));
    await waitFor(() => expect(screen.getByText('Help Center')).toBeInTheDocument());
    await user.click(screen.getByText('Tour: KAIROS AI OS Orchestration'));
    expect(mockPush).toHaveBeenCalledWith('/kairos?walkthrough=true');
    unmount3();

    const { unmount: unmount4 } = render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );
    // Test activate AI tour
    await user.click(screen.getByLabelText('Help'));
    await waitFor(() => expect(screen.getByText('Help Center')).toBeInTheDocument());
    await user.click(screen.getByText('Tour: Activate your AI Support Agent'));
    expect(await screen.findByText('Activate your AI agent.')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /Finish/i }));
    unmount4();

    const { unmount: unmount5 } = render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );
    // Test Virtual Meeting Room tour
    await user.click(screen.getByLabelText('Help'));
    await waitFor(() => expect(screen.getByText('Help Center')).toBeInTheDocument());
    await user.click(screen.getByText('Tour: Virtual Meeting Room & UltraPlan'));
    expect(await screen.findByText('Agents join the Virtual Meeting Room to debate and plan before executing tasks.')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /Next/i }));
    expect(await screen.findByText('Phase 1: Brainstorming. Phase 2: Refinement. Phase 3: Consensus (UltraPlan protocol).')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /Finish/i }));
    unmount5();
  });

  it('checks whatsnew tab and navigation', async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    await user.click(screen.getByLabelText('Help'));

    const whatsNewTab = screen.getByRole('button', { name: /New/i });
    await user.click(whatsNewTab);

    expect(screen.getByText('New AI Store Builder')).toBeInTheDocument();

    // Close widget
    await user.click(screen.getByLabelText('Help'));
    expect(screen.queryByText("What's New")).not.toBeInTheDocument();
  });
});
