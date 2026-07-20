import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import ViralGiveawayWidgetPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('ViralGiveawayWidgetPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'business_display_name') return 'TestBusiness';
        return null;
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly', () => {
    render(<ViralGiveawayWidgetPage />);
    expect(screen.getByText('Viral Giveaway Generator 🏆')).toBeDefined();
    expect(screen.getByText('Giveaway Settings')).toBeDefined();
    expect(screen.getByText('Preview: Your Landing Page')).toBeDefined();
  });

  it('updates form inputs and preview', () => {
    render(<ViralGiveawayWidgetPage />);

    const titleInput = screen.getAllByRole('textbox')[0]; // First input is title
    fireEvent.change(titleInput, { target: { value: 'Win a New Laptop' } });

    const prizeInput = screen.getAllByRole('textbox')[1]; // Second input is prize
    fireEvent.change(prizeInput, { target: { value: 'MacBook Pro' } });

    const winnersInput = screen.getByRole('spinbutton'); // Number input
    fireEvent.change(winnersInput, { target: { value: '3' } });

    // Check if preview updates
    expect(screen.getByText('Win a New Laptop')).toBeDefined();
    expect(screen.getByText('MacBook Pro')).toBeDefined();
    expect(screen.getByText('3 Winners')).toBeDefined();
  });

  it('generates a link when button is clicked', async () => {
    render(<ViralGiveawayWidgetPage />);

    const generateBtn = screen.getByRole('button', { name: 'Generate Widget' });
    fireEvent.click(generateBtn);

    expect(screen.getByText('Generating...')).toBeDefined();

    await waitFor(() => {
        expect(screen.getByText('🚀')).toBeDefined();
    }, { timeout: 1500 });
  });

  it('navigates back to dashboard', () => {
    render(<ViralGiveawayWidgetPage />);

    const backBtn = screen.getByRole('button', { name: /Back to Dashboard/i });
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });
});
