import React from 'react';
import { render, screen, act, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import GiveawayGeneratorPage from './page';
import * as navigation from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({ push: vi.fn() })),
}));

describe('GiveawayGeneratorPage', () => {
  beforeEach(() => {
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

  it('renders the giveaway generator UI correctly', async () => {
    await act(async () => {
      render(<GiveawayGeneratorPage />);
    });

    expect(screen.getByText('Viral Giveaway Generator 🎁')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
    expect(screen.getByDisplayValue('Win a Free Custom Cake!')).toBeDefined();
    expect(screen.getByText('⚡ Powered by OHC')).toBeDefined();
  });

  it('updates the live preview when title is changed', async () => {
    await act(async () => {
      render(<GiveawayGeneratorPage />);
    });

    const titleInput = screen.getByDisplayValue('Win a Free Custom Cake!');
    await act(async () => {
      fireEvent.change(titleInput, { target: { value: 'Win a Free Coffee!' } });
    });

    const previewTitles = screen.getAllByText('Win a Free Coffee!');
    expect(previewTitles.length).toBeGreaterThan(0);
  });

  it('shows the soft paywall when trying to remove branding without pro', async () => {
    await act(async () => {
      render(<GiveawayGeneratorPage />);
    });

    const brandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    await act(async () => {
      fireEvent.click(brandingCheckbox);
    });

    expect(screen.getByText('Make it 100% Yours')).toBeDefined();
    expect(screen.getByText('Upgrade to Pro')).toBeDefined();
  });
});
