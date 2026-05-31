
import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import { beforeEach, describe, it, expect, vi, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';

// Mock fetch
const originalFetch = global.fetch;

beforeEach(() => {
  useOnboardingStore.setState({
    step: 1,
    businessType: '',
    uploadedPhotos: [],
    businessName: '',
    isLoading: false,
    error: '',
    startResult: null
  });

  global.fetch = vi.fn().mockImplementation((url) => {
    if (url === '/api/onboarding/state') {
      return Promise.resolve({
        json: () => Promise.resolve({ wizardState: { step: 1 } })
      });
    }
    if (url === '/api/onboarding/start') {
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ message: 'Success' })
      });
    }
    return Promise.resolve({
      json: () => Promise.resolve({})
    });
  });
});

afterEach(() => {
  global.fetch = originalFetch;
});

describe('OnboardingWizard', () => {
  it('Renders initial screen correctly', async () => {
    await act(async () => {
        render(<OnboardingWizard />);
    });
    expect(screen.getByText('What do you sell?')).toBeInTheDocument();
  });

  it('Handles multi-step successful onboarding flow', async () => {
    const user = userEvent.setup();
    render(<OnboardingWizard />);

    // Step 1
    const businessTypeBtn = screen.getByText('Food & Beverage');
    await user.click(businessTypeBtn);

    const nextBtn1 = screen.getByRole('button', { name: /Next/i });
    await user.click(nextBtn1);

    // Step 2
    expect(screen.getByText('Add some photos')).toBeInTheDocument();
    const skipBtn = screen.getByRole('button', { name: /Skip/i });
    await user.click(skipBtn);

    // Step 3
    expect(screen.getByText("What's the name of your business?")).toBeInTheDocument();
    const nameInput = screen.getByPlaceholderText(/e.g. Maya's Custom Cakes/i);
    await user.type(nameInput, 'Maya Cakes');

    const generateBtn = screen.getByRole('button', { name: /Generate My Business/i });
    await user.click(generateBtn);

    // Step 4 (Loading)
    expect(screen.getByText('Our AI is building your store...')).toBeInTheDocument();
  });
});
