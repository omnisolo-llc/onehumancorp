import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import { describe, it, expect, vi, beforeEach } from 'vitest';

global.fetch = vi.fn() as any;

describe('TooltipRegistry', () => {
  beforeEach(() => {
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({ "test-id": "Fetched tooltip text" })
    });
  });

  it('renders default text on hover', async () => {
    render(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Hover me');

    // Create a mock getBoundingClientRect
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith("/api/tooltips");
    });

    act(() => { fireEvent.mouseEnter(button.parentElement!); });

    await waitFor(() => {
      expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();
    });

    act(() => { fireEvent.mouseLeave(button.parentElement!); });

    await waitFor(() => {
      expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
    });
  });
});
