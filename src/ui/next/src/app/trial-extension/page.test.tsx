import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import TrialExtensionPage from './page';

describe('TrialExtensionPage', () => {
  beforeEach(() => {
    vi.spyOn(window, 'open').mockImplementation(() => null);
    global.fetch = vi.fn(() => Promise.resolve({
      ok: true,
      json: () => Promise.resolve({})
    })) as any;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the initial state correctly', () => {
    render(<TrialExtensionPage />);
    expect(screen.getByText('Interactive Trial Extension')).toBeDefined();
    expect(screen.getByText('Want 7 Extra Days of Pro?')).toBeDefined();
    expect(screen.getByText(/Share on X to Unlock 7 Days/i)).toBeDefined();
  });

  it('handles the share and claim flow', async () => {
    render(<TrialExtensionPage />);

    const shareButton = screen.getByRole('button', { name: /Share on X to Unlock 7 Days/i });
    fireEvent.click(shareButton);

    expect(window.open).toHaveBeenCalledWith(
      expect.stringContaining('https://twitter.com/intent/tweet?text='),
      '_blank'
    );

    expect(screen.getByText(/Verifying Share.../i)).toBeDefined();

    await waitFor(() => {
      expect(screen.getByText('Trial Extended!')).toBeDefined();
      expect(screen.getByText(/Your Pro trial has been successfully extended by 7 days/i)).toBeDefined();
      expect(screen.getByText('Return to Dashboard')).toBeDefined();
    }, { timeout: 3000 });
  });
});
