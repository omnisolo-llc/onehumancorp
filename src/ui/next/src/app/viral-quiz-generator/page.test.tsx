import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import Page from './page';

// Mock the useRouter hook
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

// Mock the PoweredByOHC component
vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

// Mock navigator.clipboard
const mockClipboard = {
  writeText: vi.fn(),
};
Object.assign(navigator, { clipboard: mockClipboard });

describe('ViralQuizGenerator Page', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  it('renders the builder and preview correctly', () => {
    render(<Page />);

    // Header
    expect(screen.getByText('Viral Quiz Generator 🧠')).toBeDefined();

    // Builder Settings
    expect(screen.getByText('Quiz Settings')).toBeDefined();
    expect(screen.getByDisplayValue('What type of founder are you?')).toBeDefined();

    // Preview Section
    expect(screen.getByText('Live Preview')).toBeDefined();
    expect(screen.getByText('Start Quiz')).toBeDefined();
  });

  it('updates the preview when builder inputs change', () => {
    render(<Page />);

    const titleInput = screen.getByDisplayValue('What type of founder are you?');

    fireEvent.change(titleInput, { target: { value: 'New Custom Quiz Title' } });

    // Expect the preview title to update
    const previewTitles = screen.getAllByText('New Custom Quiz Title');
    expect(previewTitles.length).toBeGreaterThan(0);
  });

  it('simulates the quiz flow', async () => {
    render(<Page />);

    // Start Quiz
    const startButton = screen.getByText('Start Quiz');
    fireEvent.click(startButton);

    // Check if question view is active
    expect(screen.getByText('Question 1 of 3')).toBeDefined();

    // Select an option
    const optionBtn = screen.getByText('Take immediate charge');
    fireEvent.click(optionBtn);

    // Wait for the timeout to switch to results
    await waitFor(() => {
      expect(screen.getByText('Your Results Are Ready!')).toBeDefined();
    });

    // Check for share to unlock buttons
    expect(screen.getByText('Share on X to Unlock')).toBeDefined();
    expect(screen.getByText('Retake Quiz')).toBeDefined();

    // Retake Quiz
    const retakeBtn = screen.getByText('Retake Quiz');
    fireEvent.click(retakeBtn);

    // Back to start
    expect(screen.getByText('Start Quiz')).toBeDefined();
  });

  it('copies link to clipboard', async () => {
    render(<Page />);

    const copyButton = screen.getByText('📋 Copy Link');
    fireEvent.click(copyButton);

    expect(mockClipboard.writeText).toHaveBeenCalled();
    expect(screen.getByText('✓ Copied!')).toBeDefined();

    // Wait for the "Copied!" text to revert
    await waitFor(() => {
      expect(screen.getByText('📋 Copy Link')).toBeDefined();
    }, { timeout: 2500 });
  });
});
