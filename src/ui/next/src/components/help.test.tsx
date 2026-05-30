import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { HelpWidget, WalkthroughProvider } from './help';
import { TooltipProvider } from './TooltipRegistry';
import { describe, it, expect, vi } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
  useSearchParams: () => new URLSearchParams(),
}));

describe('HelpWidget', () => {
  it('renders closed state initially', () => {
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );
    expect(screen.getByLabelText('Help')).toBeInTheDocument();
  });

  it('opens help center when clicked and shows tabs', async () => {
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    const openBtn = screen.getByLabelText('Help');
    fireEvent.click(openBtn);

    expect(screen.getByText('Help Center')).toBeInTheDocument();
    expect(screen.getByText('Interactive Tours')).toBeInTheDocument();
  });

  it('switches between tabs', async () => {
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    fireEvent.click(screen.getByLabelText('Help'));

    const videosTab = screen.getByText('Videos');
    fireEvent.click(videosTab);
    expect(screen.getByText('Tutorials')).toBeInTheDocument();

    const newTab = screen.getByText('New');
    fireEvent.click(newTab);
    expect(screen.getByText("What's New")).toBeInTheDocument();
  });

  it('sends a chat message', async () => {
    global.fetch = vi.fn().mockImplementation(() => Promise.resolve({
      ok: true,
      json: () => Promise.resolve({ reply: "I can help with that." })
    }));

    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    fireEvent.click(screen.getByLabelText('Help'));
    fireEvent.click(screen.getByText('Ask AI'));

    const input = screen.getByPlaceholderText('Ask anything...');
    fireEvent.change(input, { target: { value: 'Hello' } });

    const sendBtn = screen.getByRole('button', { name: 'Send message' });
    fireEvent.click(sendBtn);

    expect(screen.getByText('Hello')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('I can help with that.')).toBeInTheDocument();
    });
  });
});
