import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import userEvent from '@testing-library/user-event';

describe('TooltipRegistry', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        'test-tooltip': 'This is a test tooltip loaded from API'
      })
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders children without tooltip initially', async () => {
    render(
      <TooltipProvider>
        <WithTooltip id="test-tooltip">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    expect(screen.getByText('Hover me')).toBeInTheDocument();
    expect(screen.queryByText('This is a test tooltip loaded from API')).not.toBeInTheDocument();
  });

  it('shows tooltip on hover and hides on mouse leave', async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <WithTooltip id="test-tooltip" defaultText="Fallback text">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    // We need to wait for the useEffect fetch to complete
    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    const button = screen.getByText('Hover me').parentElement!;
    await user.hover(button);

    await waitFor(() => {
      expect(screen.getByText('This is a test tooltip loaded from API')).toBeInTheDocument();
    });

    await user.unhover(button);

    await waitFor(() => {
      expect(screen.queryByText('This is a test tooltip loaded from API')).not.toBeInTheDocument();
    });
  });

  it('shows tooltip on focus and hides on blur for accessibility', async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <WithTooltip id="test-tooltip" defaultText="Fallback text">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    const wrapper = screen.getByText('Hover me').parentElement!;
    fireEvent.focus(wrapper);

    await waitFor(() => {
      expect(screen.getByText('This is a test tooltip loaded from API')).toBeInTheDocument();
      expect(wrapper).toHaveAttribute('aria-describedby', 'tooltip-test-tooltip');
    });

    fireEvent.blur(wrapper);

    await waitFor(() => {
      expect(screen.queryByText('This is a test tooltip loaded from API')).not.toBeInTheDocument();
      expect(wrapper).not.toHaveAttribute('aria-describedby');
    });
  });

  it('hides tooltip on Escape key press', async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <WithTooltip id="test-tooltip" defaultText="Fallback text">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    const wrapper = screen.getByText('Hover me').parentElement!;
    fireEvent.focus(wrapper);

    await waitFor(() => {
      expect(screen.getByText('This is a test tooltip loaded from API')).toBeInTheDocument();
    });

    fireEvent.keyDown(wrapper, { key: 'Escape', code: 'Escape' });

    await waitFor(() => {
      expect(screen.queryByText('This is a test tooltip loaded from API')).not.toBeInTheDocument();
    });
  });
});
