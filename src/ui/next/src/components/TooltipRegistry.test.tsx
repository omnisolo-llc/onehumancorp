import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import { describe, it, expect, vi, beforeEach } from 'vitest';

global.fetch = vi.fn() as any;

describe('TooltipRegistry', () => {
  beforeEach(() => {
    (global.fetch as any).mockImplementation(() => Promise.resolve({
      json: () => Promise.resolve({ "test-id": "Fetched tooltip text" })
    }));
  });

  it('renders default text on hover', async () => {
    render(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = await screen.findByText('Hover me');

    // Create a mock getBoundingClientRect
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    fireEvent.mouseEnter(button.parentElement!);

    await waitFor(() => {
      expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();
    });

    fireEvent.mouseLeave(button.parentElement!);

    await waitFor(() => {
      expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
    });
  });
});
