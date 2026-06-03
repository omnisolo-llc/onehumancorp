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

    (global.fetch as any).mockImplementation((url) => {
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

    (global.fetch as any).mockImplementation((url) => {
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
      expect(screen.getByText("Review Details")).toBeInTheDocument();
    });
  });

  it('Restores detailed setup flow testing from Step 2 onwards', async () => {
    const user = userEvent.setup({ delay: null });

    // Mock start success
    (global.fetch as any).mockImplementation((url) => {
      if (url === '/api/onboarding/start') {
        return Promise.resolve({
          ok: true,
          json: async () => ({ message: "Success!" })
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

    // Start at step 2
    act(() => {
      useOnboardingStore.setState({ step: 2 });
    });

    render(<OnboardingWizard />);

    const continueButton = screen.getByRole('button', { name: /Continue/i });

    // We need to fill data since it's step 2 validation
    act(() => {
      useOnboardingStore.setState({
        businessName: 'Maya Bakery',
        businessType: 'Online Store',
        categories: ['food'],
        firstProductName: 'Cake',
        firstProductPrice: '20'
      });
    });

    await user.click(continueButton);

    await waitFor(() => {
      expect(screen.getByText("Style & Team")).toBeInTheDocument();
    });

    const emailInput = screen.getByPlaceholderText(/you@example.com/i);
    await user.type(emailInput, 'maya@example.com');

    const passInput = screen.getByPlaceholderText(/••••••••/i);
    await user.type(passInput, 'mypassword123');

    const launchButton = screen.getByRole('button', { name: /Launch Store/i });
    await user.click(launchButton);

    await waitFor(() => {
      expect(screen.getByText("You're Live!")).toBeInTheDocument();
    });
  });
});
