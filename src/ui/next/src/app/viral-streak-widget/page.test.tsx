import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralStreakWidgetPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('ViralStreakWidgetPage', () => {
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
    render(<ViralStreakWidgetPage />);
    expect(screen.getByText('Viral Streak Widget 📅')).toBeDefined();
    expect(screen.getByText('Streak Title')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('updates form inputs and preview', () => {
    render(<ViralStreakWidgetPage />);

    const titleInput = screen.getByDisplayValue('Daily Login Streak');
    fireEvent.change(titleInput, { target: { value: '7-Day Coffee Run' } });

    const goalInput = screen.getByDisplayValue('7');
    fireEvent.change(goalInput, { target: { value: '5' } });

    const rewardInput = screen.getByDisplayValue('Free Coffee');
    fireEvent.change(rewardInput, { target: { value: 'Free Donut' } });

    // Check if preview updates
    const titleDisplays = screen.getAllByText('7-Day Coffee Run');
    expect(titleDisplays.length).toBeGreaterThan(0);

    const goalDisplays = screen.getAllByText(/Hit 5 days to unlock/);
    expect(goalDisplays.length).toBeGreaterThan(0);

    const rewardDisplays = screen.getAllByText(/Free Donut/);
    expect(rewardDisplays.length).toBeGreaterThan(0);
  });

  it('copies embed code to clipboard', async () => {
    render(<ViralStreakWidgetPage />);

    const getCodeBtn = screen.getByRole('button', { name: 'Get Embed Code' });
    fireEvent.click(getCodeBtn);

    const copyBtn = screen.getByRole('button', { name: 'Copy Code' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied!' })).toBeDefined();
  });

  it('shows paywall when removing branding without pro', () => {
    render(<ViralStreakWidgetPage />);

    const checkbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/);
    fireEvent.click(checkbox);

    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });

  it('navigates back to dashboard', () => {
    render(<ViralStreakWidgetPage />);

    const backBtn = screen.getByRole('button', { name: /Back to Dashboard/i });
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });
});
