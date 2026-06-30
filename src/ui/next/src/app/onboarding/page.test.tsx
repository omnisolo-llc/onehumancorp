import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import OnboardingChat from './page';

// Mock next/navigation
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('OnboardingChat', () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.runOnlyPendingTimers();
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it('renders initial chat screen', () => {
    render(<OnboardingChat />);
    expect(screen.getByText("Hi there! I'm your OHC Work Assistant. What kind of work do you do?")).toBeInTheDocument();
  });

  it('handles user input and shows approval card with real API', async () => {
    (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => ({
            intake_data: { business_name: 'My Cake Shop', initial_products: [{ name: 'Custom Cake' }] },
            reply: 'Got it! I drafted your storefront.'
        }),
        clone: () => ({ json: async () => ({}) })
    });

    render(<OnboardingChat />);

    const input = screen.getByPlaceholderText('Type your response...');
    fireEvent.change(input, { target: { value: 'I bake cakes' } });

    const sendButton = screen.getByRole('button');
    fireEvent.click(sendButton);

    expect(screen.getByText('I bake cakes')).toBeInTheDocument();

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/chat', expect.any(Object));
      expect(screen.getByText("Got it! I drafted your storefront.")).toBeInTheDocument();
      expect(screen.getByText('Approve & Go Live')).toBeInTheDocument();
    });
  });

  it('handles approve and transitions to live screen with real API calls', async () => {
    (global.fetch as any)
      .mockResolvedValueOnce({
          ok: true,
          json: async () => ({
              intake_data: { business_name: 'My Cake Shop', initial_products: [{ name: 'Custom Cake' }] },
              reply: 'Got it! I drafted your storefront.'
          }),
          clone: () => ({ json: async () => ({}) })
      })
      .mockResolvedValueOnce({
          ok: true,
          json: async () => ({ message: 'success', organization_id: 'org_123', user_id: 'user_123' }),
          clone: () => ({ json: async () => ({}) })
      })
      .mockResolvedValueOnce({
          ok: true,
          json: async () => ({ message: 'success', organization_id: 'org_123', user_id: 'user_123' }),
          clone: () => ({ json: async () => ({}) })
      });

    render(<OnboardingChat />);

    const input = screen.getByPlaceholderText('Type your response...');
    fireEvent.change(input, { target: { value: 'I bake cakes' } });

    const sendButton = screen.getByRole('button');
    fireEvent.click(sendButton);

    await waitFor(() => {
      expect(screen.getByText('Approve & Go Live')).toBeInTheDocument();
    });

    const approveButton = screen.getByText('Approve & Go Live');
    fireEvent.click(approveButton);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/start', expect.any(Object));
      expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/launch', expect.any(Object));
      expect(screen.getByText("Your business has been successfully launched.")).toBeInTheDocument();
    });
  });
});
