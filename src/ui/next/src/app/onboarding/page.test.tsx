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
