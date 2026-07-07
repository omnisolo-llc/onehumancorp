import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralGoalTrackerPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({
    push: vi.fn(),
  })),
}));

describe('ViralGoalTrackerPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant') return 'test-tenant';
        if (key === 'has_pro') return 'false';
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

  it('renders correctly', () => {
    render(<ViralGoalTrackerPage />);
    expect(screen.getByText('Goal Tracker Builder')).toBeDefined();
  });

  it('updates goal target input', () => {
    render(<ViralGoalTrackerPage />);
    const targetInput = screen.getByDisplayValue('10');
    fireEvent.change(targetInput, { target: { value: '25' } });
    expect(screen.getByDisplayValue('25')).toBeDefined();
    // also expect the preview to be updated
    expect(screen.getByText('25 target')).toBeDefined();
  });

  it('updates reward name input', () => {
    render(<ViralGoalTrackerPage />);
    const rewardInput = screen.getByDisplayValue('Free T-Shirt & 20% Off');
    fireEvent.change(rewardInput, { target: { value: 'Exclusive Sticker' } });
    expect(screen.getByDisplayValue('Exclusive Sticker')).toBeDefined();
    // preview update
    expect(screen.getByText('Unlock: Exclusive Sticker')).toBeDefined();
  });

  it('toggles theme', () => {
    render(<ViralGoalTrackerPage />);
    const themeSelect = screen.getByDisplayValue('Light');
    fireEvent.change(themeSelect, { target: { value: 'dark' } });
    expect(screen.getByDisplayValue('Dark')).toBeDefined();
  });

  it('copies embed code to clipboard', () => {
    render(<ViralGoalTrackerPage />);
    const copyButton = screen.getByRole('button', { name: /Copy Embed Code/i });
    fireEvent.click(copyButton);
    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByText('Copied to Clipboard!')).toBeDefined();
  });

  it('shows paywall when removing branding without pro', () => {
    render(<ViralGoalTrackerPage />);
    const checkbox = screen.getByRole('checkbox', { name: /Remove "Powered by OHC" Badge/i });
    fireEvent.click(checkbox);
    expect(screen.getAllByText('Upgrade to Remove Branding').length).toBeGreaterThan(0);
  });
});
