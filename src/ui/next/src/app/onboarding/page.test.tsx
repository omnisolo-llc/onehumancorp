import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { TooltipProvider } from '../../components/TooltipRegistry';
import { beforeEach, describe, it, expect, vi, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';

describe('OnboardingWizard', () => {
  const renderOnboardingWizard = () => render(
    <TooltipProvider>
      <OnboardingWizard />
    </TooltipProvider>
  );

  beforeEach(() => {
    localStorage.clear();
    useOnboardingStore.setState({
      step: 1,
      chatStep: 1,
      businessName: '',
      whatYouSell: '',
      location: '',
      businessDescription: '',
      domainChoice: 'subdomain',
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: '',
      startResult: null,
      saveMessage: ''
    });

    global.fetch = vi.fn().mockImplementation((url) => {
        return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    }) as any;
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial chat screen correctly', async () => {
    renderOnboardingWizard();

    expect(screen.getByText("Onboarding Expert")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Next/i });
    expect(button).toBeDisabled();
  });

  it('Handles enter key progression in chat steps', async () => {
    const user = userEvent.setup({ delay: null });

    // Mock intake success
    (global.fetch as any).mockImplementation((url: string) => {
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
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

    renderOnboardingWizard();

    // Chat Step 1 - Use Enter Key
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, 'Maya Bakery{Enter}');

    // Chat Step 2 - Use Enter Key
    const sellInput = await screen.findByPlaceholderText(/I bake custom vegan cakes/i);
    await user.type(sellInput, 'Cakes{Enter}');

    // Chat Step 3 - Use Enter Key
    const locInput = await screen.findByPlaceholderText(/Portland, OR/i);
    await user.type(locInput, 'NY{Enter}');

    // Verify it transitions to Step 2: Review Your Business by triggering handleIntake
    await waitFor(() => {
      expect(screen.getByText("Review Your Business")).toBeInTheDocument();
      expect(screen.getByDisplayValue("Maya Bakery")).toBeInTheDocument();
    });
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const user = userEvent.setup({ delay: null });

    // Mock intake success
    (global.fetch as any).mockImplementation((url: string) => {
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

    renderOnboardingWizard();

    // Chat Step 1
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, 'Maya Bakery');

    const nextBtn1 = screen.getByRole('button', { name: /Next/i });
    await user.click(nextBtn1);

    // Chat Step 2
    const sellInput = await screen.findByPlaceholderText(/I bake custom vegan cakes/i, {}, { timeout: 3000 });
    await user.type(sellInput, 'Cakes');

    const nextBtn2 = screen.getByRole('button', { name: /Next/i });
    await user.type(sellInput, '{Enter}');

    // Chat Step 3
    const locInput = await screen.findByPlaceholderText(/Portland, OR/i, {}, { timeout: 3000 });
    await user.type(locInput, 'NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).not.toBeDisabled();

    // Step 1: Intake
    await user.click(button);

    // Verify it transitions to Step 2: Review Your Business
    await waitFor(() => {
      expect(screen.getByText("Review Your Business")).toBeInTheDocument();
      expect(screen.getByDisplayValue("Maya Bakery")).toBeInTheDocument();
    });

    const continueButton = screen.getByRole('button', { name: /Everything Looks Good/i });
    await user.click(continueButton);

    // Verify it transitions to Step 3: Style & Team
    await waitFor(() => {
      expect(screen.getByText("Style & Team")).toBeInTheDocument();
      expect(screen.getByText("Storefront Vibe")).toBeInTheDocument();
    });

    // Fill in Account Setup fields
    const nameInput2 = screen.getByPlaceholderText(/Maya Smith/i);
    await user.type(nameInput2, 'Maya Smith');

    const emailInput = screen.getByPlaceholderText(/you@example.com/i);
    await user.type(emailInput, 'maya@example.com');

    const passwordInput = screen.getByPlaceholderText(/••••••••/i);
    await user.type(passwordInput, 'mypassword123');

    const launchButton = screen.getByRole('button', { name: /Launch My Business/i });
    await user.click(launchButton);

    // Verify it transitions to Step 5 (Live Screen) on success
    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("my-business.ohc.store")).toBeInTheDocument();
    });
  });

  it('Step 1: Handles intake API failure', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const user = userEvent.setup({ delay: null });

    // Mock intake failure
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/intake' || url === '/api/onboarding/start') {
        return Promise.resolve({ ok: false, json: async () => ({ error: "Failed to process business details" }) });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

    renderOnboardingWizard();

    // Chat Step 1
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, 'Maya Bakery');

    const nextBtn1 = screen.getByRole('button', { name: /Next/i });
    await user.click(nextBtn1);

    // Chat Step 2
    const sellInput = await screen.findByPlaceholderText(/I bake custom vegan cakes/i, {}, { timeout: 3000 });
    await user.type(sellInput, 'Cakes');

    const nextBtn2 = screen.getByRole('button', { name: /Next/i });
    await user.type(sellInput, '{Enter}');

    // Chat Step 3
    const locInput = await screen.findByPlaceholderText(/Portland, OR/i, {}, { timeout: 3000 });
    await user.type(locInput, 'NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });

    await user.click(button);

    // Verify error appears and step stays in Step 1
    await waitFor(() => {
      expect(screen.getByText("Failed to process business details")).toBeInTheDocument();
      expect(screen.getByText("Onboarding Expert")).toBeInTheDocument();
    });

    consoleErrorSpy.mockRestore();
  });

  it('Step 3: Handles start API failure and returns to Step 3', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const user = userEvent.setup({ delay: null });

    // Set initial state to Step 3 to test start API directly
    act(() => {
      useOnboardingStore.setState({ step: 3 });
    });

    // Mock start failure
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/intake' || url === '/api/onboarding/start') {
        return Promise.resolve({ ok: false, json: async () => ({ error: "Failed to start onboarding" }) });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

    renderOnboardingWizard();

    const launchButton = screen.getByRole('button', { name: /Launch My Business/i });

    await user.click(launchButton);

    // Verify error appears and step stays at 3
    await waitFor(() => {
      expect(screen.getByText("Failed to start onboarding")).toBeInTheDocument();
      expect(screen.getByText("Style & Team")).toBeInTheDocument();
    });

    consoleErrorSpy.mockRestore();
  });

  it('Step 5: Shows Live Screen with correct links', async () => {
    act(() => {
      useOnboardingStore.setState({
        step: 5,
        startResult: { message: "Your business has been successfully launched." }
      });
    });

    renderOnboardingWizard();

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("Your business has been successfully launched.")).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Enter Dashboard/i })).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Preview Storefront/i })).toBeInTheDocument();
    });
  });

  it('loads draft state correctly on mount', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/draft') {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            wizardState: {
              step: 1,
              chatStep: 2,
              businessName: 'Draft Business Name',
              whatYouSell: 'Draft Products'
            }
          })
        });
      }
      if (url === '/api/onboarding/state') {
        return Promise.resolve({
          ok: true,
          json: async () => ({ wizardState: {} })
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    render(<OnboardingWizard />);

    // Wait for the mock fetch to resolve and state to update
    await waitFor(() => {
      expect(screen.getByText('Draft Products')).toBeInTheDocument();
    });

    // In conversational UI, business name is in a chat bubble, not an input at this step
    expect(screen.getByText('Draft Business Name')).toBeInTheDocument();
  });

  it('Save Draft button triggers draft API and shows success message', async () => {
    const user = userEvent.setup({ delay: null });

    // Mock draft API success
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/draft') {
        return Promise.resolve({
          ok: true,
          json: async () => ({})
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

    // Start at Step 2
    act(() => {
      useOnboardingStore.setState({ step: 2 });
    });

    renderOnboardingWizard();

    const saveDraftButton = screen.getAllByRole('button', { name: /Save Draft/i })[0];
    expect(saveDraftButton).toBeInTheDocument();

    await user.click(saveDraftButton);

    await waitFor(() => {
      expect(screen.getByText('Draft Saved!')).toBeInTheDocument();
    });

    // Verify API was called
    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/draft', expect.objectContaining({
      method: 'POST'
    }));
  });
});
