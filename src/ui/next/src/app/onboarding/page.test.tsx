import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';
import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock the store
vi.mock('./store', () => ({
  useOnboardingStore: vi.fn(),
}));

describe('OnboardingWizard', () => {
  let mockStore: any;

  beforeEach(() => {
    mockStore = {
      step: 1,
      setStep: vi.fn(),
      businessType: '',
      setBusinessType: vi.fn(),
      businessName: '',
      setBusinessName: vi.fn(),
      businessCategory: '',
      setBusinessCategory: vi.fn(),
      isLoading: false,
      setIsLoading: vi.fn(),
      error: '',
      setError: vi.fn(),
      intakeData: null,
      setIntakeData: vi.fn(),
      startResult: null,
      setStartResult: vi.fn(),
      loadStateFromBackend: vi.fn().mockResolvedValue(undefined),
    };
    (useOnboardingStore as any).mockReturnValue(mockStore);
  });

  it('calls loadStateFromBackend on mount', async () => {
    render(<OnboardingWizard />);
    expect(mockStore.loadStateFromBackend).toHaveBeenCalled();
  });

  it('shows loading spinner while initializing', () => {
    // The component defaults to isInitializing = true
    render(<OnboardingWizard />);
    expect(document.querySelector('.animate-spin')).toBeInTheDocument();
  });

  it('clears error when typing business name', async () => {
    mockStore.error = "Please enter your business name.";
    mockStore.step = 2;
    // We need to re-render to reflect the error state, but the mock is set up.
    // However, the component will be in initializing state until loadStateFromBackend resolves.

    const { rerender } = render(<OnboardingWizard />);

    // Wait for the spinner to disappear
    await waitFor(() => {
      expect(document.querySelector('.animate-spin')).not.toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText("e.g. Maya's Cakes");
    fireEvent.change(input, { target: { value: 'New Name' } });

    expect(mockStore.setBusinessName).toHaveBeenCalledWith('New Name');
    expect(mockStore.setError).toHaveBeenCalledWith('');
  });

  it('clears error when typing business category', async () => {
    mockStore.error = "Please describe your niche.";
    mockStore.step = 3;

    render(<OnboardingWizard />);

    // Wait for the spinner to disappear
    await waitFor(() => {
      expect(document.querySelector('.animate-spin')).not.toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText("e.g. I bake custom wedding cakes");
    fireEvent.change(input, { target: { value: 'New Category' } });

    expect(mockStore.setBusinessCategory).toHaveBeenCalledWith('New Category');
    expect(mockStore.setError).toHaveBeenCalledWith('');
  });
});
