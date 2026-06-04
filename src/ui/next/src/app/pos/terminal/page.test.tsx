import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import TerminalPage from './page';
import { vi } from 'vitest';
import { useTranslation, useCurrency } from '../../../lib/localizationStore';
import { useOfflineSyncStore } from '../../../lib/offlineSyncStore';
import { SyncManager } from '../../../lib/SyncManager';

// Mock dependencies
vi.mock('../../../lib/localizationStore', () => ({
  useTranslation: vi.fn(),
  useCurrency: vi.fn(),
}));

vi.mock('../../../components/LocalizationToggle', () => ({
  LocalizationToggle: () => <div data-testid="localization-toggle" />
}));

vi.mock('../../../lib/SyncManager', () => ({
  SyncManager: {
    start: vi.fn(),
    stop: vi.fn(),
  }
}));

// Mock window.alert
const mockAlert = vi.fn();
window.alert = mockAlert;

// Mock OfflineStore internally used in page.tsx
const mockStaff = [{ id: '1', name: 'Test User', role: 'Manager', pin_hash: '1234' }];
let mockEvents: any[] = [];

// Expose these for testing
(global as any).localStorage = {
  getItem: vi.fn((key) => {
    if (key === 'ohc_offline_staff') return JSON.stringify(mockStaff);
    if (key === 'ohc_offline_events') return JSON.stringify(mockEvents);
    return null;
  }),
  setItem: vi.fn((key, value) => {
    if (key === 'ohc_offline_events') mockEvents = JSON.parse(value);
  }),
};

describe('TerminalPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockEvents = [];
    (useTranslation as any).mockReturnValue({ t: (k: string) => k });
    (useCurrency as any).mockReturnValue({
      currency: 'USD',
      convert: vi.fn().mockReturnValue({ amount: 5000, isOffline: false }),
    });
    useOfflineSyncStore.getState().clearQueue();
  });

  it('renders lock screen initially', () => {
    render(<TerminalPage />);
    expect(screen.getByText('Terminal Locked')).toBeInTheDocument();
  });

  it('unlocks with correct PIN', async () => {
    render(<TerminalPage />);

    // Enter 1234
    fireEvent.click(screen.getByText('1'));
    fireEvent.click(screen.getByText('2'));
    fireEvent.click(screen.getByText('3'));
    fireEvent.click(screen.getByText('4'));

    await waitFor(() => {
      expect(screen.getByText('Test User')).toBeInTheDocument();
    });
  });

  it('enqueues a new order offline', async () => {
    const enqueueMutationSpy = vi.spyOn(useOfflineSyncStore.getState(), 'enqueueMutation');

    render(<TerminalPage />);

    // Unlock
    fireEvent.click(screen.getByText('1'));
    fireEvent.click(screen.getByText('2'));
    fireEvent.click(screen.getByText('3'));
    fireEvent.click(screen.getByText('4'));

    await waitFor(() => {
      expect(screen.getByText('New Order')).toBeInTheDocument();
    });

    // Create new order
    fireEvent.click(screen.getByText('New Order'));

    expect(enqueueMutationSpy).toHaveBeenCalledWith(
      '/api/pos/orders',
      'POST',
      'NEW_ORDER',
      expect.any(Object)
    );
    expect(mockAlert).toHaveBeenCalledWith('New Order Total: 50 USD');
  });

  it('starts SyncManager on mount', () => {
    render(<TerminalPage />);
    expect(SyncManager.start).toHaveBeenCalled();
  });
});
