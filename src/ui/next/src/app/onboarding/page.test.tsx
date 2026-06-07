import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';
import OnboardingWizard from './page';

// We do NOT mock useOnboardingStore! Zustand stores work perfectly in React Testing Library if we reset them before each test.
import { useOnboardingStore } from './store';

const initialState = useOnboardingStore.getState();

describe('OnboardingWizard', () => {
  beforeEach(() => {
    useOnboardingStore.setState(initialState, true);

    vi.clearAllMocks();
    global.fetch = vi.fn();
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({})
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the welcome screen initially', async () => {
    await act(async () => {
      render(<OnboardingWizard />);
    });

    expect(screen.getByText('Welcome')).toBeInTheDocument();
  });

  it('progresses to single input screen and allows submission', async () => {
    const user = userEvent.setup();

    await act(async () => {
      render(<OnboardingWizard />);
    });

    const startButton = await screen.findByText(/Start Onboarding/i);
    await user.click(startButton);

    expect(await screen.findByText('Describe your business in one sentence')).toBeInTheDocument();

    const input = await screen.findByPlaceholderText(/e.g. I bake custom vegan cakes/i);
    await user.type(input, 'I sell vegan cookies in San Francisco');

    const generateButton = await screen.findByText(/Generate Storefront/i);
    expect(generateButton).toBeInTheDocument();

    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({
        business_type: 'Bakery',
        business_name: 'Vegan Cookies SF',
        categories: ['food'],
        initial_products: [{ name: 'Choc Chip', price: '5.00' }]
      })
    });

    await user.click(generateButton);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/intake', expect.any(Object));
    });
  });

  it('shows error if input is too short', async () => {
    const user = userEvent.setup();
    await act(async () => {
      render(<OnboardingWizard />);
    });
    const startButton = await screen.findByText(/Start Onboarding/i);
    await user.click(startButton);

    const input = await screen.findByPlaceholderText(/e.g. I bake custom vegan cakes/i);
    await user.type(input, 'short');

    const generateButton = await screen.findByText(/Generate Storefront/i);
    await user.click(generateButton);

    expect(await screen.findByText('Please provide a bit more detail about your business.')).toBeInTheDocument();
  });

  it('handles API errors gracefully', async () => {
    const user = userEvent.setup();
    await act(async () => {
      render(<OnboardingWizard />);
    });
    const startButton = await screen.findByText(/Start Onboarding/i);
    await user.click(startButton);

    const input = await screen.findByPlaceholderText(/e.g. I bake custom vegan cakes/i);
    await user.type(input, 'I sell vegan cookies in San Francisco');

    const generateButton = await screen.findByText(/Generate Storefront/i);

    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      json: async () => ({ message: 'Failed to generate' })
    });

    await user.click(generateButton);

    expect(await screen.findByText('Failed to generate')).toBeInTheDocument();
  });
});
