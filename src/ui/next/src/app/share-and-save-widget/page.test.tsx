import React from 'react';
import { render, screen, act, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ShareAndSaveWidgetPage from './page';
import * as navigation from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({ push: vi.fn() })),
}));

describe('ShareAndSaveWidgetPage', () => {
  const mockPush = vi.fn();

  beforeEach(() => {
    (navigation.useRouter as any).mockReturnValue({ push: mockPush });
    vi.clearAllMocks();

    const localStorageMock = {
      getItem: vi.fn().mockImplementation((key) => {
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

  it('renders the configuration form', async () => {
    await act(async () => {
      render(<ShareAndSaveWidgetPage />);
    });

    expect(screen.getByText('Share & Save Widget')).toBeDefined();
    expect(screen.getByText('Configure Incentive')).toBeDefined();
    expect(screen.getByText('Brand Settings')).toBeDefined();
  });

  it('renders the Powered by OHC watermark when not removed', async () => {
    await act(async () => {
      render(<ShareAndSaveWidgetPage />);
    });

    const watermark = screen.getByText('⚡ Powered by OHC');
    expect(watermark).toBeDefined();
    expect(watermark.closest('a')?.href).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=test-tenant');
  });

  it('shows soft paywall when trying to remove branding without pro', async () => {
    await act(async () => {
      render(<ShareAndSaveWidgetPage />);
    });

    const checkboxLabel = screen.getByText('Remove "Powered by OHC" branding');

    await act(async () => {
      fireEvent.click(checkboxLabel.closest('label')!);
    });

    expect(screen.getByText('Make it Yours')).toBeDefined();

    // Verify upgrade button works
    const upgradeBtns = screen.getAllByText(/Upgrade to Pro/);
    await act(async () => {
      // The button text is exactly "Upgrade to Pro" inside the modal
      fireEvent.click(screen.getByRole('button', { name: /Upgrade to Pro/i }));
    });

    expect(mockPush).toHaveBeenCalledWith('/settings?tab=billing&upgrade=true');
  });
});
