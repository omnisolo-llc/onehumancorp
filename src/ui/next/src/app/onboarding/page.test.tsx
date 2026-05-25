import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import OnboardingWizard from './page';
import { useOnboardingStore } from './store';

global.fetch = vi.fn();

describe('OnboardingWizard', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    localStorage.clear();
    useOnboardingStore.setState({
      step: 1,
      businessBio: '',
      firstProductName: '',
      firstProductPrice: '',
      template: 'Modern',
      domain: 'free',
      isLoading: false,
      error: '',
      intakeData: null,
      startResult: null,
    });
  });

  it('loads state from backend on mount if data.step >= step', async () => {
    (global.fetch as any)
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          step: 2,
          businessBio: 'My Bakery sells cakes'
        })
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({})
      });

    render(<OnboardingWizard />);

    await waitFor(() => {
      expect(useOnboardingStore.getState().step).toBe(2);
      expect(useOnboardingStore.getState().businessBio).toBe('My Bakery sells cakes');
    });
  });

  it('updates state when textarea is changed and calls intake api', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({}) // initial state fetch
    });

    render(<OnboardingWizard />);

    const textarea = screen.getByPlaceholderText(/e\.g\. My name is Maya/i);
    fireEvent.change(textarea, { target: { value: 'This is a long enough description of my business' } });

    expect(useOnboardingStore.getState().businessBio).toBe('This is a long enough description of my business');

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ business_type: 'Test', business_name: 'Test Name', categories: [], initial_products: [] }) // intake fetch
    });

    const button = screen.getByText('Generate Draft');
    fireEvent.click(button);

    await waitFor(() => {
      expect(useOnboardingStore.getState().step).toBe(2);
      expect(useOnboardingStore.getState().intakeData).not.toBeNull();
    });
  });

  it('shows error if description is too short', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({}) // initial state fetch
    });

    render(<OnboardingWizard />);
    const textarea = screen.getByPlaceholderText(/e\.g\. My name is Maya/i);
    fireEvent.change(textarea, { target: { value: 'short' } });
    const button = screen.getByText('Generate Draft');
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByText('Please provide a bit more detail (at least 10 characters).')).toBeDefined();
    });
  });
});
