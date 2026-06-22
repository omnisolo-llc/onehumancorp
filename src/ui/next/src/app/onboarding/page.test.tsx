/* @vitest-environment jsdom */
import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { TooltipProvider } from '../../components/TooltipRegistry';
import { beforeEach, describe, it, expect, vi, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';

const mockRouterPush = vi.hoisted(() => vi.fn());

vi.mock('next/navigation', () => ({
  usePathname: () => '/onboarding',
  useRouter: () => ({
    push: mockRouterPush,
    replace: vi.fn(),
    prefetch: vi.fn(),
    back: vi.fn(),
    forward: vi.fn(),
    refresh: vi.fn(),
  }),
}));

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
    mockRouterPush.mockClear();
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
        return Promise.resolve({ ok: true, json: async () => ({ wizardState: { bio: "Draft Bio" } }) });
    }) as any;
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders conversational initial screen correctly', async () => {
    act(() => {
      useOnboardingStore.setState({ step: 0 });
    });
    await renderOnboardingWizard();

    expect(screen.getByText("Hi, I'm your OHC assistant.")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Build my business/i });
    expect(button).toBeInTheDocument();
  });

  it('Handles single conversational input flow', async () => {
    const user = userEvent.setup({ delay: null });

    // Mock intake success
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/start') {
        return Promise.resolve({
          ok: true,
          json: async () => ({ organization_id: 'org_123', status: 'started' })
        });
      }
      if (url === '/api/onboarding/launch') { return Promise.resolve({ ok: true, json: async () => ({}) }); }
      if (url === '/api/onboarding/intake') {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            business_type: 'Plumbing',
            business_name: 'Carlos Plumbing',
            categories: ['service'],
            initial_products: [{ name: 'Plumbing Repair', price: '100' }]
          })
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: { bio: "" } }) });
    });

    act(() => {
      useOnboardingStore.setState({ step: 0 });
    });
    await renderOnboardingWizard();

    const bioInput = screen.getByPlaceholderText(/I'm Carlos, I fix plumbing/i);
    await user.type(bioInput, "I'm Carlos, I fix plumbing");

    const submitBtn = screen.getByRole('button', { name: /Build my business/i });
    await user.click(submitBtn);

    // Should transition to step 4 processing, then to step 5 completion
    await waitFor(() => {
      expect(screen.getByText("Building Your Business...")).toBeInTheDocument();
    });
  });

  it('Handles validation failures when fields are empty', async () => {
    act(() => {
        useOnboardingStore.setState({ step: 0, bio: ' ' });
    });
    const user = userEvent.setup({ delay: null });
    await renderOnboardingWizard();

    const bioInput = screen.getByPlaceholderText(/I'm Carlos, I fix plumbing/i);
    await user.clear(bioInput);

    const button = screen.getByRole('button', { name: /Build my business/i });
    expect(button).toBeDisabled();
  });
  it('Handles Save Draft button functionality', async () => {
    const user = userEvent.setup({ delay: null });

    // Mock the draft save success
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/onboarding/draft') { return Promise.resolve({ ok: true, json: async () => ({}) }); }
      return Promise.resolve({ ok: true, json: async () => ({ wizardState: { bio: "Draft Bio" } }) });
    });

    await renderOnboardingWizard();

    // Chat Step 1
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, 'Draft Bakery');

    // Proceed to Step 2
    const nextBtn = screen.getByRole('button', { name: /Next/i });
    await user.click(nextBtn);

    // On step 2, wait for "What do you sell" or another input indicating step 2 is active
    await screen.findByText(/What do you sell\?/i);

    // Try finding the url inputs
    const urlInputs = screen.queryAllByPlaceholderText(/Image URL \(Optional\)/i);
    if (urlInputs.length > 0) {
      const urlInput = urlInputs.find(el => el.id === 'instant-image-url') || urlInputs[0];
      await user.type(urlInput, 'https://example.com/save_draft.png');
    } else {
        // Fallback to chat input or another state if URL isn't here
        // We know instantImageUrl is in state, so we update it directly to test draft save
        act(() => {
            useOnboardingStore.setState({ instantImageUrl: 'https://example.com/save_draft.png' });
        });
    }

    // Click Save Draft
    const saveDraftBtn = screen.getByRole('button', { name: /Save Draft/i });
    await user.click(saveDraftBtn);

    // Verify it saved
    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/draft', expect.objectContaining({
        method: 'POST',
        body: expect.stringContaining('https://example.com/save_draft.png')
    }));
    await waitFor(() => {
      expect(screen.getByText('Draft Saved!')).toBeInTheDocument();
    });
  });

  it('allows skipping setup and opens the assistant', async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({ step: 0 });
    });

    await renderOnboardingWizard();

    const skipButton = screen.getByRole('button', { name: /Skip setup/i });
    await user.click(skipButton);

    expect(localStorage.getItem('has_onboarded')).toBe('true');
    expect(mockRouterPush).toHaveBeenCalledWith('/dashboard');
  });

  it('offers a global back control on later wizard steps', async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 3,
        businessName: 'Maya Bakery',
        businessType: 'Bakery',
        firstProductName: 'Cake',
        firstProductPrice: '20',
      });
    });

    await renderOnboardingWizard();

    expect(screen.getByText('Style & Team')).toBeInTheDocument();

    // Get all back buttons and take the visible one
    const backButton = screen.getAllByRole('button', { name: /Back/i })[0];
    await user.click(backButton);

    expect(screen.getByText('Review Details')).toBeInTheDocument();
    expect(useOnboardingStore.getState().step).toBe(2);
  });

  it('can go back from the first question to the intro screen', async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({ step: 1, chatStep: 1 });
    });

    await renderOnboardingWizard();

    expect(screen.getByText("What's the name of your business?")).toBeInTheDocument();

    // Get all back buttons and take the visible one
    const backButton = screen.getAllByRole('button', { name: /Back/i })[0];
    await user.click(backButton);

    expect(screen.getByText("Hi, I'm your OHC assistant.")).toBeInTheDocument();
    expect(useOnboardingStore.getState().step).toBe(0);
  });

  it('Step 3: Passes initial_products from localStorage to /api/onboarding/start', async () => {
    const user = userEvent.setup({ delay: null });

    localStorage.setItem('onboarding_initial_products', JSON.stringify([
      { name: 'Custom AI Product', price: '99' }
    ]));

    act(() => {
      useOnboardingStore.setState({ step: 3, adminName: "Test Admin", adminEmail: "test@example.com", adminPassword: "Password123" });
    });

    let startRequestPayload: any = null;
    (global.fetch as any).mockImplementation((url: string, options: any) => {
      if (url === '/api/onboarding/start') {
        startRequestPayload = JSON.parse(options.body);
        return Promise.resolve({
          ok: true,
          json: async () => ({ organization_id: 'org_123', status: 'started' })
        });
      }
      if (url === '/api/onboarding/launch') { return Promise.resolve({ ok: true, json: async () => ({}) }); }
      if (url === '/api/onboarding/state') { return Promise.resolve({ ok: true, json: async () => ({}) }); }
      if (url === '/api/onboarding/draft') { return Promise.resolve({ ok: true, json: async () => ({}) }); }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    render(<TooltipProvider><OnboardingWizard /></TooltipProvider>);

    const launchButton = await screen.findByRole('button', { name: /Approve & Go Live/i });
    await user.click(launchButton);

    expect(startRequestPayload).toBeDefined();
    expect(startRequestPayload.initial_products).toEqual([{ name: 'Custom AI Product', price: '99' }]);
  });
});
