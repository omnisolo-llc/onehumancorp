import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import TriagePage from './page';
import React from 'react';

// Mock AppShell
vi.mock('../components/AppShell', () => ({
  AppShell: ({ children, title }: any) => (
    <div>
      <h1>{title}</h1>
      {children}
    </div>
  ),
}));

// Mock fetch
const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('TriagePage', () => {
  it('renders triage page and displays items', async () => {
    const mockItems = [
      {
        id: '1',
        tenant_id: 'default',
        source: 'Shopify',
        priority: 'Urgent',
        context: 'Discrepancy in Vintage Denim Jacket',
        action_type: 'INVENTORY_RECONCILE',
        action_payload: JSON.stringify({
          product_name: 'Vintage Denim Jacket',
          sku: 'VDJ-001',
          platform_counts: [
            { platform: 'Square', quantity: 15 },
            { platform: 'Shopify', quantity: 12 }
          ],
          recommended_quantity: 12,
          discrepancy_reason: 'Testing discrepancy'
        })
      }
    ];

    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => mockItems,
    });

    render(<TriagePage />);

    // Check title
    expect(screen.getByText('Work Triage')).toBeDefined();

    // Wait for items to load
    const itemCard = await screen.findByTestId('triage-card-1');
    expect(itemCard).toBeDefined();
    // One in list, one in detail header, one in reconciliation card
    expect(screen.getAllByText('Shopify')).toHaveLength(3);
    // One in list, one in detail context
    expect(screen.getAllByText('Discrepancy in Vintage Denim Jacket')).toHaveLength(2);

    // Check ReconciliationCard content
    expect(screen.getByText('Inventory Sync')).toBeDefined();
    expect(screen.getByText('VDJ-001')).toBeDefined();
    expect(screen.getByText('Vintage Denim Jacket')).toBeDefined();
    expect(screen.getByText('Square')).toBeDefined();
    expect(screen.getByText('15')).toBeDefined();
    expect(screen.getAllByText('12')).toHaveLength(2); // One in Square row, one in Shopify row
  });

  it('renders empty state when no items', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => [],
    });

    render(<TriagePage />);
    expect(await screen.findByText('No items need your attention right now. Great job!')).toBeDefined();
  });

  it('handles approve action', async () => {
    const mockItems = [{ id: '1', source: 'Shopify', action_type: 'INVENTORY_RECONCILE', action_payload: '{}' }];
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => mockItems });
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => ({ status: 'success' }) });

    render(<TriagePage />);
    const approveBtn = await screen.findByTestId('approve-btn');

    // Trigger approval
    await import('@testing-library/react').then(tl => tl.fireEvent.click(approveBtn));

    expect(screen.getByText('Approving...')).toBeDefined();
  });

  it('handles dismiss action', async () => {
    const mockItems = [{ id: '1', source: 'Shopify', action_type: 'INVENTORY_RECONCILE', action_payload: '{}' }];
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => mockItems });
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => ({ status: 'success' }) });

    render(<TriagePage />);
    const dismissBtn = await screen.findByTestId('dismiss-btn');

    // Trigger dismiss
    await import('@testing-library/react').then(tl => tl.fireEvent.click(dismissBtn));

    expect(screen.getByText('Dismissing...')).toBeDefined();
  });

  it('renders standard action payload when not a reconciliation', async () => {
    const mockItems = [{ id: '1', source: 'Support', action_type: 'DRAFT_EMAIL', action_payload: 'Hello world' }];
    mockFetch.mockResolvedValueOnce({ ok: true, json: async () => mockItems });

    render(<TriagePage />);
    expect(await screen.findByText('Hello world')).toBeDefined();
  });
});
