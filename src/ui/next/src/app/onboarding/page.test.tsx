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
      businessName: '',
      businessType: 'Online Store',
      categories: [],
      businessDescription: '',
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: '',
      startResult: null,
    });

    global.fetch = vi.fn(() => Promise.resolve({
      ok: true,
      json: () => Promise.resolve({})
    })) as any;
  });

  afterEach(() => {
    vi.clearAllMocks();
    vi.useRealTimers();
  });

  it('Step 1: Renders initial screen correctly', async () => {
    act(() => { render(<OnboardingWizard />); });

    expect(screen.getByText("Tell us about your business")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).toBeDisabled();
  });

  it('Handles single-step successful onboarding flow', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock state fetch success
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

    // Mock state update success
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({})
    });

    // Mock start success
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ message: "Success!" })
    });

    act(() => { render(<OnboardingWizard />); });

    // Step 1: Description
    const descInput = screen.getByPlaceholderText(/I am Maya. I bake vegan cakes in Austin/i);
    await userEvent.type(descInput, 'I bake custom vegan cakes for weddings and parties in Austin');

    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).not.toBeDisabled();

    // Step 1: Intake
    await act(async () => {
      button.click();
    });

    // Verify it transitions to Step 2: Review Details
    await waitFor(() => {
      expect(screen.getByText("Review Details")).toBeInTheDocument();
      expect(screen.getByText("Maya Bakery")).toBeInTheDocument();
    });

    const launchButton = screen.getByRole('button', { name: /Approve & Launch/i });
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

    // Mock state fetch success
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({})
    });

    // Mock intake failure
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      json: async () => ({})
    });

    act(() => { render(<OnboardingWizard />); });

    const descInput = screen.getByPlaceholderText(/I am Maya. I bake vegan cakes in Austin/i);
    await userEvent.type(descInput, 'I bake custom vegan cakes');

    const button = screen.getByRole('button', { name: /Generate My Business/i });

    await act(async () => {
      button.click();
    });

    // Verify error appears and step stays on 1
    await waitFor(() => {
      expect(screen.getByText("Failed to process business details")).toBeInTheDocument();
      expect(screen.getByText("Tell us about your business")).toBeInTheDocument();
    });
  });

  it('Step 2: Handles start API failure and returns to Step 2', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Set initial state to Step 2 to test start API directly
    useOnboardingStore.setState({
      step: 2,
      businessName: 'Maya Bakery',
      businessType: 'Bakery',
      categories: ['food'],
      firstProductName: 'Cake',
      firstProductPrice: '20'
    });

    // Mock state fetch success
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({})
    });

    // Mock start failure
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      json: async () => ({})
    });

    act(() => { render(<OnboardingWizard />); });

    const launchButton = screen.getByRole('button', { name: /Approve & Launch/i });

    await act(async () => {
      launchButton.click();
    });

    // Verify error appears and step goes back to 2
    await waitFor(() => {
      expect(screen.getByText("Failed to start onboarding")).toBeInTheDocument();
      expect(screen.getByText("Review Details")).toBeInTheDocument();
    });
  });

  it('Step 5: Shows Live Screen with correct links', async () => {
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
