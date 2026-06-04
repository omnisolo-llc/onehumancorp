import { render, screen, act, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { vi } from 'vitest';
import OnboardingWizard from './page';
import { WalkthroughProvider } from '../../components/help';
import { useOnboardingStore } from './store';

// Mock matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(), // deprecated
    removeListener: vi.fn(), // deprecated
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

// Mock ResizeObserver
class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
}
window.ResizeObserver = ResizeObserver;

describe('OnboardingWizard', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ wizardState: {} })
    }));

    act(() => {
      useOnboardingStore.setState({
        step: 0,
        businessName: '',
        businessType: '',
        hasPhysicalProducts: false,
        hasDigitalProducts: false,
        firstProductName: '',
        firstProductPrice: '',
        paymentMethod: '',
        adminEmail: '',
        adminPassword: '',
        template: 'Modern',
        domainChoice: 'subdomain',
        aiAgents: [],
        aiAutoRespond: true,
        isLoading: false,
        error: '',
        startResult: null,
        status: 'idle',
      });
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('Renders initial screen correctly', () => {
    render(<WalkthroughProvider><OnboardingWizard /></WalkthroughProvider>);
    expect(screen.getByText('10-Minute Setup Wizard')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Start My Business/i })).toBeInTheDocument();
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const user = userEvent.setup({ delay: null });

    // Mock the start API to return success
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/state') {
        return Promise.resolve({ ok: true, json: async () => ({ message: "Draft Saved!" }) });
      }
      if (url === '/api/onboarding/start') {
        return Promise.resolve({ ok: true, json: async () => ({ message: "Your business has been successfully launched." }) });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

    render(<WalkthroughProvider><OnboardingWizard /></WalkthroughProvider>);

    // Step 0 -> 1
    await waitFor(() => {
        expect(screen.getByRole('button', { name: /Start My Business/i })).toBeInTheDocument();
    });

    await user.click(screen.getByRole('button', { name: /Start My Business/i }));
    expect(await screen.findByText('What kind of business are you building?')).toBeInTheDocument();

    // Step 1 -> 2
    await user.click(screen.getByRole('button', { name: /Online Store/i }));
    expect(await screen.findByText('Give your business a name')).toBeInTheDocument();

    // Step 2
    await user.type(screen.getByPlaceholderText('What is your business called?'), 'Maya Bakery');
    await user.click(screen.getByRole('button', { name: /Next/i }));

    // Step 3
    expect(await screen.findByText('What do you sell?')).toBeInTheDocument();
    await user.click(screen.getByLabelText(/Physical Products/i));
    await user.click(screen.getByRole('button', { name: /Next/i }));

    // Step 4
    expect(await screen.findByText('Product details')).toBeInTheDocument();
    await user.type(screen.getByPlaceholderText('What is the name of this product?'), 'Cake');
    await user.type(screen.getByPlaceholderText('0.00'), '20');
    await user.click(screen.getByRole('button', { name: /Next/i }));

    // Step 5
    expect(await screen.findByText('How do you want to receive payments?')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /Online/i }));

    // Step 6
    expect(await screen.findByText('Create your account')).toBeInTheDocument();
    await user.type(screen.getByPlaceholderText('e.g. Maya Smith'), 'Maya User');
    await user.type(screen.getByPlaceholderText('you@email.com'), 'maya@example.com');
    await user.type(screen.getByPlaceholderText('Password'), 'mypassword123');
    await user.click(screen.getByRole('button', { name: /Next/i }));

    // Step 7
    expect(await screen.findByText('Template selection')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /Modern/i }));
    await user.click(screen.getByRole('button', { name: /Next/i }));

    // Step 8
    expect(await screen.findByText('Choose your domain')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /Free OHC Domain/i }));
    await user.click(screen.getByRole('button', { name: /Next/i }));

    // Step 9
    expect(await screen.findByText('Review your choices')).toBeInTheDocument();

    // Publish
    await user.click(screen.getByRole('button', { name: /Publish my business/i }));

    // Expect success UI
    await waitFor(() => {
      expect(screen.getByText("Success! Your business is live!")).toBeInTheDocument();
    }, { timeout: 3000 });
  });

  it('Save Draft button triggers draft API and shows success message', async () => {
    const user = userEvent.setup({ delay: null });

    // Mock draft API success
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/state') {
        return Promise.resolve({
          ok: true,
          json: async () => ({})
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) });
    });

    render(<WalkthroughProvider><OnboardingWizard /></WalkthroughProvider>);

    // We must wait for the initial render properly if state changes or just click
    await waitFor(() => {
        expect(screen.getByRole('button', { name: /Start My Business/i })).toBeInTheDocument();
    });

    await user.click(screen.getByRole('button', { name: /Start My Business/i }));

    const saveDraftButton = await screen.findByRole('button', { name: /Save Draft/i });
    expect(saveDraftButton).toBeInTheDocument();

    await user.click(saveDraftButton);

    await waitFor(() => {
      expect(screen.getByText('Draft Saved!')).toBeInTheDocument();
    });

    // Verify API was called
    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/state', expect.objectContaining({
      method: 'POST'
    }));
  });
});
