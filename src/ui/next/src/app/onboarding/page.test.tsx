import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { beforeEach, describe, it, expect, vi, afterEach } from 'vitest';

describe('OnboardingWizard', () => {
  beforeEach(() => {
    localStorage.clear();
    useOnboardingStore.setState({
      step: 1,
      whatDoYouCreate: '',
      instagramHandle: '',
      stripeConnected: false,
      businessDescription: '',
      isLoading: false,
      error: '',
      startResult: null,
    });

    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial screen correctly', async () => {
    act(() => { render(<OnboardingWizard />); });

    expect(screen.getByText("Launch your creator business")).toBeInTheDocument();
    expect(screen.getByText("What do you create?")).toBeInTheDocument();
    expect(screen.getByText("What is your Instagram handle?")).toBeInTheDocument();
    expect(screen.getByText("Connect Stripe for deposits")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Generate Storefront/i });
    expect(button).toBeDisabled();
  });

  it('Handles instant generation successful onboarding flow', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock intake success
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        business_type: 'Bakery',
        business_name: 'Maya Bakery',
        categories: ['food'],
        initial_products: [{ name: 'Cake', price: '20' }]
      })
    });

    // Mock start success
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ message: "Success!" })
    });

    act(() => { render(<OnboardingWizard />); });

    const createInput = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(createInput, 'I bake cakes');

    const instaInput = screen.getByPlaceholderText(/your\.handle/i);
    await userEvent.type(instaInput, 'maya_bakes');

    const stripeToggle = screen.getByText('Connect Stripe for deposits');
    await userEvent.click(stripeToggle);

    const button = screen.getByRole('button', { name: /Generate Storefront/i });
    expect(button).not.toBeDisabled();

    // Trigger submission
    await act(async () => {
      button.click();
    });

    // Verify it transitions instantly to Step 5 (Live Screen) on success
    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("my-business.ohc.store")).toBeInTheDocument();
      expect(screen.getByText(/Marketing Agent is now optimizing/i)).toBeInTheDocument();
    });

    expect(global.fetch).toHaveBeenCalledTimes(2);
    const startCallBody = JSON.parse((global.fetch as any).mock.calls[1][1].body);
    expect(startCallBody.social_links.instagram).toBe('maya_bakes');
    expect(startCallBody.payment_pref).toBe('stripe');
  });

  it('Step 1: Handles intake API failure', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock intake failure
    (global.fetch as any).mockResolvedValueOnce({
      ok: false
    });

    act(() => { render(<OnboardingWizard />); });

    const createInput = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(createInput, 'I am a baker in NY');

    const button = screen.getByRole('button', { name: /Generate Storefront/i });

    await act(async () => {
      button.click();
    });

    // Verify error appears and step goes back to 1
    await waitFor(() => {
      expect(screen.getByText("Failed to process business details")).toBeInTheDocument();
      expect(screen.getByText("Launch your creator business")).toBeInTheDocument();
    });
  });

  it('Step 1: Handles start API failure and returns to Step 1', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock intake success
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        business_type: 'Bakery',
      })
    });

    // Mock start failure
    (global.fetch as any).mockResolvedValueOnce({
      ok: false
    });

    act(() => { render(<OnboardingWizard />); });

    const createInput = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(createInput, 'I am a baker in NY');

    const button = screen.getByRole('button', { name: /Generate Storefront/i });

    await act(async () => {
      button.click();
    });

    // Verify error appears and step goes back to 1
    await waitFor(() => {
      expect(screen.getByText("Failed to start onboarding")).toBeInTheDocument();
      expect(screen.getByText("Launch your creator business")).toBeInTheDocument();
    });
  });

  it('Step 5: Shows Live Screen with correct links and Marketing Agent mention', async () => {
    useOnboardingStore.setState({
      step: 5,
      startResult: { message: "Your business has been successfully launched." }
    });

    act(() => { render(<OnboardingWizard />); });

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("Your business has been successfully launched.")).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Go to Dashboard/i })).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Preview Storefront/i })).toBeInTheDocument();
    });
  });
});
