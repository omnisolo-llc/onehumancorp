import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralTierListGeneratorPage from './page';

describe('ViralTierListGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant_id' || key === 'tenant') return 'test-tenant';
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
    render(<ViralTierListGeneratorPage />);
    expect(screen.getAllByText('Viral Tier List Generator').length).toBeGreaterThan(0);
    expect(screen.getByText('List Title')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('updates form inputs and preview', () => {
    render(<ViralTierListGeneratorPage />);

    const titleInput = screen.getByPlaceholderText('e.g., Best Coffees of 2024');
    fireEvent.change(titleInput, { target: { value: 'Top Movies' } });

    const descInput = screen.getByPlaceholderText('e.g., A definitive ranking of my favorites.');
    fireEvent.change(descInput, { target: { value: 'Best movies of the decade' } });

    // Check if preview updates
    expect(screen.getByText('Top Movies')).toBeDefined();
    expect(screen.getByText('Best movies of the decade')).toBeDefined();
  });

  it('generates a share link', () => {
    render(<ViralTierListGeneratorPage />);

    const titleInput = screen.getByPlaceholderText('e.g., Best Coffees of 2024');
    fireEvent.change(titleInput, { target: { value: 'Test List' } });

    const generateBtn = screen.getByRole('button', { name: 'Generate Share Link' });
    fireEvent.click(generateBtn);

    const linkInput = screen.getByTestId('generated-link') as HTMLInputElement;
    expect(linkInput.value).toContain('Test%20List');
  });

  it('shows paywall when removing branding without pro', () => {
    render(<ViralTierListGeneratorPage />);

    const toggle = screen.getByTestId('branding-toggle');
    fireEvent.click(toggle);

    expect(screen.getByText('Upgrade to Pro')).toBeDefined();
  });
});
