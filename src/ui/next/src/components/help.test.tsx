import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, act } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { HelpWidget, WalkthroughProvider } from './help';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { TooltipProvider } from './TooltipRegistry';

describe('HelpWidget', () => {
  beforeEach(() => {
    window.HTMLElement.prototype.scrollIntoView = vi.fn();
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/walkthrough/store-setup') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([{ targetId: 'test-target', title: 'Test', content: 'Test Content' }])
        });
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve([])
      });
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the help widget', async () => {
    render(<TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider>);
    expect(screen.getByRole('button', { name: 'Help' })).toBeInTheDocument();
  });

  it('fetches dynamic walkthroughs when clicked', async () => {
    const user = userEvent.setup();
    render(<div><div id="test-target">Mock Target</div><TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider></div>);

    const helpBtn = screen.getByRole('button', { name: 'Help' });
    await act(async () => {
      await user.click(helpBtn);
    });

    const tourBtn = screen.getByText('Tour: Set up your store');
    await act(async () => {
      await user.click(tourBtn);
    });

    expect(global.fetch).toHaveBeenCalledWith('/api/walkthrough/store-setup');
  });
});
