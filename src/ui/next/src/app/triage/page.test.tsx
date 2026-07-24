import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import TriagePage from './page';

// Mock dependencies
vi.mock('../components/AppShell', () => ({
  AppShell: ({ children, title }: { children: React.ReactNode, title: string }) => (
    <div data-testid="app-shell" title={title}>{children}</div>
  ),
}));

vi.mock('../../lib/sync/SyncManager', () => ({
  SyncManager: {
    syncNow: vi.fn(),
  }
}));

vi.mock('../utils/offlineQueue', () => ({
  getActions: vi.fn().mockResolvedValue([]),
}));

const mockTriageItems = [
  {
    id: 'item1',
    tenant_id: 'default',
    customer_id: 'Maya',
    source: 'Instagram DM',
    priority: 'high',
    context: 'Can I get a vegan cake for Saturday?',
    action_type: 'Draft Reply',
    action_payload: 'Yes, we have vegan options.',
    status: 'pending',
    created_at: new Date().toISOString()
  }
];

describe('TriagePage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn() as any;

    // Mock local storage
    const localStorageMock = {
      getItem: vi.fn(),
      setItem: vi.fn(),
      clear: vi.fn(),
    };
    Object.defineProperty(window, 'localStorage', { value: localStorageMock });
  });

  it('renders triage items successfully', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ items: mockTriageItems })
    });

    render(<TriagePage />);

    // Shows loading state


    // Renders the mocked item
    await waitFor(() => {
      expect(screen.getByText('Maya')).toBeDefined();
    });

    expect(screen.getByText('Can I get a vegan cake for Saturday?')).toBeDefined();
    expect(screen.getByText('high')).toBeDefined();
  });

  it('shows empty state when no items', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ items: [] })
    });

    render(<TriagePage />);

    await waitFor(() => {
      expect(screen.getByText(/Your AI assistant has handled all outstanding items/i)).toBeDefined();
    });
  });

  it('expands item when clicked', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ items: mockTriageItems })
    });

    render(<TriagePage />);

    await waitFor(() => {
      expect(screen.getByText('Maya')).toBeDefined();
    });

    // Expand the item
    fireEvent.click(screen.getByTestId('triage-card-header-item1'));

    // Check if the proposed action is visible
    expect(screen.getByText('Proposed Action: Draft Reply')).toBeDefined();
    expect(screen.getByText('Yes, we have vegan options.')).toBeDefined();
  });

  it('approves an item', async () => {
    (global.fetch as any)
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ items: mockTriageItems })
      })
      .mockResolvedValueOnce({
        ok: true
      });

    render(<TriagePage />);

    await waitFor(() => {
      expect(screen.getByText('Maya')).toBeDefined();
    });

    // Expand the item
    fireEvent.click(screen.getByTestId('triage-card-header-item1'));

    const approveBtn = screen.getByTestId('triage-approve-item1');
    fireEvent.click(approveBtn);

    await waitFor(() => {
      // API call should be made to action endpoint
      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/v1/triage/action'),
        expect.objectContaining({
          method: 'POST',
          body: expect.stringContaining('"approved":true')
        })
      );
      // Item should be removed from the list, showing empty state
      expect(screen.getByText(/Your AI assistant has handled all outstanding items/i)).toBeDefined();
    });
  });

  it('dismisses an item', async () => {
    (global.fetch as any)
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ items: mockTriageItems })
      })
      .mockResolvedValueOnce({
        ok: true
      });

    render(<TriagePage />);

    await waitFor(() => {
      expect(screen.getByText('Maya')).toBeDefined();
    });

    // Expand the item
    fireEvent.click(screen.getByTestId('triage-card-header-item1'));

    const dismissBtn = screen.getByTestId('triage-dismiss-item1');
    fireEvent.click(dismissBtn);

    await waitFor(() => {
      // API call should be made
      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/v1/triage/action'),
        expect.objectContaining({
          method: 'POST',
          body: expect.stringContaining('"approved":false')
        })
      );
    });
  });

  it('can edit and save a draft', async () => {
    (global.fetch as any)
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ items: mockTriageItems })
      })
      .mockResolvedValueOnce({
        ok: true
      });

    render(<TriagePage />);

    await waitFor(() => {
      expect(screen.getByText('Maya')).toBeDefined();
    });

    // Expand the item
    fireEvent.click(screen.getByTestId('triage-card-header-item1'));

    // Click Review Draft
    const reviewBtn = screen.getByTestId('triage-review-btn-item1');
    fireEvent.click(reviewBtn);

    // Edit the text
    const textarea = screen.getByTestId('triage-edit-textarea-item1');
    fireEvent.change(textarea, { target: { value: 'Edited response' } });

    // Save
    const saveBtn = screen.getByTestId('triage-save-btn-item1');
    fireEvent.click(saveBtn);

    await waitFor(() => {
      // API call should be made with the updated payload
      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/v1/triage/action'),
        expect.objectContaining({
          method: 'POST',
          body: expect.stringContaining('"edited_payload":"Edited response"')
        })
      );
    });
  });

  it('shows error state when fetching fails', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false
    });

    render(<TriagePage />);

    await waitFor(() => {
      expect(screen.getByText(/Failed to load triage items from the database/i)).toBeDefined();
    });
  });
});
