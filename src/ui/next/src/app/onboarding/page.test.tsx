import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { beforeEach, describe, it, expect, vi, afterEach } from 'vitest';

// Mock next/navigation
const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter() {
    return {
      push: mockPush,
      replace: vi.fn(),
      prefetch: vi.fn(),
    };
  },
}));

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
      businessType: 'Online Store',
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: '',
      startResult: null,
    });

    global.fetch = vi.fn((url) => {
      return Promise.resolve({ ok: true, json: async () => ({ message: "Success!" }) });
    }) as any;
    mockPush.mockClear();
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial screen correctly', async () => {
    act(() => { render(<OnboardingWizard />); });

    expect(screen.getByText("Welcome to OHC")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Start a Business/i });
    expect(button).toBeInTheDocument();
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    act(() => { render(<OnboardingWizard />); });

    // Step 1: Start
    const startBtn = screen.getByRole('button', { name: /Start a Business/i });
    await act(async () => { startBtn.click(); });

    // Step 2: Business Name
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await userEvent.type(nameInput, 'Maya Bakery');

    const nextBtn1 = screen.getByRole('button', { name: /Next/i });
    await act(async () => { nextBtn1.click(); });

    // Step 3: Business Type
    expect(screen.getByText("Business Type")).toBeInTheDocument();
    const typeOption = screen.getByText('Food');
    await act(async () => { typeOption.click(); });

    const launchButton = screen.getByRole('button', { name: /Launch Store/i });
    expect(launchButton).not.toBeDisabled();

    // Submit
    await act(async () => {
      launchButton.click();
    });

    // Verify it transitions to Step 4: Loading Screen, and then redirects
    await waitFor(() => {
      expect(screen.getByText("AI is building your storefront...")).toBeInTheDocument();
    });

    await waitFor(() => {
      expect(mockPush).toHaveBeenCalledWith('/dashboard');
    });
  });
});
