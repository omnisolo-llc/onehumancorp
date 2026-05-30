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
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: '',
      startResult: null,
    });




    global.fetch = vi.fn().mockImplementation((url) => {
        return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial screen correctly', async () => {
    act(() => { render(<OnboardingWizard />); });

    expect(screen.getByText("Tell us about your business")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).toBeDisabled();
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock intake success

    (global.fetch as any).mockImplementation((url, options) => {
      if (url === '/api/onboarding/intake') {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            business_type: 'Bakery',
            business_name: 'Maya Bakery',
            categories: ['food'],
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


    act(() => { render(<OnboardingWizard />); });

    // Single Step 1
    const descInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await userEvent.type(descInput, 'Maya Bakery. Cakes. NY');

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

    // Mock intake failure

    (global.fetch as any).mockImplementation((url, options) => {
      if (url === '/api/onboarding/intake' || url === '/api/onboarding/start') {
        return Promise.resolve({ ok: false });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });


    act(() => { render(<OnboardingWizard />); });

    // Single Step 1
    const descInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await userEvent.type(descInput, 'Maya Bakery. Cakes. NY');

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

    // Mock start failure

    (global.fetch as any).mockImplementation((url, options) => {
      if (url === '/api/onboarding/intake' || url === '/api/onboarding/start') {
        return Promise.resolve({ ok: false });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });


    act(() => { render(<OnboardingWizard />); });

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


  it('Step 2: Displays validation error when business name is too short', async () => {
    // Set initial state to Step 2
    useOnboardingStore.setState({
      step: 2,
      businessName: 'A',
      businessType: 'Bakery',
      categories: ['food'],
      firstProductName: 'Cake',
      firstProductPrice: '20'
    });

    act(() => { render(<OnboardingWizard />); });

    const continueButton = screen.getByRole('button', { name: /Continue/i });

    await act(async () => {
      continueButton.click();
    });

    expect(await screen.findByText('Business Name must be at least 3 characters.')).toBeInTheDocument();
  });

  it('Step 3: Can select AI agents and toggle auto-respond', async () => {
    useOnboardingStore.setState({ step: 3, aiAgents: [], aiAutoRespond: true });

    act(() => { render(<OnboardingWizard />); });

    // Verify initial state
    const salesAgent = screen.getByText('Sales Agent');
    expect(salesAgent).toBeInTheDocument();

    // Check toggle
    const toggle = screen.getByRole('checkbox');
    expect(toggle).toBeChecked();

    // Select Sales Agent
    await act(async () => {
      salesAgent.click();
    });

    // Toggle auto respond
    await act(async () => {
      toggle.click();
    });

    await waitFor(() => {
      const state = useOnboardingStore.getState();
      expect(state.aiAgents).toContain('Sales Agent');
      expect(state.aiAutoRespond).toBe(false);
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
