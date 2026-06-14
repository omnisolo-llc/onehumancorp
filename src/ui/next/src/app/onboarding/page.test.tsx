import { render, screen, act } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { vi } from 'vitest';

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

// Mock next/navigation
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('OnboardingWizard', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    useOnboardingStore.setState({
      step: 0,
      chatStep: 1,
      bio: '',
      businessDescription: '',
      businessGoal: '',
      businessName: '',
      whatYouSell: '',
      location: '',
      targetAudience: '',
      businessType: 'Online Store',
      categories: [],
      websiteTemplate: 'Modern',
      domainChoice: 'subdomain',
      firstProductName: '',
      firstProductPrice: '',
      adminName: '',
      adminEmail: '',
      adminPassword: '',
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: '',
      startResult: null,
      instantImageUrl: '',
    });
    localStorage.clear();
    global.fetch = vi.fn((url) => {
        if (url === '/api/onboarding/state') {
            return Promise.resolve({
                ok: true,
                json: () => Promise.resolve({})
            });
        }
        if (url.includes('/api/onboarding/draft')) {
             return Promise.resolve({
                 ok: true,
                 json: () => Promise.resolve({})
             });
        }
        return Promise.resolve({
            ok: true,
            json: () => Promise.resolve({})
        });
    }) as any;
  });

  const renderOnboardingWizard = async () => {
    let res;
    await act(async () => {
      res = render(<OnboardingWizard />);
      // Wait for initial state load and fetch to complete
      await new Promise(resolve => setTimeout(resolve, 0));
    });
    return res;
  };

  it('renders chat interface initially', async () => {
    await renderOnboardingWizard();
    expect(screen.getByText(/What kind of business are you starting/i)).toBeInTheDocument();
  });

  it('can send a chat message', async () => {
    const user = userEvent.setup();
    await renderOnboardingWizard();

    (global.fetch as any).mockImplementationOnce((url: string) => {
        if (url.includes('/api/onboarding/draft')) {
             return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
        }
        return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
    });

    (global.fetch as any).mockImplementation((url: string) => {
        if (url.includes('/api/onboarding/chat')) {
             return Promise.resolve({
                 ok: true,
                 json: () => Promise.resolve({
                     is_complete: false,
                     reply: 'Cool! Can you tell me more about your cakes?'
                 })
             });
        }
        return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
    });

    const input = screen.getByPlaceholderText(/I sell custom wedding cakes/i);
    await user.type(input, 'I sell cupcakes');

    const sendButton = screen.getByRole('button');
    await user.click(sendButton);

    expect(screen.getByText('I sell cupcakes')).toBeInTheDocument();
  });

it('shows account setup step and launches', async () => {
    const user = userEvent.setup();
    await renderOnboardingWizard();

    (global.fetch as any).mockImplementation((url: string) => {
        if (url.includes('/api/onboarding/chat')) {
             return Promise.resolve({
                 ok: true,
                 json: () => Promise.resolve({
                     is_complete: true,
                     reply: 'Got it! Give me a minute...',
                     intake_data: {
                       business_name: 'Test Business',
                       business_type: 'Bakery',
                       categories: ['physical']
                     }
                 })
             });
        }
        if (url.includes('/api/onboarding/start')) {
             return Promise.resolve({
                 ok: true,
                 json: () => Promise.resolve({ organization_id: 'org_123' })
             });
        }
        if (url.includes('/api/onboarding/state') || url.includes('/api/onboarding/launch') || url.includes('/api/onboarding/draft')) {
             return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
        }
        return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
    });

    const input = screen.getByPlaceholderText(/I sell custom wedding cakes/i);
    await user.type(input, 'I sell cakes');

    const sendButton = screen.getByRole('button');
    await user.click(sendButton);

    // Expect to be on the secure step
    const emailInput = await screen.findByPlaceholderText(/you@example.com/i);
    const passInput = await screen.findByPlaceholderText(/Create a password/i);
    const launchButton = screen.getByRole('button', { name: /Launch Store/i });

    await user.type(emailInput, 'test@example.com');
    await user.type(passInput, 'password123');
    await user.click(launchButton);

    // Expect to hit success step
    expect(await screen.findByText(/You're Live!/i)).toBeInTheDocument();
  });
});
