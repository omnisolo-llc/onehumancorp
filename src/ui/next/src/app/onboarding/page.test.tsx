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
      businessDescription: '',
      isLoading: false,
      error: '',
      startResult: null,
    });

        global.fetch = vi.fn().mockImplementation((url) => {
      if (url && url.includes && url.includes('/api/onboarding/state')) {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial screen correctly', async () => {
    render(<OnboardingWizard />);

    expect(await screen.findByText("Tell us about your business")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).toBeDisabled();
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    let fetchCount = 0;
    global.fetch = vi.fn().mockImplementation((url) => {
      fetchCount++;
      if (url && url.includes && url.includes('/api/onboarding/state')) {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      if (fetchCount === 2) {
        return Promise.resolve({ ok: true, json: async () => ({ business_type: 'Bakery', business_name: 'Maya Bakery', categories: ['food'], initial_products: [{ name: 'Cake', price: '20' }] }) });
      }
      if (fetchCount === 3) {
        return Promise.resolve({ ok: true, json: async () => ({ message: "Success!" }) });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    render(<OnboardingWizard />);

    const input = await screen.findByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(input, 'I am a baker in NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).not.toBeDisabled();

    // Step 1: Intake
    await userEvent.click(button);

    // Verify it transitions to Step 2: Review Details
    await waitFor(() => {
      expect(screen.getByText(/Review Details/i)).toBeInTheDocument();
      expect(screen.getByDisplayValue(/Maya Bakery/i)).toBeInTheDocument();
    });

    const continueButton = screen.getByRole('button', { name: /Continue/i });
    await userEvent.click(continueButton);

    // Verify it transitions to Step 3: Style & Team
    await waitFor(() => {

      expect(screen.getByText("Website Template")).toBeInTheDocument();
    });

    const launchButton = screen.getByRole('button', { name: /Launch Store/i });
    await userEvent.click(launchButton);

    // Verify it transitions to Step 5 (Live Screen) on success
    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("my-business.ohc.store")).toBeInTheDocument();
    });
  });

  it('Step 1: Handles intake API failure', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    let failFetchCount = 0;
    global.fetch = vi.fn().mockImplementation((url) => {
      failFetchCount++;
      if (url && url.includes && url.includes('/api/onboarding/state')) {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      return Promise.resolve({ ok: false });
    });

    render(<OnboardingWizard />);

    const input = await screen.findByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(input, 'I am a baker in NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });

    await userEvent.click(button);

    // Verify error appears and step goes back to 1
    await waitFor(() => {
      expect(screen.getByText(/Failed to process business details/i)).toBeInTheDocument();
      expect(screen.getByText(/Tell us about your business/i)).toBeInTheDocument();

    });
  });

  it('Step 3: Handles start API failure and returns to Step 3', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Set initial state to Step 3 to test start API directly
    useOnboardingStore.setState({ step: 3 });

    let failFetchCount = 0;
    global.fetch = vi.fn().mockImplementation((url) => {
      failFetchCount++;
      if (url && url.includes && url.includes('/api/onboarding/state')) {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      return Promise.resolve({ ok: false });
    });

    render(<OnboardingWizard />);

    const launchButton = await screen.findByRole('button', { name: /Launch Store/i });

    await userEvent.click(launchButton);

    // Verify error appears and step goes back to 3
    await waitFor(() => {
      expect(screen.getByText(/Failed to start onboarding/i)).toBeInTheDocument();
      expect(screen.getByText(/Style & Team/i)).toBeInTheDocument();

    });
  });

  it('Step 5: Shows Live Screen with correct links', async () => {
    useOnboardingStore.setState({
      step: 5,
      startResult: { message: "Your business has been successfully launched." }
    });

    render(<OnboardingWizard />);

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("Your business has been successfully launched.")).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Go to Dashboard/i })).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Preview Storefront/i })).toBeInTheDocument();
    });
  });
});
