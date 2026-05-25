import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';

global.fetch = vi.fn();

describe('OnboardingWizard', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({})
    });
    localStorage.clear();
    useOnboardingStore.setState({
      step: 1,
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

  it('renders step 1 by default', () => {
    render(<OnboardingWizard />);
    expect(screen.getByText('What do you do?')).toBeDefined();
  });

  it('renders AI Team selection at step 5', async () => {
    useOnboardingStore.setState({ step: 5 });
    render(<OnboardingWizard />);
    expect(screen.getByText('Your AI Team')).toBeDefined();
    expect(screen.getByText('The Manager')).toBeDefined();
  });

  it('renders launch screen at step 6', async () => {
    useOnboardingStore.setState({ step: 6, businessName: 'Test Biz' });
    render(<OnboardingWizard />);
    expect(screen.getByText('Ready to Launch?')).toBeDefined();
    expect(screen.getByText('Test Biz')).toBeDefined();
  });
});
