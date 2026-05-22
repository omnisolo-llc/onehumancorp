import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';

// Mock the store
vi.mock('./store', () => {
  return {
    useOnboardingStore: vi.fn(),
  };
});

describe('OnboardingWizard', () => {
  let mockStore: any;

  beforeEach(() => {
    mockStore = {
      step: 1,
      setStep: vi.fn(),
      businessName: '',
      setBusinessName: vi.fn(),
      businessCategory: '',
      setBusinessCategory: vi.fn(),
      isInstantBuild: false,
      setIsInstantBuild: vi.fn(),
      businessDescription: '',
      setBusinessDescription: vi.fn(),
      isLoading: false,
      setIsLoading: vi.fn(),
      error: '',
      setError: vi.fn(),
      intakeData: null,
      setIntakeData: vi.fn(),
      startResult: null,
      setStartResult: vi.fn(),
    };
    (useOnboardingStore as any).mockReturnValue(mockStore);
    global.fetch = vi.fn();
  });

  it('renders Step 1 correctly', () => {
    render(<OnboardingWizard />);
    expect(screen.getByText("What's the name of your business?")).toBeInTheDocument();
  });

  it('handles Next button in Step 1 correctly when name is empty', () => {
    render(<OnboardingWizard />);
    fireEvent.click(screen.getByText('Next'));
    expect(mockStore.setError).toHaveBeenCalledWith("Please enter your business name.");
  });

  it('handles Next button in Step 1 correctly when name is provided', () => {
    mockStore.businessName = 'Test Name';
    render(<OnboardingWizard />);
    fireEvent.click(screen.getByText('Next'));
    expect(mockStore.setError).toHaveBeenCalledWith("");
    expect(mockStore.setStep).toHaveBeenCalledWith(2);
  });

  it('toggles Instant Build mode correctly', () => {
    render(<OnboardingWizard />);
    const toggle = screen.getByRole('switch');
    fireEvent.click(toggle);
    expect(mockStore.setIsInstantBuild).toHaveBeenCalledWith(true);
  });

  it('handles Generate Draft button in Instant Build mode correctly when description is empty', () => {
    mockStore.isInstantBuild = true;
    render(<OnboardingWizard />);
    fireEvent.click(screen.getByText('Generate Draft'));
    expect(mockStore.setError).toHaveBeenCalledWith("Please describe your business.");
  });

  it('handles Generate Draft button in Instant Build mode successfully', async () => {
    mockStore.isInstantBuild = true;
    mockStore.businessDescription = 'Test Description';
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ business_name: 'Test Name' }),
    });

    render(<OnboardingWizard />);
    fireEvent.click(screen.getByText('Generate Draft'));

    await waitFor(() => {
      expect(mockStore.setIsLoading).toHaveBeenCalledWith(true);
      expect(mockStore.setIntakeData).toHaveBeenCalledWith({ business_name: 'Test Name' });
      expect(mockStore.setStep).toHaveBeenCalledWith(3);
    });
  });
});
