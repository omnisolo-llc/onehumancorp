import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import { describe, it, expect, vi, beforeEach } from 'vitest';

const mockTooltipFetch = vi.fn().mockImplementation((url) => {
    if (url === '/api/tooltips' || url.toString().includes('/api/tooltips')) {
        return Promise.resolve({ ok: true, json: async () => ({ "test-id": "Fetched tooltip text" }) });
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
});

describe('TooltipRegistry', () => {
  beforeEach(() => {
    mockTooltipFetch.mockClear();
    global.fetch = mockTooltipFetch as any;
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

    // Wait for context to be populated via fetch call in TooltipProvider
    await waitFor(() => {
        expect(global.fetch).toHaveBeenCalled();
    });

    // Create a mock getBoundingClientRect
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 10, left: 10, bottom: 30, right: 110, x: 10, y: 10, toJSON: () => {}
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
