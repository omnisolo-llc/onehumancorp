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

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({})
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial screen correctly', async () => {
    render(<OnboardingWizard />);
    await waitFor(() => expect(screen.getByText("Tell us about your business")).toBeInTheDocument());
    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).toBeDisabled();
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    global.fetch = vi.fn()
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          business_type: 'Bakery',
          business_name: 'My Business',
          categories: ['food'],
          initial_products: [{ name: 'Cake', price: '20' }]
        })
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ message: "Success!" })
      });

    render(<OnboardingWizard />);

    const input = await waitFor(() => screen.getByPlaceholderText(/e.g. I bake custom vegan cakes/i));
    await userEvent.type(input, 'I am a baker in NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).not.toBeDisabled();

    await act(async () => {
      button.click();
    });

    await waitFor(() => {
      expect(screen.getByText("Review Details")).toBeInTheDocument();
      expect(screen.getByDisplayValue("My Business")).toBeInTheDocument();
    });

    const continueButton = screen.getByRole('button', { name: /Continue/i });
    await act(async () => {
      continueButton.click();
    });

    await waitFor(() => {
      expect(screen.getByText("Style & Team")).toBeInTheDocument();
      expect(screen.getByText("Website Template")).toBeInTheDocument();
    });

    const launchButton = screen.getByRole('button', { name: /Launch Store/i });
    await act(async () => {
      launchButton.click();
    });
  });

  it('Step 1: Handles intake API failure', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    global.fetch = vi.fn().mockResolvedValueOnce({
      ok: false
    });

    render(<OnboardingWizard />);

    const input = await waitFor(() => screen.getByPlaceholderText(/e.g. I bake custom vegan cakes/i));
    await userEvent.type(input, 'I am a baker in NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });

    await act(async () => {
      button.click();
    });
  });

  it('Step 3: Handles start API failure and returns to Step 3', async () => {
    useOnboardingStore.setState({ step: 3 });

    global.fetch = vi.fn().mockResolvedValueOnce({
      ok: false
    });

    render(<OnboardingWizard />);

    const launchButton = await waitFor(() => screen.getByRole('button', { name: /Launch Store/i }));

    await act(async () => {
      launchButton.click();
    });

    await waitFor(() => {
      expect(screen.getByText("Style & Team")).toBeInTheDocument();
    });
  });

  it('Step 5: Shows Live Screen with correct links', async () => {
    useOnboardingStore.setState({
      step: 5,
      startResult: { message: "Your business has been successfully launched." }
    });

    render(<OnboardingWizard />);

    await waitFor(() => {
      expect(screen.getByText("Your business has been successfully launched.")).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Go to Dashboard/i })).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Preview Storefront/i })).toBeInTheDocument();
    });
  });
});
