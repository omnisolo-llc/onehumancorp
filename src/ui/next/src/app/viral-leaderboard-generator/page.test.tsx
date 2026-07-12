import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralLeaderboardGeneratorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

describe('ViralLeaderboardGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant_id' || key === 'tenant') return 'test-tenant';
        return null;
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly', () => {
    render(<ViralLeaderboardGeneratorPage />);
    expect(screen.getByText('Viral Leaderboard Generator 🏆')).toBeDefined();
    expect(screen.getByText('Leaderboard Title')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('updates form inputs and iframe src', () => {
    render(<ViralLeaderboardGeneratorPage />);

    // There's an input and an option with this value. Use getByRole or getAllByDisplayValue
    const titleInput = screen.getAllByDisplayValue('Top Referrers').find(el => el.tagName.toLowerCase() === 'input') as HTMLElement;
    fireEvent.change(titleInput, { target: { value: 'Top Affiliates' } });

    // The select box doesn't have "Top Referrers" as its display value in the DOM easily, finding it by label is better
    // Or we can just find it by role
    const metricSelect = screen.getAllByDisplayValue('Top Referrers').find(el => el.tagName.toLowerCase() === 'select') as HTMLElement || screen.getByRole('combobox', { name: /metric/i });
    if(metricSelect) {
        fireEvent.change(metricSelect, { target: { value: 'buyers' } });
    }

    // Check if iframe src updates
    const iframe = document.querySelector('iframe');
    expect(iframe?.getAttribute('src')).toContain('title=Top%20Affiliates');
    if(metricSelect) {
       expect(iframe?.getAttribute('src')).toContain('metric=buyers');
    }
  });

  it('copies embed code to clipboard', async () => {
    render(<ViralLeaderboardGeneratorPage />);

    const copyBtn = screen.getByRole('button', { name: 'Copy Embed Code' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied to Clipboard!' })).toBeDefined();
  });

  it('shows paywall when removing branding without pro', () => {
    render(<ViralLeaderboardGeneratorPage />);

    const checkbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/);
    fireEvent.click(checkbox);

    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });
});
