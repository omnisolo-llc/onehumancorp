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
      businessType: '',
      businessName: '',
      businessCategory: '',
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
          businessType: 'Bakery',
          businessName: 'My Bakery'
        })
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({})
      });

    render(<OnboardingWizard />);

    await waitFor(() => {
      expect(useOnboardingStore.getState().step).toBe(2);
      expect(useOnboardingStore.getState().businessType).toBe('Bakery');
      expect(useOnboardingStore.getState().businessName).toBe('My Bakery');
    });
  });

  it('navigates through steps 1 to 5 successfully', async () => {
    (global.fetch as any).mockImplementation((url: string, options: any) => {
      if (url === '/api/onboarding/state') {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      if (url === '/api/onboarding/intake') {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            business_type: "Bakery",
            business_name: "My Bakery",
            categories: ["Cakes"],
            initial_products: [{ name: "Custom Cake", price: "50.00" }]
          })
        });
      }
      if (url === '/api/onboarding/start') {
        return Promise.resolve({
          ok: true,
          json: async () => ({ message: "Success!" })
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    render(<OnboardingWizard />);

    // Step 1
    await waitFor(() => expect(screen.getByText('What do you do?')).toBeDefined());
    const input1 = screen.getByPlaceholderText('e.g. Sell cakes, plumbing');
    fireEvent.change(input1, { target: { value: 'Bakery' } });
    fireEvent.click(screen.getByText('Next'));

    // Step 2
    await waitFor(() => expect(screen.getByText("What's the name of your business?")).toBeDefined());
    const input2 = screen.getByPlaceholderText("e.g. Maya's Cakes");
    fireEvent.change(input2, { target: { value: 'My Bakery' } });
    fireEvent.click(screen.getByText('Next'));

    // Step 3
    await waitFor(() => expect(screen.getByText("What's your niche?")).toBeDefined());
    const input3 = screen.getByPlaceholderText("e.g. I bake custom wedding cakes");
    fireEvent.change(input3, { target: { value: 'Wedding cakes' } });
    fireEvent.click(screen.getByText('Generate Draft'));

    // Step 4
    await waitFor(() => expect(screen.getByText('Ready to Launch!')).toBeDefined());

    // Interact with template and domain
    fireEvent.click(screen.getByText('Elegant'));
    fireEvent.click(screen.getByText('Connect Custom Domain'));

    // Click publish
    fireEvent.click(screen.getByText('Publish Now'));

    // Step 5
    await waitFor(() => expect(screen.getByText("You're Live!")).toBeDefined());
    expect(screen.getByText('Success!')).toBeDefined();

    expect(useOnboardingStore.getState().step).toBe(5);
  });
});
