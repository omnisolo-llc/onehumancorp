import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { TooltipProvider } from '../../components/TooltipRegistry';
import { beforeEach, describe, it, expect, vi, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';
import React from 'react';

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
      step: 0,
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


  it('Instant Build: Successfully processes bio and admin details to create storefront', async () => {
    const user = userEvent.setup({ delay: null });

    let startPayload: any = null;
    let intakePayload: any = null;

    (global.fetch as any).mockImplementation((url: string, options: any) => {
      if (url === '/api/onboarding/intake') {
        intakePayload = JSON.parse(options.body);
        return Promise.resolve({
          ok: true,
          json: async () => ({
            business_name: 'Instant Bakery',
            business_type: 'Bakery',
            categories: ['food'],
            location: 'Local',
            target_audience: 'Everyone',
            initial_products: [{ name: 'Instant Cake', price: '25.00' }]
          })
        });
      }
      if (url === '/api/onboarding/start') {
        startPayload = JSON.parse(options.body);
        return Promise.resolve({
          ok: true,
          json: async () => ({ organization_id: 'org_instant', status: 'started' })
        });
      }
      if (url === '/api/onboarding/launch') { return Promise.resolve({ ok: true, json: async () => ({}) }); }
      if (url === '/api/onboarding/state') { return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) }); }
      if (url === '/api/onboarding/draft') { return Promise.resolve({ ok: true, json: async () => ({ wizardState: {} }) }); }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    await renderOnboardingWizard();

    // Fill bio
    const bioInput = screen.getByTestId('instant-bio');
    await user.type(bioInput, 'Test Business Description');

    // Fill admin credentials
    const adminEmailInput = screen.getByTestId('admin-email');
    await user.type(adminEmailInput, 'admin@test.com');
    const adminPasswordInput = screen.getByTestId('admin-password');
    await user.type(adminPasswordInput, 'password123');

    const nextButton = screen.getByRole('button', { name: /Next/i });
    expect(nextButton).not.toBeDisabled();
    await user.click(nextButton);

    await waitFor(() => {
      expect(intakePayload).toBeDefined();
    });
    if(intakePayload) { expect(intakePayload.description).toContain('Test Business Description'); }

    await waitFor(() => {
      expect(startPayload).toBeDefined();
    });
    if(startPayload) { expect(startPayload.admin_email).toBe('admin@test.com'); }
    if(startPayload) { expect(startPayload.admin_password).toBe('password123'); }


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
});
