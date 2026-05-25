import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import OnboardingWizard from './page';

// Mock Zustand store
vi.mock('./store', () => {
  let step = 1;
  let businessType = '';
  let businessName = '';
  let businessCategory = '';

  return {
    useOnboardingStore: vi.fn(() => ({
      step,
      setStep: vi.fn((newStep) => { step = newStep; }),
      businessType,
      setBusinessType: vi.fn((val) => { businessType = val; }),
      businessName,
      setBusinessName: vi.fn((val) => { businessName = val; }),
      businessCategory,
      setBusinessCategory: vi.fn((val) => { businessCategory = val; }),
      firstProductName: '',
      setFirstProductName: vi.fn(),
      firstProductPrice: '',
      setFirstProductPrice: vi.fn(),
      template: 'Modern',
      setTemplate: vi.fn(),
      domain: 'free',
      setDomain: vi.fn(),
      isLoading: false,
      setIsLoading: vi.fn(),
      error: '',
      setError: vi.fn(),
      intakeData: null,
      setIntakeData: vi.fn(),
      startResult: null,
      setStartResult: vi.fn()
    }))
  };
});

describe('OnboardingWizard', () => {
  beforeEach(() => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({}),
      })
    ) as any;
  });

  it('renders step 1 and validates immediate state persistence', async () => {
    render(<OnboardingWizard />);

    expect(screen.getByText('What do you do?')).toBeInTheDocument();

    const input = screen.getByPlaceholderText('e.g. Sell cakes, plumbing');
    fireEvent.change(input, { target: { value: 'Baking' } });

    // Test autoCapitalize is applied
    expect(input.getAttribute('autoCapitalize')).toBe('words');

    const nextBtn = screen.getByRole('button', { name: 'Next' });
    fireEvent.click(nextBtn);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/state', expect.objectContaining({
        headers: expect.objectContaining({ 'x-tenant-id': 'storefront' })
      }));
    });
  });
});
