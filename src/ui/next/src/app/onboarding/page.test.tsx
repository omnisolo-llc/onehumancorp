import React, { act } from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { TooltipProvider } from '@radix-ui/react-tooltip';

const mockRouterPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockRouterPush }),
}));

describe('OnboardingWizard Chat Interface', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    act(() => {
      useOnboardingStore.setState({
        step: 0,
        chatStep: 1,
        isLoading: false,
        error: '',
        businessName: '',
        chatMessages: [],
      });
    });
    localStorage.clear();

    // Mock smooth scroll
    window.HTMLElement.prototype.scrollIntoView = vi.fn();
  });

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

  it('Displays the chat UI at step 0', async () => {
    await renderOnboardingWizard();
    expect(screen.getByText('What do you want to build or manage today?')).toBeInTheDocument();

    // Check chips
    expect(screen.getByText('Cake Shop')).toBeInTheDocument();
    expect(screen.getByText('Handyman')).toBeInTheDocument();
  });

  it('allows skipping setup and opens the assistant', async () => {
    const user = userEvent.setup({ delay: null });

    await renderOnboardingWizard();

    const skipButton = screen.getByRole('button', { name: /Skip setup/i });
    await user.click(skipButton);

    expect(localStorage.getItem('has_onboarded')).toBe('true');
    expect(mockRouterPush).toHaveBeenCalledWith('/dashboard');
  });
});
