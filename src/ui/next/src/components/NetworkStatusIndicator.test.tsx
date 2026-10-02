import React from 'react';
import { act, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { NetworkStatusIndicator } from './NetworkStatusIndicator';
const readSummary = vi.hoisted(() => vi.fn());
vi.mock('../lib/sync/SyncManager', () => ({ SyncManager: { getInstance: () => ({ getQueueSummary: readSummary }) } }));
vi.mock('./TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: React.ReactNode }) => <div>{children}</div> }));
beforeEach(() => {
  readSummary.mockResolvedValue({ pending: 0, needsAttention: 0, reconciliation: 0, legacyHeld: 0, storageUnavailable: false });
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true);
});
afterEach(() => vi.restoreAllMocks());
describe('NetworkStatusIndicator', () => {
  it('shows offline without claiming unverified local persistence', async () => {
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    await act(async () => { render(<NetworkStatusIndicator />); });
    expect(screen.getByText('Offline')).toBeInTheDocument();
    expect(screen.queryByText(/Changes saved locally/)).not.toBeInTheDocument();
  });
  it('separates pending, blocked, reconciliation and unowned counts without payload details', async () => {
    readSummary.mockResolvedValue({ pending: 2, needsAttention: 3, reconciliation: 1, legacyHeld: 4, storageUnavailable: false });
    await act(async () => { render(<NetworkStatusIndicator />); });
    expect(screen.getByText(/Pending: 2/)).toHaveTextContent('Needs attention: 3');
    expect(screen.getByText(/Reconciliation: 1/)).toHaveTextContent('Unassigned: 4');
    expect(screen.queryByRole('button', { name: /retry/i })).not.toBeInTheDocument();
  });
  it('shows unavailable storage rather than an empty or completed queue', async () => {
    readSummary.mockRejectedValue(new Error('Unavailable'));
    await act(async () => { render(<NetworkStatusIndicator />); });
    expect(screen.getByText(/Queue status unavailable/)).toBeInTheDocument();
  });
});
