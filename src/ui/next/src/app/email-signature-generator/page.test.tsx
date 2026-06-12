import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import EmailSignatureGeneratorPage from './page';

// Mock Next.js router
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('EmailSignatureGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn();
    // Setup localStorage mock
    const store: Record<string, string> = {
      'tenant': 'demo-store',
      'has_pro': 'false',
    };
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: (key: string) => store[key] || null,
        setItem: (key: string, value: string) => { store[key] = value.toString(); },
      },
      writable: true
    });
  });

  it('renders the generator form', () => {
    render(<EmailSignatureGeneratorPage />);
    expect(screen.getByText('Email Signature Generator ✉️')).toBeDefined();
    expect(screen.getByPlaceholderText('Full Name (e.g. Maya Smith)')).toBeDefined();
    expect(screen.getByText('Generate Signature')).toBeDefined();
  });

  it('shows upgrade modal when trying to remove branding without Pro', async () => {
    render(<EmailSignatureGeneratorPage />);

    const checkbox = screen.getByRole('checkbox', { name: /Remove "Powered by OHC" Badge/i });
    fireEvent.click(checkbox);

    expect(screen.getByText('Upgrade to Pro')).toBeDefined();
    expect(checkbox).not.toBeChecked(); // should remain unchecked
  });

  it('calls generate endpoint and displays result', async () => {
    const mockHtml = '<div class="signature">Test Signature</div>';
    (global.fetch as any).mockResolvedValueOnce({
      json: vi.fn().mockResolvedValueOnce({ html: mockHtml }),
    });

    render(<EmailSignatureGeneratorPage />);

    const nameInput = screen.getByPlaceholderText('Full Name (e.g. Maya Smith)');
    fireEvent.change(nameInput, { target: { value: 'Test User' } });

    const generateBtn = screen.getByText('Generate Signature');
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/email-signature/generate', expect.any(Object));
      expect(screen.getByText('HTML Source')).toBeDefined();
      expect(screen.getByDisplayValue(mockHtml)).toBeDefined();
    });
  });
});
