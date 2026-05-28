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
      chatStep: 1,
      businessName: '',
      whatYouSell: '',
      location: '',
      businessDescription: '',
      isLoading: false,
      error: '',
      startResult: null,
      selectedAgents: ['The Manager'],
      brandTone: 'Professional',
      firstProductName: '',
      firstProductPrice: ''
    });

    global.fetch = vi.fn().mockImplementation(() => Promise.resolve({ ok: true, json: () => Promise.resolve({}) }));
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial screen correctly', async () => {
    act(() => { render(<OnboardingWizard />); });

    expect(screen.getByText("What's your business name?")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Next/i });
    expect(button).toBeDisabled();
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock intake success
    (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => ({})
    }) // initial track call in useEffect
    .mockResolvedValueOnce({
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

    // Chat Step 1
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await userEvent.type(nameInput, 'Maya Bakery');

    const nextBtn1 = screen.getByRole('button', { name: /Next/i });
    await act(async () => { nextBtn1.click(); });

    // Chat Step 2
    const sellInput = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(sellInput, 'Cakes');

    const nextBtn2 = screen.getByRole('button', { name: /Next/i });
    await act(async () => { nextBtn2.click(); });

    // Chat Step 3
    const locInput = screen.getByPlaceholderText(/Portland, OR/i);
    await userEvent.type(locInput, 'NY');

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

    // Verify it transitions to Step 3: Brand Style
    await waitFor(() => {
      expect(screen.getByText("Brand Style")).toBeInTheDocument();
    });

    const nextBtn3 = screen.getByRole('button', { name: /Continue/i });
    await act(async () => {
      nextBtn3.click();
    });

    // Verify it transitions to Step 4: Your AI Team
    await waitFor(() => {
        expect(screen.getByText("Your AI Team")).toBeInTheDocument();
    });

    const launchButton = screen.getByRole('button', { name: /Launch My Business/i });
    await act(async () => {
      launchButton.click();
    });

    // Verify it transitions to Step 6 (Live Screen) on success
    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("maya-bakery.ohc.store")).toBeInTheDocument();
    });
  });

  it('Validation: Business name must be at least 2 characters', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });
    act(() => { render(<OnboardingWizard />); });

    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    const nextBtn = screen.getByRole('button', { name: /Next/i });

    await userEvent.type(nameInput, 'M');
    expect(nextBtn).toBeDisabled();

    await userEvent.type(nameInput, 'a');
    expect(nextBtn).not.toBeDisabled();
  });

  it('Validation: Price must be numeric and non-negative', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });
    useOnboardingStore.setState({
        step: 2,
        businessName: 'Valid Name',
        businessType: 'Store',
        firstProductName: 'Valid Product',
        firstProductPrice: ''
    });

    act(() => { render(<OnboardingWizard />); });

    const priceInput = screen.getByLabelText(/Price/i);
    const continueBtn = screen.getByRole('button', { name: /Continue/i });

    await userEvent.type(priceInput, 'abc');
    expect(continueBtn).toBeDisabled();

    await userEvent.clear(priceInput);
    await userEvent.type(priceInput, '-10');
    expect(continueBtn).toBeDisabled();

    await userEvent.clear(priceInput);
    await userEvent.type(priceInput, '10.50');
    expect(continueBtn).not.toBeDisabled();
  });
});
