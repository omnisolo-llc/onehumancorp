import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralChallengeGeneratorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

describe('ViralChallengeGeneratorPage', () => {
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
    render(<ViralChallengeGeneratorPage />);
    expect(screen.getByText('Viral Challenge Generator 🎯')).toBeDefined();
    expect(screen.getByText('Challenge Name')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('updates form inputs and iframe src', () => {
    render(<ViralChallengeGeneratorPage />);

    const nameInput = screen.getByDisplayValue('30-Day Fitness Challenge');
    fireEvent.change(nameInput, { target: { value: '7-Day Code Challenge' } });

    const durationInput = screen.getByDisplayValue('30');
    fireEvent.change(durationInput, { target: { value: '7' } });

    const rewardInput = screen.getByDisplayValue('Free T-Shirt');
    fireEvent.change(rewardInput, { target: { value: 'Discount Code' } });

    // Check if iframe src updates
    const iframe = document.querySelector('iframe');
    expect(iframe?.getAttribute('src')).toContain('title=7-Day%20Code%20Challenge');
    expect(iframe?.getAttribute('src')).toContain('duration=7');
    expect(iframe?.getAttribute('src')).toContain('reward=Discount%20Code');
  });

  it('copies embed code to clipboard', async () => {
    render(<ViralChallengeGeneratorPage />);

    const copyBtn = screen.getByRole('button', { name: 'Copy Embed Code' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied to Clipboard!' })).toBeDefined();
  });

  it('shows paywall when removing branding without pro', () => {
    render(<ViralChallengeGeneratorPage />);

    const checkbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/);
    fireEvent.click(checkbox);

    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });
});
