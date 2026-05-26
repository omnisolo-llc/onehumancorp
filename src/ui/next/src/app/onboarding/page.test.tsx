import { render, screen, waitFor, act, fireEvent } from '@testing-library/react';
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

    (global.fetch as any).mockImplementation((url: string) => {
        if (url.includes('/api/onboarding/state')) {
            return Promise.resolve({ ok: true, json: async () => ({}) });
        }
        return Promise.resolve({ ok: true, json: async () => ({}) });
    });
  });

  it('loads state from backend on mount', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
        if (url.includes('/api/onboarding/state')) {
            return Promise.resolve({
                ok: true,
                json: async () => ({
                    step: 2,
                    businessType: 'Store',
                    businessName: 'My Store'
                })
            });
        }
        return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    await act(async () => {
        render(<OnboardingWizard />);
    });

    await waitFor(() => {
      expect(useOnboardingStore.getState().step).toBe(2);
      expect(useOnboardingStore.getState().businessType).toBe('Store');
    });
  });

  it('Step 1: selects business type via card and continues', async () => {
    await act(async () => { render(<OnboardingWizard />); });
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    await userEvent.click(screen.getByText('Store'));

    await waitFor(() => {
      expect(useOnboardingStore.getState().step).toBe(2);
      expect(useOnboardingStore.getState().businessType).toBe('Store');
    });
  });

  it('Step 1: types business type and continues', async () => {
    await act(async () => { render(<OnboardingWizard />); });
    const input = screen.getByPlaceholderText(/e.g. Handmade Jewellery/i);
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    await userEvent.type(input, 'Bakery');
    await userEvent.click(screen.getByText(/Continue/i));

    await waitFor(() => {
      expect(useOnboardingStore.getState().step).toBe(2);
    });
  });

  it('Step 1: validation errors', async () => {
    await act(async () => { render(<OnboardingWizard />); });
    const continueBtn = screen.getByText(/Continue/i);
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    await userEvent.click(continueBtn);
    expect(await screen.findByText(/Please describe what you sell/i)).toBeInTheDocument();

    const input = screen.getByPlaceholderText(/e.g. Handmade Jewellery/i);
    await userEvent.type(input, 'ab');
    await userEvent.click(continueBtn);
    expect(await screen.findByText(/Please enter at least 3 characters/i)).toBeInTheDocument();
  });

  it('Step 2: navigates back and next', async () => {
    useOnboardingStore.setState({ step: 2, businessType: 'Store', businessName: '' });
    await act(async () => { render(<OnboardingWizard />); });
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    await userEvent.click(screen.getByText('Back'));
    expect(useOnboardingStore.getState().step).toBe(1);

    act(() => { useOnboardingStore.setState({ step: 2 }); });
    await userEvent.type(screen.getByPlaceholderText(/e.g. Luna Crafts/i), 'My Bakery');
    await userEvent.click(screen.getByText('Next'));
    expect(useOnboardingStore.getState().step).toBe(3);
  });

  it('Step 3: handles intake generation', async () => {
    useOnboardingStore.setState({ step: 3, businessType: 'Store', businessName: 'Bakery' });
    (global.fetch as any).mockImplementation((url: string) => {
        if (url.includes('/api/onboarding/intake')) {
            return Promise.resolve({
                ok: true,
                json: async () => ({ initial_products: [{ name: 'Cake', price: '10' }] })
            });
        }
        return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    await act(async () => { render(<OnboardingWizard />); });

    const textarea = screen.getByPlaceholderText(/e.g. I create sustainable/i);
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });
    await userEvent.type(textarea, 'I sell organic cakes and cookies.');

    await userEvent.click(screen.getByText(/Generate/i));

    await waitFor(() => {
      expect(useOnboardingStore.getState().step).toBe(4);
    }, { timeout: 10000 });
  });

  it('Step 4: edits product and selects options', async () => {
    useOnboardingStore.setState({
      step: 4,
      intakeData: { initial_products: [{ name: 'Cake', price: '10' }] },
      firstProductName: 'Original Cake',
      template: 'Modern',
      domain: 'free'
    });
    await act(async () => { render(<OnboardingWizard />); });
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    const nameInput = screen.getByPlaceholderText('Product Name');

    fireEvent.change(nameInput, { target: { value: 'Custom Cake' } });

    await waitFor(() => {
        expect(useOnboardingStore.getState().firstProductName).toBe('Custom Cake');
    });

    await userEvent.click(screen.getByText('Minimal'));
    expect(useOnboardingStore.getState().template).toBe('Minimal');

    await userEvent.click(screen.getByText('Custom Domain'));
    expect(useOnboardingStore.getState().domain).toBe('custom');
  });

  it('Step 4: publishes business', async () => {
    useOnboardingStore.setState({
      step: 4,
      intakeData: { initial_products: [{ name: 'Cake', price: '10' }] },
      template: 'Modern',
      domain: 'free'
    });
    (global.fetch as any).mockImplementation((url: string) => {
        if (url.includes('/api/onboarding/start')) {
            return Promise.resolve({
                ok: true,
                json: async () => ({ message: 'Live!' })
            });
        }
        return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    await act(async () => { render(<OnboardingWizard />); });

    const publishBtn = screen.getByText(/Publish/i);
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });
    await userEvent.click(publishBtn);

    await waitFor(() => {
      expect(screen.getByText(/You're Live!/i)).toBeInTheDocument();
    });
  });

  it('handles API errors', async () => {
    useOnboardingStore.setState({ step: 3, businessType: 'Store', businessName: 'Bakery', businessCategory: 'Something valid' });
    (global.fetch as any).mockImplementation((url: string) => {
        if (url.includes('/api/onboarding/intake')) {
            return Promise.resolve({ ok: false });
        }
        return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    await act(async () => { render(<OnboardingWizard />); });
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });
    await userEvent.click(screen.getByText(/Generate/i));

    await waitFor(() => {
        expect(screen.getByText(/Failed to process intake/i)).toBeInTheDocument();
    }, { timeout: 10000 });
  });
});
