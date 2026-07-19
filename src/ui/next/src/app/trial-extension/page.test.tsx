import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import TrialExtensionPage from './page';

describe('TrialExtensionPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();

    // Mock window.open
    window.open = vi.fn();
    window.alert = vi.fn();

    // Mock global fetch
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ message: 'Trial extended successfully' }),
      } as Response)
    );
  });

  it('renders the initial state correctly', () => {
    render(<TrialExtensionPage />);
    expect(screen.getByText('Interactive Trial Extension')).toBeDefined();
    expect(screen.getByText('Want 7 Extra Days of Pro?')).toBeDefined();
    expect(screen.getByRole('button', { name: /Share on X to Unlock 7 Days/i })).toBeDefined();
  });

  it('handles the claim process', async () => {
    render(<TrialExtensionPage />);

    const claimButton = screen.getByRole('button', { name: /Share on X to Unlock 7 Days/i });
    fireEvent.click(claimButton);

    // Verify window.open was called
    expect(window.open).toHaveBeenCalledWith(expect.stringContaining('twitter.com/intent/tweet'), '_blank');

    // Verify loading state
    expect(screen.getByText(/Verifying Share/i)).toBeDefined();

    // Verify API call
    expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/trial-extension/claim', expect.any(Object));

    // Verify success state
    await waitFor(() => {
      expect(screen.getByText('Trial Extended!')).toBeDefined();
    });
  });
});
