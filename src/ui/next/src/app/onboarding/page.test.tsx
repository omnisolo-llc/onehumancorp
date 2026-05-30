import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { beforeEach, describe, it, expect, vi, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';

describe('OnboardingWizard', () => {
  beforeEach(() => {
    localStorage.clear();
    useOnboardingStore.setState({
      step: 1,
      businessName: '',
      whatYouSell: '',
      style: '',
      firstProductName: '',
      firstProductPrice: '',
      isLoading: false,
      error: '',
      startResult: null,
    });

    global.fetch = vi.fn().mockImplementation((url) => {
        return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    }) as any;
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial screen correctly', async () => {
    render(<OnboardingWizard />);

    expect(screen.getByText("Welcome to OneHumanCorp")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Create My Business/i });
    expect(button).toBeDisabled();
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const user = userEvent.setup({ delay: null });

    // Mock intake success
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/intake') {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            business_name: 'Maya Bakery',
            initial_products: [{ name: 'Cake', price: '20' }]
          })
        });
      }
      if (url === '/api/onboarding/start') {
        return Promise.resolve({
          ok: true,
          json: async () => ({ message: "Success!" })
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

    render(<OnboardingWizard />);

    // Step 1 - Inputs
    const nameInput = screen.getByPlaceholderText(/e.g. Maya's Cakes/i);
    await user.type(nameInput, 'Maya Bakery');

    const sellInput = screen.getByPlaceholderText(/e.g. Custom Cakes/i);
    await user.type(sellInput, 'Cakes');

    const styleInput = screen.getByPlaceholderText(/e.g. Elegant, Playful/i);
    await user.type(styleInput, 'Playful');

    const button = screen.getByRole('button', { name: /Create My Business/i });
    expect(button).not.toBeDisabled();

    // Submit Step 1
    await user.click(button);

    // Verify it transitions to Step 3: First Product (via Magic Loading)
    await waitFor(() => {
      expect(screen.getByText("Let's add your first item.")).toBeInTheDocument();
      expect(screen.getByDisplayValue("Cake")).toBeInTheDocument();
    });

    const launchButton = screen.getByRole('button', { name: /Looks Good! Go Live./i });
    await user.click(launchButton);

    // Verify it transitions to Step 5 (Live Screen) on success
    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("my-business.ohc.store")).toBeInTheDocument();
    });
  });

  it('Step 1: Handles intake API failure', async () => {
    const user = userEvent.setup({ delay: null });

    // Mock intake failure
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/intake' || url === '/api/onboarding/start') {
        return Promise.resolve({ ok: false });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

    render(<OnboardingWizard />);

    // Step 1 - Inputs
    const nameInput = screen.getByPlaceholderText(/e.g. Maya's Cakes/i);
    await user.type(nameInput, 'Maya Bakery');

    const sellInput = screen.getByPlaceholderText(/e.g. Custom Cakes/i);
    await user.type(sellInput, 'Cakes');

    const styleInput = screen.getByPlaceholderText(/e.g. Elegant, Playful/i);
    await user.type(styleInput, 'Playful');

    const button = screen.getByRole('button', { name: /Create My Business/i });

    await user.click(button);

    // Verify error appears and step goes back to 1
    await waitFor(() => {
      expect(screen.getByText("Failed to process business details")).toBeInTheDocument();
      expect(screen.getByText("Welcome to OneHumanCorp")).toBeInTheDocument();
    });
  });

  it('Step 3: Handles start API failure and returns to Step 3', async () => {
    const user = userEvent.setup({ delay: null });

    // Set initial state to Step 3 to test start API directly
    act(() => {
      useOnboardingStore.setState({ step: 3, firstProductName: 'Cake', firstProductPrice: '20' });
    });

    // Mock start failure
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/intake' || url === '/api/onboarding/start') {
        return Promise.resolve({ ok: false });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

    render(<OnboardingWizard />);

    const launchButton = screen.getByRole('button', { name: /Looks Good! Go Live./i });

    await user.click(launchButton);

    // Verify error appears and step goes back to 3
    await waitFor(() => {
      expect(screen.getByText("Failed to start onboarding")).toBeInTheDocument();
      expect(screen.getByText("Let's add your first item.")).toBeInTheDocument();
    });
  });

  it('Step 1: Displays validation error when business name is too short', async () => {
    const user = userEvent.setup({ delay: null });

    render(<OnboardingWizard />);

    const nameInput = screen.getByPlaceholderText(/e.g. Maya's Cakes/i);
    await user.type(nameInput, 'Ma');

    const sellInput = screen.getByPlaceholderText(/e.g. Custom Cakes/i);
    await user.type(sellInput, 'Cakes');

    const styleInput = screen.getByPlaceholderText(/e.g. Elegant, Playful/i);
    await user.type(styleInput, 'Playful');

    const button = screen.getByRole('button', { name: /Create My Business/i });

    await user.click(button);

    expect(await screen.findByText('Business Name must be at least 3 characters.')).toBeInTheDocument();
  });

  it('Step 5: Shows Live Screen with correct links', async () => {
    act(() => {
      useOnboardingStore.setState({
        step: 5,
        startResult: { message: "Your business has been successfully launched." }
      });
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
