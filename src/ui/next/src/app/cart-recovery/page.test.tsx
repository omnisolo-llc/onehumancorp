import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import CartRecoveryPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

describe('CartRecoveryPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ isEnabled: false, delay: "4", includeDiscount: true }),
    });
    Storage.prototype.getItem = vi.fn(() => 'Pro');
  });

  it('renders the page correctly', () => {
    render(<CartRecoveryPage />);
    expect(screen.getByText('Automated Cart Recovery 🛒')).toBeDefined();
    expect(screen.getByText('Enable Agent')).toBeDefined();
    expect(screen.getByText('Follow-up Delay')).toBeDefined();
  });

  it('handles enable/disable agent toggle', () => {
    render(<CartRecoveryPage />);

    const checkbox = screen.getByRole('checkbox', { name: '' });
    // Assuming the first checkbox is the enable agent
    // Since there are multiple checkboxes without aria-labels, we use the first one
    const checkboxes = screen.getAllByRole('checkbox');
    fireEvent.click(checkboxes[0]);
    expect((checkboxes[0] as HTMLInputElement).checked).toBe(true);
  });

  it('calls generate preview and displays result', async () => {
    const mockMessage = "Hi Sarah, you left some items...";
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ message: mockMessage }),
    });

    render(<CartRecoveryPage />);

    const generateButton = screen.getByText('Generate AI Preview');
    fireEvent.click(generateButton);

    expect(generateButton.textContent).toBe('Generating...');

    await waitFor(() => {
      expect(screen.getByText(mockMessage)).toBeDefined();
    });

    // Check if the regenerate button appears
    expect(screen.getByText('Regenerate Preview')).toBeDefined();
  });

  it('handles failed API response for preview', async () => {
    (global.fetch as any).mockResolvedValue({
      ok: false,
    });

    render(<CartRecoveryPage />);

    const generateButton = screen.getByText('Generate AI Preview');
    fireEvent.click(generateButton);

    await waitFor(() => {
      expect(screen.getByText('Failed to generate preview. Please try again.')).toBeDefined();
    });
  });

  it('handles save button click', async () => {
    render(<CartRecoveryPage />);

    const saveButton = screen.getByText('Save Configuration');
    fireEvent.click(saveButton);

    // It should change text to Saved!
    await waitFor(() => {
        expect(saveButton.textContent).toBe('Saved!');
    });
  });
});
