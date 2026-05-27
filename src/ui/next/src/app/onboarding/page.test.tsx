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
      businessCategory: '',
      businessGoal: '',
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

    expect(screen.getByText("What is the name of your business?")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Next/i });
    expect(button).toBeDisabled();
  });

  it('Step 1-3: Flows correctly and handles successful onboarding', async () => {
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

    // Step 1: Business Name
    const nameInput = screen.getByPlaceholderText(/e.g. Maya's Cakes/i);
    await userEvent.type(nameInput, 'Maya Bakery');

    const nextBtn1 = screen.getByRole('button', { name: /Next/i });
    expect(nextBtn1).not.toBeDisabled();
    await act(async () => { nextBtn1.click(); });

    // Step 2: Business Category
    await waitFor(() => expect(screen.getByText("What kind of business is it?")).toBeInTheDocument());
    const categoryInput = screen.getByPlaceholderText(/e.g. Food\/Bakery/i);
    await userEvent.type(categoryInput, 'Bakery');

    const nextBtn2 = screen.getByRole('button', { name: /Next/i });
    expect(nextBtn2).not.toBeDisabled();
    await act(async () => { nextBtn2.click(); });

    // Step 3: Business Goal
    await waitFor(() => expect(screen.getByText("What is your main goal?")).toBeInTheDocument());
    const goalInput = screen.getByPlaceholderText(/e.g. Sell my custom cakes online/i);
    await userEvent.type(goalInput, 'Sell cakes');

    const generateBtn = screen.getByRole('button', { name: /Generate My Business/i });
    expect(generateBtn).not.toBeDisabled();

    // Trigger submission
    await act(async () => { generateBtn.click(); });

    // Verify it transitions to step 5 on success
    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("my-business.ohc.store")).toBeInTheDocument();
    });
  });

  it('Step 3: Handles intake API failure', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock intake failure
    (global.fetch as any).mockResolvedValueOnce({
      ok: false
    });

    // We start directly from Step 3 for this test
    useOnboardingStore.setState({
      step: 3,
      businessName: 'Maya Bakery',
      businessCategory: 'Bakery',
      businessGoal: 'Sell cakes'
    });

    act(() => { render(<OnboardingWizard />); });

    const generateBtn = screen.getByRole('button', { name: /Generate My Business/i });
    await act(async () => { generateBtn.click(); });

    // Verify error appears and step goes back to 1
    await waitFor(() => {
      expect(screen.getByText("Failed to process business details")).toBeInTheDocument();
      expect(screen.getByText("What is the name of your business?")).toBeInTheDocument();
    });
  });

  it('Step 3: Handles start API failure', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock intake success
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({})
    });

    // Mock start failure
    (global.fetch as any).mockResolvedValueOnce({
      ok: false
    });

    useOnboardingStore.setState({
      step: 3,
      businessName: 'Maya Bakery',
      businessCategory: 'Bakery',
      businessGoal: 'Sell cakes'
    });

    act(() => { render(<OnboardingWizard />); });

    const generateBtn = screen.getByRole('button', { name: /Generate My Business/i });
    await act(async () => { generateBtn.click(); });

    // Verify error appears and step goes back to 1
    await waitFor(() => {
      expect(screen.getByText("Failed to start onboarding")).toBeInTheDocument();
      expect(screen.getByText("What is the name of your business?")).toBeInTheDocument();
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