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

    global.fetch = vi.fn().mockImplementation((url: string) => {
        return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    }) as any;
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial Instant Build screen correctly', async () => {
    render(<OnboardingWizard />);

    expect(screen.getByText("Tell us about your business")).toBeInTheDocument();
    expect(screen.getByRole('textbox')).toBeInTheDocument();
    const instantLaunchBtn = screen.getByRole('button', { name: /Instant Launch/i });
    expect(instantLaunchBtn).toBeDisabled();
    expect(screen.getByRole('button', { name: /Detailed Setup/i })).toBeInTheDocument();
  });

  it('Performs Instant Launch successfully', async () => {
    const user = userEvent.setup({ delay: null });

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

    const textarea = screen.getByRole('textbox');
    await user.type(textarea, 'I am building a vegan bakery called Maya Bakery selling custom cakes in Portland.');

    const instantLaunchBtn = screen.getByRole('button', { name: /Instant Launch/i });
    expect(instantLaunchBtn).not.toBeDisabled();

    await user.click(instantLaunchBtn);

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
      expect(screen.getByText("my-business.ohc.store")).toBeInTheDocument();
    });

    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/start', expect.objectContaining({
      method: 'POST',
      body: expect.stringContaining('"admin_email":'),
    }));
  });

  it('Handles intake API failure during Instant Launch', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const user = userEvent.setup({ delay: null });

    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/intake') {
        return Promise.resolve({ ok: false, json: async () => ({ error: "Failed to process business details" }) });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

    render(<OnboardingWizard />);

    const textarea = screen.getByRole('textbox');
    await user.type(textarea, 'A random description that is long enough.');

    const instantLaunchBtn = screen.getByRole('button', { name: /Instant Launch/i });
    await user.click(instantLaunchBtn);

    await waitFor(() => {
      expect(screen.getByText("Failed to process business details")).toBeInTheDocument();
    });

    consoleErrorSpy.mockRestore();
  });

  it('Navigates to Detailed Setup', async () => {
    const user = userEvent.setup({ delay: null });

    render(<OnboardingWizard />);

    const detailedSetupBtn = screen.getByRole('button', { name: /Detailed Setup/i });
    await user.click(detailedSetupBtn);

    await waitFor(() => {
      expect(screen.getByText("What's the name of your business?")).toBeInTheDocument();
    });
  });

  it('Step 1: Displays validation error when description is too short', async () => {
    const user = userEvent.setup({ delay: null });

    render(<OnboardingWizard />);

    const textarea = screen.getByRole('textbox');
    await user.type(textarea, 'Short');

    const instantLaunchBtn = screen.getByRole('button', { name: /Instant Launch/i });
    expect(instantLaunchBtn).toBeDisabled();
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

    render(<OnboardingWizard />);

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
    await user.click(screen.getByText('Allow AI to Auto-Respond'));

    // Check that states updated.
    const state = useOnboardingStore.getState();
    expect(state.domainChoice).toBe('custom');
    expect(state.aiAgents).toContain('Sales Agent');
    expect(state.aiAutoRespond).toBe(false);
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
