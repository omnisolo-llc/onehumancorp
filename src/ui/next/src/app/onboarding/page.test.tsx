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
    await act(async () => { render(<OnboardingWizard />); });

    await waitFor(() => {
      expect(screen.getByText("Tell us about your business")).toBeInTheDocument();
    });
    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).toBeDisabled();
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock initial state load
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({})
    });

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

    await act(async () => { render(<OnboardingWizard />); });

    await waitFor(() => {
        expect(screen.getByPlaceholderText(/I bake custom vegan cakes/i)).toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(input, 'I am a baker in NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).not.toBeDisabled();

    // Step 1: Intake
    await act(async () => {
      button.click();
    });

    // Verify it transitions to Step 2: Review Details
    await waitFor(() => {
      expect(screen.getByText("Review Details")).toBeInTheDocument();
      expect(screen.getByDisplayValue("Maya Bakery")).toBeInTheDocument();
    });

    const continueButton = screen.getByRole('button', { name: /Continue/i });
    await act(async () => {
      continueButton.click();
    });

    // Verify it transitions to Step 3: Style & Team
    await waitFor(() => {
      expect(screen.getByText("Style & Team")).toBeInTheDocument();
      expect(screen.getByText("Website Template")).toBeInTheDocument();
    });

    const launchButton = screen.getByRole('button', { name: /Launch Store/i });
    await act(async () => {
      launchButton.click();
    });

    // Verify it transitions to Step 5 (Live Screen) on success
    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("my-business.ohc.store")).toBeInTheDocument();
    });
  });

  it('Step 1: Handles intake API failure', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock initial fetch for load state
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({})
    });

    // Mock intake failure
    (global.fetch as any).mockResolvedValueOnce({
      ok: false
    });

    await act(async () => { render(<OnboardingWizard />); });

    await waitFor(() => {
        expect(screen.getByPlaceholderText(/I bake custom vegan cakes/i)).toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(input, 'I am a baker in NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });

    await act(async () => {
      button.click();
    });

    // Verify error appears and step goes back to 1
    await waitFor(() => {
      expect(screen.getByText("Failed to process business details")).toBeInTheDocument();
      expect(screen.getByText("Tell us about your business")).toBeInTheDocument();
    });
  });

  it('Step 3: Handles start API failure and returns to Step 3', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Set initial state to Step 3 to test start API directly
    useOnboardingStore.setState({ step: 3 });

    // Mock initial fetch for load state
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({})
    });

    // Mock start failure
    (global.fetch as any).mockResolvedValueOnce({
      ok: false
    });

    await act(async () => { render(<OnboardingWizard />); });

    await waitFor(() => {
        expect(screen.getByRole('button', { name: /Launch Store/i })).toBeInTheDocument();
    });

    const launchButton = screen.getByRole('button', { name: /Launch Store/i });

    await act(async () => {
      launchButton.click();
    });

    // Verify error appears and step goes back to 3
    await waitFor(() => {
      expect(screen.getByText("Failed to start onboarding")).toBeInTheDocument();
      expect(screen.getByText("Style & Team")).toBeInTheDocument();
    });
  });

  it('Step 5: Shows Live Screen with correct links', async () => {
    useOnboardingStore.setState({
      step: 5,
      startResult: { message: "Your business has been successfully launched." }
    });

    await act(async () => { render(<OnboardingWizard />); });

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("Your business has been successfully launched.")).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Go to Dashboard/i })).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Preview Storefront/i })).toBeInTheDocument();
    });
  });
});
