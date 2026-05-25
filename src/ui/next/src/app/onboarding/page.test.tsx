import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';

global.fetch = vi.fn();

describe('OnboardingWizard', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    localStorage.clear();
    useOnboardingStore.setState({
      step: 1,
      mode: 'guided',
      bio: '',
      businessType: '',
      businessName: '',
      businessCategory: '',
      firstProductName: '',
      firstProductPrice: '',
      template: 'Modern',
      domain: 'free',
      isLoading: false,
      error: '',
      intakeData: null,
      startResult: null,
    });
  });

  it('loads state from backend on mount if data.step >= step', async () => {
    (global.fetch as any)
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          step: 2,
          businessType: 'Bakery',
          businessName: 'My Bakery'
        })
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({})
      });

    render(<OnboardingWizard />);

    await waitFor(() => {
      expect(useOnboardingStore.getState().step).toBe(2);
      expect(useOnboardingStore.getState().businessType).toBe('Bakery');
      expect(useOnboardingStore.getState().businessName).toBe('My Bakery');
    });
  });

  it('can toggle instant mode and validate bio', async () => {
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({})
    });

    render(<OnboardingWizard />);

    const instantBtn = screen.getByRole('button', { name: /Instant Setup/i });
    instantBtn.click();

    await waitFor(() => {
        expect(useOnboardingStore.getState().mode).toBe('instant');
    });

    const generateBtn = screen.getByRole('button', { name: /Generate Draft/i });
    generateBtn.click();

    await waitFor(() => {
        expect(screen.getByText('Please provide a bit more detail about your business (at least 10 characters).')).toBeInTheDocument();
    });
  });
});
