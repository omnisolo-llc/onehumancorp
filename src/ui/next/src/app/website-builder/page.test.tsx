import { describe, test, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import Page from './page';

vi.mock('../../components/help', () => ({
  Tooltip: ({ children }: any) => <>{children}</>,
  useWalkthrough: () => ({ startWalkthrough: vi.fn() })
}));

describe('WebsiteBuilderPage', () => {
  let mockFetch: any;

  beforeEach(() => {
    mockFetch = vi.fn((url) => {
      if (url === '/api/v1/builder/generate') {
          return Promise.resolve({
            json: () => Promise.resolve({ pages: [{ blocks: [] }] }),
            ok: true
          });
      }
      return Promise.resolve({
        json: () => Promise.resolve({}),
        ok: true
      });
    });
    vi.stubGlobal('fetch', mockFetch);
    vi.stubGlobal('localStorage', {
      getItem: vi.fn(),
      setItem: vi.fn()
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  test('renders initial welcome state', async () => {
    render(<Page />);
    expect(screen.getByText('Your business, live in minutes.')).toBeDefined();
    expect(screen.getByText('Start My Business')).toBeDefined();
    expect(screen.getByText('Instant Build')).toBeDefined();
  });

  test('handles full Start My Business flow to publish', async () => {
    render(<Page />);
    fireEvent.click(screen.getByText('Start My Business'));

    // Step 1
    await waitFor(() => expect(screen.getByText('What kind of business are you building?')).toBeDefined());
    fireEvent.click(screen.getByText('Creative'));
    fireEvent.click(screen.getByText('Next'));

    // Step 2
    await waitFor(() => expect(screen.getByText('Give your business a name')).toBeDefined());
    fireEvent.change(screen.getByPlaceholderText('What is your business called?'), { target: { value: 'Test' } });
    fireEvent.click(screen.getByText('Next'));

    // Step 3
    await waitFor(() => expect(screen.getByText('What do you sell?')).toBeDefined());
    // Simulate clicking checkbox by checking the input directly
    const servicesCheckbox = screen.getAllByRole('checkbox')[0];
    fireEvent.click(servicesCheckbox);
    fireEvent.click(screen.getByText('Next'));

    // Step 4
    await waitFor(() => expect(screen.getByText("Let's add your first product")).toBeDefined());
    fireEvent.change(screen.getByPlaceholderText('What is the name of this product?'), { target: { value: 'Test Product' } });
    fireEvent.change(screen.getByPlaceholderText('0.00'), { target: { value: '10.00' } });
    fireEvent.click(screen.getByText('Next'));

    // Step 5
    await waitFor(() => expect(screen.getByText('How do you want to receive payments?')).toBeDefined());
    fireEvent.click(screen.getByText('Online'));
    fireEvent.click(screen.getByText('Next'));

    // Step 6
    await waitFor(() => expect(screen.getByText('Create your admin account')).toBeDefined());
    fireEvent.change(screen.getByPlaceholderText('e.g. Maya Smith'), { target: { value: 'Test Admin' } });
    fireEvent.change(screen.getByPlaceholderText('you@email.com'), { target: { value: 'test@test.com' } });
    fireEvent.change(screen.getByPlaceholderText('Password'), { target: { value: 'password' } });
    fireEvent.click(screen.getByText('Next'));

    // Step 7
    await waitFor(() => expect(screen.getByText('Choose a starting template')).toBeDefined());
    fireEvent.click(screen.getByText('Modern'));
    fireEvent.click(screen.getByText('Next'));

    // Step 8
    await waitFor(() => expect(screen.getByText('Choose your domain')).toBeDefined());
    fireEvent.click(screen.getByText('Free OHC Domain'));
    fireEvent.click(screen.getByText('Next'));

    // Step 9
    await waitFor(() => expect(screen.getByText('Ready to launch!')).toBeDefined());

    // Test publish
    fireEvent.click(screen.getByText('Publish my business'));

    await waitFor(() => {
        expect(mockFetch).toHaveBeenCalledWith('/api/onboarding/start', expect.any(Object));
    });

    // Step 10
    await waitFor(() => expect(screen.getByText('CONFETTI SUCCESS')).toBeDefined());
    fireEvent.click(screen.getByText('View Welcome Checklist'));

    // Step 11
    await waitFor(() => expect(screen.getByText("You're set up! Here's what to do next:")).toBeDefined());
  });

  test('handles Instant Build flow', async () => {
    render(<Page />);
    fireEvent.click(screen.getByText('Instant Build'));
    await waitFor(() => {
      expect(screen.getByText('Describe your business in a sentence')).toBeDefined();
    });

    // Can test generating here, but since it uses setTimeout, we'll keep it simple
    fireEvent.click(screen.getByText('Generate Storefront'));
    await waitFor(() => {
        expect(screen.getByText('Agents are building your store...')).toBeDefined();
    });
  });
});
