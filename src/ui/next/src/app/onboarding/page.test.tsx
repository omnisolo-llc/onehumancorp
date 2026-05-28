import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { beforeEach, describe, it, expect, vi, afterEach } from 'vitest';

describe('OnboardingWizard', () => {
  let intakeShouldFail = false;
  let startShouldFail = false;

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
    });

    intakeShouldFail = false;
    startShouldFail = false;

    global.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url.includes('/api/onboarding/state')) {
        return { ok: true, json: async () => ({}) };
      }
      if (url.includes('/api/onboarding/intake')) {
        if (intakeShouldFail) return { ok: false };
        return {
          ok: true,
          json: async () => ({
            business_type: 'Bakery',
            business_name: 'Maya Bakery',
            categories: ['food'],
            initial_products: [{ name: 'Cake', price: '20' }]
          })
        };
      }
      if (url.includes('/api/onboarding/start')) {
        if (startShouldFail) return { ok: false };
        return {
          ok: true,
          json: async () => ({ message: "Success!" })
        };
      }
      return { ok: true, json: async () => ({}) };
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial screen correctly', async () => {
    act(() => { render(<OnboardingWizard />); });

    expect(screen.getByText("What's the name of your business?")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Next/i });
    expect(button).toBeDisabled();
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    act(() => { render(<OnboardingWizard />); });

    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await userEvent.type(nameInput, 'Maya Bakery');

    const nextBtn1 = screen.getByRole('button', { name: /Next/i });
    await act(async () => { nextBtn1.click(); });

    const sellInput = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(sellInput, 'Cakes');

    const nextBtn2 = screen.getByRole('button', { name: /Next/i });
    await act(async () => { nextBtn2.click(); });

    const locInput = screen.getByPlaceholderText(/Portland, OR/i);
    await userEvent.type(locInput, 'NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).not.toBeDisabled();

    await act(async () => {
      button.click();
    });

    await waitFor(() => {
      expect(screen.getByText("Review Details")).toBeInTheDocument();
      expect(screen.getByDisplayValue("Maya Bakery")).toBeInTheDocument();
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

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("my-business.ohc.store")).toBeInTheDocument();
    });
  });

  it('Step 1: Handles intake API failure', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    intakeShouldFail = true;

    act(() => { render(<OnboardingWizard />); });

    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await userEvent.type(nameInput, 'Maya Bakery');

    const nextBtn1 = screen.getByRole('button', { name: /Next/i });
    await act(async () => { nextBtn1.click(); });

    const sellInput = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(sellInput, 'Cakes');

    const nextBtn2 = screen.getByRole('button', { name: /Next/i });
    await act(async () => { nextBtn2.click(); });

    const locInput = screen.getByPlaceholderText(/Portland, OR/i);
    await userEvent.type(locInput, 'NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });

    await act(async () => {
      button.click();
    });

    await waitFor(() => {
      expect(screen.getByText("Failed to process business details")).toBeInTheDocument();
      expect(screen.getByText("Where are you located?")).toBeInTheDocument();
    });
  });

  it('Step 3: Handles start API failure and returns to Step 3', async () => {
    useOnboardingStore.setState({ step: 3 });

    startShouldFail = true;

    act(() => { render(<OnboardingWizard />); });

    const launchButton = screen.getByRole('button', { name: /Launch Store/i });

    await act(async () => {
      launchButton.click();
    });

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

    act(() => { render(<OnboardingWizard />); });

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("Your business has been successfully launched.")).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Go to Dashboard/i })).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Preview Storefront/i })).toBeInTheDocument();
    });
  });

  it('Loads state from backend on mount', async () => {
    global.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url.includes('/api/onboarding/state')) {
        return { ok: true, json: async () => ({ businessName: 'Restored Bakery', step: 2 }) };
      }
      return { ok: true };
    });

    act(() => { render(<OnboardingWizard />); });

    await waitFor(() => {
      expect(useOnboardingStore.getState().businessName).toBe('Restored Bakery');
      expect(useOnboardingStore.getState().step).toBe(2);
    });
  });


  it('Debounces state syncs correctly', async () => {
    vi.useRealTimers();

    // reset store manually to make sure
    act(() => { useOnboardingStore.setState({ businessName: '' }); });

    global.fetch = vi.fn().mockImplementation(async (url: string) => {
      if (url.includes('/api/onboarding/state')) {
        return { ok: true, json: async () => ({ businessName: 'MOCKED_BACKEND_STATE' }) };
      }
      return { ok: true, json: async () => ({}) };
    });

    act(() => { render(<OnboardingWizard />); });

    // Wait for the initial loadState to finish
    await waitFor(() => {
      expect(useOnboardingStore.getState().businessName).toBe('MOCKED_BACKEND_STATE');
    });

    (global.fetch as any).mockClear();

    act(() => { useOnboardingStore.getState().setBusinessName('Test 1'); });
    act(() => { useOnboardingStore.getState().setBusinessName('Test 2'); });

    await waitFor(() => {
      const postCalls = (global.fetch as any).mock.calls.filter((c: any) => c[1]?.method === 'POST');
      expect(postCalls.length).toBeGreaterThan(0);
    }, { timeout: 2000 });

    const postCalls = (global.fetch as any).mock.calls.filter((c: any) => c[1]?.method === 'POST');
    expect(JSON.parse(postCalls[postCalls.length - 1][1].body).businessName).toBe('Test 2');
  });
});
