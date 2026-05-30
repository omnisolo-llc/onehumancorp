import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import { describe, it, expect, vi, beforeEach } from 'vitest';

global.fetch = vi.fn().mockImplementation((url) => {
    if (url === '/api/tooltips') {
        return Promise.resolve({ ok: true, json: async () => ({ "test-id": "Fetched tooltip text" }) });
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
}) as any;

describe('TooltipRegistry', () => {

  it('renders fetched text on hover after fetching', async () => {
    await act(async () => {
      render(
        <TooltipProvider>
          <WithTooltip id="test-id" defaultText="Default Tooltip">
            <button>Hover me</button>
          </WithTooltip>
        </TooltipProvider>
      );
    });

    const button = screen.getByText('Hover me');

    // Wait for context to be populated via fetch call in TooltipProvider
    await waitFor(() => {
        expect(global.fetch).toHaveBeenCalled();
    })

    // Create a mock getBoundingClientRect
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    await act(async () => {
      fireEvent.mouseEnter(button.parentElement!);
    });

    await waitFor(() => {
      expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();
    });

    await act(async () => {
      fireEvent.mouseLeave(button.parentElement!);
    });

    await waitFor(() => {
      expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
    });
  });
});
