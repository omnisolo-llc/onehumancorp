import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { TooltipProvider } from '../../components/TooltipRegistry';
import { beforeEach, describe, it, expect, vi, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';

describe('OnboardingWizard', () => {
  const renderOnboardingWizard = async () => {
    let view: any;
    await act(async () => {
      view = render(
        <TooltipProvider>
          <OnboardingWizard />
        </TooltipProvider>
      );
    });
    return view;
  };

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
    });

    global.fetch = vi.fn().mockImplementation((url) => {
        return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    }) as any;
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial screen correctly', async () => {
    await renderOnboardingWizard();

    expect(screen.getByText("Tell us about your business")).toBeInTheDocument();
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

    await renderOnboardingWizard();

    // Chat Step 1 - Use Enter Key
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
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

  it('Handles validation failures when fields are empty', async () => {
    const user = userEvent.setup({ delay: null });

    await renderOnboardingWizard();

    // Chat Step 1 - Enter Key with short name
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, 'Ma{Enter}');
    expect(await screen.findByText('Business Name must be at least 3 characters.')).toBeInTheDocument();

    await user.clear(nameInput);
    await user.type(nameInput, 'Maya Bakery{Enter}');

    // Chat Step 2 - Next click with empty value
    const sellInput = await screen.findByPlaceholderText(/I bake custom vegan cakes/i);

    // Test validation with missing data
    await user.clear(sellInput);

    const nextBtn2 = screen.getByRole('button', { name: /Next/i });

    // Verify the button is disabled when empty
    expect(nextBtn2).toBeDisabled();

    // Provide value to enable button and proceed
    await user.type(sellInput, 'Cakes');
    expect(nextBtn2).not.toBeDisabled();
    await user.type(sellInput, '{Enter}');

    // Chat Step 3 - Next click with empty value
    const locInput = await screen.findByPlaceholderText(/Portland, OR/i);

    await user.clear(locInput);

    const nextBtn3 = screen.getByRole('button', { name: /Generate My Business/i });

    // Verify the button is disabled when empty
    expect(nextBtn3).toBeDisabled();

    // Provide value to enable button and proceed
    await user.type(locInput, 'NY');
    expect(nextBtn3).not.toBeDisabled();
    await user.click(nextBtn3);
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

    await renderOnboardingWizard();

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

    // Fill in Account Setup fields
    const nameInput2 = screen.getByPlaceholderText(/e.g. Maya Smith/i);
    await user.type(nameInput2, 'Maya Smith');

    const emailInput = screen.getByPlaceholderText(/you@example.com/i);
    await user.type(emailInput, 'maya@example.com');

    const passwordInput = screen.getByPlaceholderText(/••••••••/i);
    await user.type(passwordInput, 'mypassword123');

    const launchButton = screen.getByRole('button', { name: /Launch Store/i });
    await user.click(launchButton);

    // Verify it transitions to Step 5 (Live Screen) on success
    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("my-business.ohc.store")).toBeInTheDocument();
    });

    // Check that start API was called with the correct credentials
    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/start', expect.objectContaining({
      method: 'POST',
      body: expect.stringContaining('"admin_name":"Maya Smith"'),
    }));
    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/start', expect.objectContaining({
      method: 'POST',
      body: expect.stringContaining('"admin_email":"maya@example.com"'),
    }));
    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/start', expect.objectContaining({
      method: 'POST',
      body: expect.stringContaining('"admin_password":"mypassword123"'),
    }));
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

    await renderOnboardingWizard();

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

    await renderOnboardingWizard();

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
        chatStep: 1,
        businessName: 'A',
        location: '',
        businessType: 'Online Store',
        categories: [],
        firstProductName: '',
        firstProductPrice: ''
      });
    });

    await renderOnboardingWizard();

    const nextButton = screen.getByRole('button', { name: /Next/i });

    await user.click(nextButton);

    expect(await screen.findByText('Business Name must be at least 3 characters.')).toBeInTheDocument();
  });

  it('Step 2: Displays validation error when product price is invalid', async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 2,
        businessName: 'Valid Name',
        businessType: 'Bakery',
        categories: ['food'],
        domainChoice: 'subdomain',
        firstProductName: 'Cake',
        firstProductPrice: 'abc' // Invalid price
      });
    });

    await renderOnboardingWizard();

    const continueButton = screen.getByRole('button', { name: /Continue/i });
    expect(continueButton).not.toBeDisabled(); // Button should not be disabled based on input length, but validation will stop it

    const priceInput = screen.getByDisplayValue('abc');
    await user.type(priceInput, 'd'); // Type 'd' to trigger the onChange validation.

    await user.click(continueButton);

    await waitFor(() => {
      // The general error message should trigger
      expect(screen.getByText('Please fix the errors before continuing.')).toBeInTheDocument();
      expect(screen.getByText('Invalid price.')).toBeInTheDocument();
    });

    // Check that we're still on step 2
    expect(useOnboardingStore.getState().step).toBe(2);
  });

  it('Step 2: Displays validation error when business type is empty', async () => {
    const user = userEvent.setup({ delay: null });

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

    await renderOnboardingWizard();

    const continueButton = screen.getByRole('button', { name: /Continue/i });
    expect(continueButton).not.toBeDisabled();

    // Find the input element that is associated with the 'Business Type' label
    const inputs = screen.getAllByRole('textbox');
    const businessTypeInput = screen.getByDisplayValue('Bakery');

    // Clear the input to trigger validation
    await user.clear(businessTypeInput);

    // Button should now be disabled because businessType is empty
    expect(continueButton).toBeDisabled();

    // Type something to make it empty string on blur or just type and clear
    await user.type(businessTypeInput, 'A');
    await user.clear(businessTypeInput);

    await waitFor(() => {
      expect(screen.getByText('Business Type is required to configure your agents.')).toBeInTheDocument();
    });
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

    await renderOnboardingWizard();

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

    await renderOnboardingWizard();

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
    // Checkbox might be hidden by sr-only or similar, use label text instead or get by id
    const toggle = document.querySelector('input[type="checkbox"]') as HTMLInputElement;
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

    await renderOnboardingWizard();

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("Your business has been successfully launched.")).toBeInTheDocument();
      expect(screen.getByRole('link', { name: /Go to Dashboard/i })).toBeInTheDocument();
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
      expect(screen.getByText('What do you sell?')).toBeInTheDocument();
    });

    expect(screen.getByDisplayValue('Draft Products')).toBeInTheDocument();
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

    await renderOnboardingWizard();

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

describe('Onboarding Wizard Back Navigation', () => {
  it('allows user to navigate back from step 2 to step 1', async () => {
    useOnboardingStore.setState({ step: 2 });
    await act(async () => {
      render(<TooltipProvider><OnboardingWizard /></TooltipProvider>);
    });

    const backButton = screen.getByRole('button', { name: /Back/i });
    await userEvent.click(backButton);

    expect(useOnboardingStore.getState().step).toBe(1);
  });

  it('allows user to navigate back from step 3 to step 2', async () => {
    useOnboardingStore.setState({ step: 3 });
    await act(async () => {
      render(<TooltipProvider><OnboardingWizard /></TooltipProvider>);
    });

    const backButton = screen.getByRole('button', { name: /Back/i });
    await userEvent.click(backButton);

    expect(useOnboardingStore.getState().step).toBe(2);
  });
});
