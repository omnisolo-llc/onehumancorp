import { render, screen, waitFor, act } from '@testing-library/react';
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

  it('syncs state to backend when it changes', async () => {
    (global.fetch as any)
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({})
      })
      .mockResolvedValueOnce({
        ok: true,
      });

    render(<OnboardingWizard />);

    await waitFor(() => {
      expect((global.fetch as any)).toHaveBeenCalledWith('/api/onboarding/state', expect.anything());
    });

    act(() => {
      useOnboardingStore.getState().setStep(2);
    });

    await waitFor(() => {
      expect((global.fetch as any)).toHaveBeenCalledWith('/api/onboarding/state', expect.objectContaining({
        method: 'POST',
      }));
    }, {
      timeout: 2000
    });
  });

  it('processes intake and goes to review step', async () => {
    (global.fetch as any)
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({})
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ initial_products: [{name: 'Test', price: '10'}] })
      });

    act(() => {
      useOnboardingStore.setState({ step: 3, businessType: 'Bakery', businessName: 'My Bakery', businessCategory: 'Wedding Cakes' });
    });

    render(<OnboardingWizard />);

    screen.getByText('Generate Draft').click();

    await waitFor(() => {
      expect(useOnboardingStore.getState().step).toBe(4);
      expect(useOnboardingStore.getState().intakeData).not.toBeNull();
    });
  });

  it('starts onboarding and goes to final step', async () => {
    (global.fetch as any)
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({})
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ message: 'Success' })
      });

    act(() => {
      useOnboardingStore.setState({
        step: 4,
        businessType: 'Bakery',
        businessName: 'My Bakery',
        businessCategory: 'Wedding Cakes',
        intakeData: { initial_products: [{name: 'Test', price: '10'}] }
      });
    });

    render(<OnboardingWizard />);

    screen.getByText('Publish Now').click();

    await waitFor(() => {
      expect(useOnboardingStore.getState().step).toBe(5);
      expect(useOnboardingStore.getState().startResult).not.toBeNull();
    });
  });
});
