import '@testing-library/jest-dom';

import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import { describe, it, expect, vi } from 'vitest';

global.fetch = vi.fn() as any;

const TestComponent = () => {
  return (
    <WithTooltip id="test-id" defaultText="Default Tooltip">
      <button>Hover Me</button>
    </WithTooltip>
  );
};

describe('TooltipRegistry', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders children without tooltip initially', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({ "test-id": "Fetched Tooltip" })
    }));

    render(
      <TooltipProvider>
        <TestComponent />
      </TooltipProvider>
    );

    expect(screen.getByText('Hover Me')).toBeInTheDocument();
    expect(screen.queryByText('Fetched Tooltip')).not.toBeInTheDocument();
  });

  it('shows default text if fetch fails or id not found', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: false
    }));

    render(
      <TooltipProvider>
        <TestComponent />
      </TooltipProvider>
    );

    const button = screen.getByText('Hover Me');
    fireEvent.mouseEnter(button.closest('div')!);

    await waitFor(() => {
      expect(screen.getByText('Default Tooltip')).toBeInTheDocument();
    });

    fireEvent.mouseLeave(button.closest('div')!);
    await waitFor(() => {
      expect(screen.queryByText('Default Tooltip')).not.toBeInTheDocument();
    });
  });

  it('shows fetched text when available', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({
      ok: true,
      json: async () => ({ "test-id": "Fetched Tooltip Data" })
    }));

    render(
      <TooltipProvider>
        <TestComponent />
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalled();
    });

    const button = screen.getByText('Hover Me');
    fireEvent.mouseEnter(button.closest('div')!);

    await waitFor(() => {
      expect(screen.getByText('Fetched Tooltip Data')).toBeInTheDocument();
    });
  });
});
