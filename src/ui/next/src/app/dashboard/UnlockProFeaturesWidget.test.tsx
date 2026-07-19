import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { UnlockProFeaturesWidget } from './UnlockProFeaturesWidget';
import * as React from 'react';

// Mock clipboard
Object.assign(navigator, {
  clipboard: {
    writeText: vi.fn(),
  },
});

describe('UnlockProFeaturesWidget', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn(() => 'test-tenant'),
      },
      writable: true
    });
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        total_invites: 1
      }),
    } as any);
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders progress bar and title correctly', async () => {
    render(<UnlockProFeaturesWidget />);

    expect(await screen.findByText(/Unlock Pro Features/i)).toBeDefined();
    expect(screen.getByText(/1 \/ 3 Invites/i)).toBeDefined();
  });

  it('copies share link to clipboard and updates button state', async () => {
    render(<UnlockProFeaturesWidget />);
    await screen.findByText(/Unlock Pro Features/i);

    const copyButton = screen.getByText(/Copy Invite Link/i).closest('button');
    expect(copyButton).toBeDefined();

    fireEvent.click(copyButton!);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
      expect.stringContaining('test-tenant')
    );

    expect(await screen.findByText(/Copied Link!/i)).toBeDefined();
  });

  it('shows unlocked state when invites target is reached', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        total_invites: 3
      }),
    } as any);
    render(<UnlockProFeaturesWidget />);

    expect(await screen.findByText(/Pro Features Unlocked!/i)).toBeDefined();
    expect(screen.queryByText(/Copy Invite Link/i)).toBeNull();
  });
});
