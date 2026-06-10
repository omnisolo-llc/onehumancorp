import { render, screen, act } from '@testing-library/react';
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
      step: 0,
      bio: '',
      businessDescription: '',
      businessName: '',
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
    });

    // Mock fetch for all calls
    global.fetch = vi.fn().mockImplementation(async (url) => {
      if (url === '/api/onboarding/state') {
        return {
          ok: true,
          json: async () => ({
            wizardState: {
              step: 0,
            }
          })
        };
      }
      return { ok: true, json: async () => ({}) };
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the initial step 0 zero-click prompt', async () => {
    await renderOnboardingWizard();
    expect(screen.getByText('What do you do?')).toBeInTheDocument();
    expect(screen.getByPlaceholderText(/I'm a plumber in Miami.../i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Generate My Business/i })).toBeInTheDocument();
  });

  it('handles zero-click input and proceeds to loading state', async () => {
    const user = userEvent.setup();
    await renderOnboardingWizard();

    const bioInput = screen.getByPlaceholderText(/I'm a plumber in Miami.../i);
    await user.type(bioInput, 'I run a bakery');

    expect(bioInput).toHaveValue('I run a bakery');

    const generateBtn = screen.getByRole('button', { name: /Generate My Business/i });
    expect(generateBtn).not.toBeDisabled();

    // Re-mock fetch to handle the intake/start workflow
    global.fetch = vi.fn().mockImplementation(async (url) => {
      if (url === '/api/onboarding/intake') {
        return {
          ok: true,
          json: async () => ({
            business_name: 'My Bakery',
            business_type: 'Bakery',
            initial_products: [{ name: 'Cake', price: '10' }],
            location: 'Miami'
          })
        };
      }
      if (url === '/api/onboarding/start') {
        return {
          ok: true,
          json: async () => ({
            organization_id: 'org_123',
            message: 'Success'
          })
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    // Mock the setStep internally by just expecting it reaches "Agents are building..."
    await user.click(generateBtn);

    // Expect the state to change eventually (loading screen -> success)
    expect(await screen.findByText(/You're Live!/i, {}, { timeout: 3000 })).toBeInTheDocument();
  });

  it('disables submit button when bio is empty', async () => {
    await renderOnboardingWizard();
    const generateBtn = screen.getByRole('button', { name: /Generate My Business/i });
    expect(generateBtn).toBeDisabled();
  });

  it('shows error state if intake API fails', async () => {
    const user = userEvent.setup();
    await renderOnboardingWizard();

    const bioInput = screen.getByPlaceholderText(/I'm a plumber in Miami.../i);
    await user.type(bioInput, 'I run a bakery');

    global.fetch = vi.fn().mockImplementation(async (url) => {
      if (url === '/api/onboarding/intake') {
        return {
          ok: false,
          json: async () => ({
            error: 'Intake failed'
          })
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    const generateBtn = screen.getByRole('button', { name: /Generate My Business/i });
    await user.click(generateBtn);

    // Eventually shows error
    expect(await screen.findByText('Intake failed')).toBeInTheDocument();
  });

  it('renders success screen on step 5', async () => {
    useOnboardingStore.setState({
      step: 5,
      businessName: 'My Bakery',
      startResult: {
        message: 'Your business has been successfully launched.'
      }
    });

    await renderOnboardingWizard();

    expect(screen.getByText("You're Live!")).toBeInTheDocument();
    expect(screen.getByText("Your business has been successfully launched.")).toBeInTheDocument();
    expect(screen.getByText(/my-bakery.ohc.app/i)).toBeInTheDocument();
    expect(screen.getByRole('link', { name: /Publish & Share Link/i })).toBeInTheDocument();
    expect(screen.getByRole('link', { name: /Preview Storefront/i })).toBeInTheDocument();
  });
});
