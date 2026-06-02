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
      chatStep: 0,
      isInstantBuildMode: false,
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

    expect(screen.getByText("Choose Setup Mode")).toBeInTheDocument();
    const guidedButton = screen.getByRole('button', { name: /Guided Setup/i });
    const instantButton = screen.getByRole('button', { name: /Instant Build/i });

    expect(guidedButton).toBeInTheDocument();
    expect(instantButton).toBeInTheDocument();
  });

  it('Handles instant build mode flow successfully', async () => {
    const user = userEvent.setup({ delay: null });

    // Mock API success
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/intake') {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            business_type: 'Handyman',
            business_name: 'Carlos Fixes',
            categories: ['services'],
            initial_products: [{ name: 'Repair', price: '100' }]
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

    const instantButton = screen.getByRole('button', { name: /Instant Build/i });
    await user.click(instantButton);

    const descriptionInput = await screen.findByPlaceholderText(/I bake custom vegan cakes for weddings/i);
    await user.type(descriptionInput, 'I am a handyman in NYC{Enter}');

    // Verify it jumps directly to start/live screen (Step 5)
    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("Success!")).toBeInTheDocument();
    });

    const state = useOnboardingStore.getState();
    expect(state.businessName).toBe('Carlos Fixes');
    expect(state.businessType).toBe('Handyman');
    expect(state.location).toBe('Online'); // Default
    expect(state.websiteTemplate).toBe('Modern'); // Default
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

    render(<OnboardingWizard />);

    const guidedButton = screen.getByRole('button', { name: /Guided Setup/i });
    await user.click(guidedButton);

    // Chat Step 1 - Use Enter Key
    const nameInput = await screen.findByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, 'Maya Bakery{Enter}');

    // Chat Step 2 - Use Enter Key
    const sellInput = await screen.findByPlaceholderText(/I bake custom vegan cakes/i);
    await user.type(sellInput, 'Cakes{Enter}');

    // Chat Step 3 - Use Enter Key
    const locInput = await screen.findByPlaceholderText(/Portland, OR/i);
    await user.type(locInput, 'NY{Enter}');

    // Verify it transitions to Step 2: Review Details by triggering handleIntake
    await waitFor(() => {
      expect(screen.getByText("Review Details")).toBeInTheDocument();
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

    render(<OnboardingWizard />);

    const guidedButton = screen.getByRole('button', { name: /Guided Setup/i });
    await user.click(guidedButton);

    // Chat Step 1
    const nameInput = await screen.findByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, 'Maya Bakery');

    const nextBtn1 = screen.getByRole('button', { name: /Next/i });
    await user.click(nextBtn1);

    // Chat Step 2
    const sellInput = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await user.type(sellInput, 'Cakes');

    const nextBtn2 = screen.getByRole('button', { name: /Next/i });
    await user.click(nextBtn2);

    // Chat Step 3
    const locInput = screen.getByPlaceholderText(/Portland, OR/i);
    await user.type(locInput, 'NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).not.toBeDisabled();

    // Step 1: Intake
    await user.click(button);

    // Verify it transitions to Step 2: Review Details
    await waitFor(() => {
      expect(screen.getByText("Review Details")).toBeInTheDocument();
      expect(screen.getByDisplayValue("Maya Bakery")).toBeInTheDocument();
    });

    const continueButton = screen.getByRole('button', { name: /Continue/i });
    await user.click(continueButton);

    // Verify it transitions to Step 3: Style & Team
    await waitFor(() => {
      expect(screen.getByText("Style & Team")).toBeInTheDocument();
      expect(screen.getByText("Website Template")).toBeInTheDocument();
    });

    const launchButton = screen.getByRole('button', { name: /Launch Store/i });
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

    render(<OnboardingWizard />);

    const guidedButton = screen.getByRole('button', { name: /Guided Setup/i });
    await user.click(guidedButton);

    // Chat Step 1
    const nameInput = await screen.findByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, 'Maya Bakery');

    const nextBtn1 = screen.getByRole('button', { name: /Next/i });
    await user.click(nextBtn1);

    // Chat Step 2
    const sellInput = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await user.type(sellInput, 'Cakes');

    const nextBtn2 = screen.getByRole('button', { name: /Next/i });
    await user.click(nextBtn2);

    // Chat Step 3
    const locInput = screen.getByPlaceholderText(/Portland, OR/i);
    await user.type(locInput, 'NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });

    await user.click(button);

    // Verify error appears and step goes back to 1
    await waitFor(() => {
      expect(screen.getByText("Failed to process business details")).toBeInTheDocument();
      expect(screen.getByText("Where are you located?")).toBeInTheDocument();
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

    render(<OnboardingWizard />);

    const launchButton = screen.getByRole('button', { name: /Launch Store/i });

    await user.click(launchButton);

    // Verify error appears and step goes back to 3
    await waitFor(() => {
      expect(screen.getByText("Failed to start onboarding")).toBeInTheDocument();
      expect(screen.getByText("Style & Team")).toBeInTheDocument();
    });

    consoleErrorSpy.mockRestore();
  });

  it('Step 1: Displays validation error when business name is too short', async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 1,
        chatStep: 1, // Skip to step 1 to bypass the choose setup mode
        businessName: 'A',
        location: '',
        businessType: 'Online Store',
        categories: [],
        firstProductName: '',
        firstProductPrice: ''
      });
    });

    render(<OnboardingWizard />);

    const nextButton = screen.getByRole('button', { name: /Next/i });

    await user.click(nextButton);

    expect(await screen.findByText('Business Name must be at least 3 characters.')).toBeInTheDocument();
  });

  it('Step 2: Proceeds to Step 3 when validation passes', async () => {
    const user = userEvent.setup({ delay: null });

    // Set initial state to Step 2
    act(() => {
      useOnboardingStore.setState({
        step: 2,
        businessName: 'Valid Name',
        businessType: 'Bakery',
        categories: ['food'],
        domainChoice: 'subdomain',
        firstProductName: 'Cake',
        firstProductPrice: '20'
      });
    });

    render(<OnboardingWizard />);

    const continueButton = screen.getByRole('button', { name: /Continue/i });

    await user.click(continueButton);

    expect(screen.queryByText('Business Name must be at least 3 characters.')).not.toBeInTheDocument();
    expect(screen.getByText('Style & Team')).toBeInTheDocument();
  });

  it('Step 3: Can select Web Address, AI agents and toggle auto-respond', async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({ step: 3, aiAgents: [], aiAutoRespond: true, domainChoice: 'subdomain' });
    });

    render(<OnboardingWizard />);

    // Verify initial Web Address options
    const subdomainOption = screen.getByText('Free Subdomain');
    const customOption = screen.getByText('Custom Domain');
    expect(subdomainOption).toBeInTheDocument();
    expect(customOption).toBeInTheDocument();

    // Select Custom Domain
    await user.click(customOption);

    // Verify initial state
    const salesAgent = screen.getByText('Sales Agent');
    expect(salesAgent).toBeInTheDocument();

    // Check toggle
    const toggle = screen.getByRole('checkbox');
    expect(toggle).toBeChecked();

    // Select Sales Agent
    await user.click(salesAgent);

    // Toggle auto respond
    await user.click(toggle);

    await waitFor(() => {
      const state = useOnboardingStore.getState();
      expect(state.aiAgents).toContain('Sales Agent');
      expect(state.aiAutoRespond).toBe(false);
      expect(state.domainChoice).toBe('custom');
    });
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

    render(<OnboardingWizard />);

    const saveDraftButton = screen.getByRole('button', { name: /Save Draft/i });
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
