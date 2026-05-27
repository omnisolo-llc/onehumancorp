import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { beforeEach, describe, it, expect, vi, afterEach } from 'vitest';
import { useRouter } from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn()
}));

describe('OnboardingWizard', () => {
  const mockPush = vi.fn();

  beforeEach(() => {
    localStorage.clear();
    useOnboardingStore.setState({
      step: 1,
      businessDescription: '',
      isLoading: false,
      error: '',
      startResult: null,
    });

    (useRouter as any).mockReturnValue({ push: mockPush });
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('Step 1: Renders initial screen correctly', async () => {
    act(() => { render(<OnboardingWizard />); });

    expect(screen.getByText("Let's build your business.")).toBeInTheDocument();
    const button = screen.getByRole('button', { name: /Start Now/i });
    expect(button).not.toBeDisabled();
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock intake success
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        business_type: 'Bakery',
        business_name: 'Maya Bakery',
        categories: ['food'],
        initial_products: [{ name: 'Cake', price: '20' }]
      })
    }).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ message: "Success!" })
    });

    act(() => { render(<OnboardingWizard />); });

    // Go to step 2
    const startButton = screen.getByRole('button', { name: /Start Now/i });
    await act(async () => {
      startButton.click();
    });

    await waitFor(() => {
      expect(screen.getByText("Tell me what you sell in one sentence.")).toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(input, 'I am a baker in NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });
    expect(button).not.toBeDisabled();

    await act(async () => {
      button.click();
    });

    // Verify it shows loading screen (step 3)
    await waitFor(() => {
      expect(screen.getByText("Building Your Business...")).toBeInTheDocument();
    });

    // Verify redirect
    await waitFor(() => {
      expect(mockPush).toHaveBeenCalledWith('/dashboard');
    });
  });

  it('Handles API failure', async () => {
    const userEvent = (await import('@testing-library/user-event')).default.setup({ delay: null });

    // Mock intake failure
    (global.fetch as any).mockResolvedValueOnce({
      ok: false
    });

    act(() => { render(<OnboardingWizard />); });

    // Go to step 2
    const startButton = screen.getByRole('button', { name: /Start Now/i });
    await act(async () => {
      startButton.click();
    });

    const input = screen.getByPlaceholderText(/I bake custom vegan cakes/i);
    await userEvent.type(input, 'I am a baker in NY');

    const button = screen.getByRole('button', { name: /Generate My Business/i });

    await act(async () => {
      button.click();
    });

    // Verify error appears and step goes back to 2
    await waitFor(() => {
      expect(screen.getByText("Failed to process business details")).toBeInTheDocument();
      expect(screen.getByText("Tell me what you sell in one sentence.")).toBeInTheDocument();
    });
  });
});
