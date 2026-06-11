import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import SpinToWinGeneratorPage from './page';

// Mock Next.js router
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('SpinToWinGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset window and local storage
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn((key) => {
          if (key === 'tenant_id') return 'test-tenant';
          return null;
        }),
        setItem: vi.fn(),
      },
      writable: true,
    });

    // Mock navigator.clipboard
    Object.defineProperty(navigator, 'clipboard', {
      value: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
      writable: true,
    });
  });

  it('renders correctly', () => {
    render(<SpinToWinGeneratorPage />);
    expect(screen.getByText('Spin-to-Win Generator')).toBeDefined();
    expect(screen.getByText('Campaign Details')).toBeDefined();
    expect(screen.getByText('Wheel Slices')).toBeDefined();
    expect(screen.getByText('Preview & Embed')).toBeDefined();
  });

  it('allows changing campaign details', () => {
    render(<SpinToWinGeneratorPage />);

    const titleInputs = screen.getAllByDisplayValue('Spin to Win!');
    expect(titleInputs.length).toBeGreaterThan(0);

    const titleInput = titleInputs[0];
    fireEvent.change(titleInput, { target: { value: 'Holiday Special Spin' } });

    expect(screen.getAllByDisplayValue('Holiday Special Spin').length).toBeGreaterThan(0);

    // Test the preview title updates
    expect(screen.getByText('Holiday Special Spin')).toBeDefined();
  });

  it('allows changing wheel slice labels and values', () => {
    render(<SpinToWinGeneratorPage />);

    // Use getAllByDisplayValue to find the specific input
    const sliceLabels = screen.getAllByDisplayValue('10% Off');
    fireEvent.change(sliceLabels[0], { target: { value: '15% Off' } });

    expect(screen.getAllByDisplayValue('15% Off').length).toBeGreaterThan(0);

    const sliceValues = screen.getAllByDisplayValue('10OFF');
    fireEvent.change(sliceValues[0], { target: { value: '15OFF' } });

    expect(screen.getAllByDisplayValue('15OFF').length).toBeGreaterThan(0);
  });

  it('generates the correct embed code', () => {
    render(<SpinToWinGeneratorPage />);

    // There are multiple text inputs, the textarea is the one with the embed code
    const textAreas = screen.getAllByRole('textbox');
    // find the textarea
    const textArea = textAreas.find(t => (t as HTMLTextAreaElement).value.includes('<iframe')) as HTMLTextAreaElement;

    expect(textArea.value).toContain('<iframe src="');
    expect(textArea.value).toContain('tenant=test-tenant');
    expect(textArea.value).toContain('title=Spin%20to%20Win!');
    expect(textArea.value).toContain('⚡ Powered by OHC');
  });

  it('copies the embed code to clipboard and shows "Copied!"', async () => {
    render(<SpinToWinGeneratorPage />);

    const copyButton = screen.getByRole('button', { name: /Copy HTML/i });
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();

    expect(screen.getByRole('button', { name: /Copied!/i })).toBeDefined();
  });
});
