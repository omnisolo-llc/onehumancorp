import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import AutonomousOnboarding from './page';
import { vi } from 'vitest';

describe('AutonomousOnboarding', () => {
  beforeEach(() => {
    // Reset fetch mock before each test
    global.fetch = vi.fn() as any;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders initial chat state', () => {
    render(<AutonomousOnboarding />);
    expect(screen.getByText('OHC Setup Agent')).toBeInTheDocument();
    expect(screen.getByText("Hi! Let's get your business online. What do you sell?")).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Type your message...')).toBeInTheDocument();
  });

  it('handles sending a message', async () => {
    const mockReply = 'Great! Could you provide an example photo?';
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ reply: mockReply, is_complete: false }),
    });

    render(<AutonomousOnboarding />);

    const input = screen.getByPlaceholderText('Type your message...');
    fireEvent.change(input, { target: { value: 'I make vegan cakes' } });
    fireEvent.keyDown(input, { key: 'Enter', code: 'Enter' });

    // User message should appear
    expect(screen.getByText('I make vegan cakes')).toBeInTheDocument();

    // Wait for the mock fetch to resolve and reply to appear
    await waitFor(() => {
      expect(screen.getByText(mockReply)).toBeInTheDocument();
    });
  });

  it('shows credentials form when intake is complete', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        reply: "Give me a minute...",
        is_complete: true,
        intake_data: { business_name: 'Test Business' }
      }),
    });

    render(<AutonomousOnboarding />);

    const input = screen.getByPlaceholderText('Type your message...');
    fireEvent.change(input, { target: { value: 'I make vegan cakes' } });
    fireEvent.keyDown(input, { key: 'Enter', code: 'Enter' });

    // Wait for the credentials form to appear
    await waitFor(() => {
      expect(screen.getByText('Create Owner Account')).toBeInTheDocument();
    });

    // Check that input fields are present
    expect(screen.getByPlaceholderText('Your Name')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Email Address')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Password (8+ chars, 1 number)')).toBeInTheDocument();
  });
});
