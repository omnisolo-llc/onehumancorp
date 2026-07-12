import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralJobBoardGeneratorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('ViralJobBoardGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });

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
    render(<ViralJobBoardGeneratorPage />);
    expect(screen.getByText('Viral Job Board Generator 📢')).toBeDefined();
    expect(screen.getByText('Board Settings')).toBeDefined();
    expect(screen.getByText('Preview: Your Job Board Page')).toBeDefined();
  });

  it('updates form inputs and preview', () => {
    render(<ViralJobBoardGeneratorPage />);

    const titleInput = screen.getByPlaceholderText('e.g. We are hiring!');
    fireEvent.change(titleInput, { target: { value: 'Join the Revolution' } });

    const descInput = screen.getByPlaceholderText('e.g. Join our team and help us build the future.');
    fireEvent.change(descInput, { target: { value: 'We are looking for misfits.' } });

    // Check if preview updates
    const titleDisplays = screen.getAllByText('Join the Revolution');
    expect(titleDisplays.length).toBeGreaterThan(0);

    const descDisplays = screen.getAllByText('We are looking for misfits.');
    expect(descDisplays.length).toBeGreaterThan(0);
  });

  it('copies link to clipboard', async () => {
    render(<ViralJobBoardGeneratorPage />);

    const copyBtn = screen.getByRole('button', { name: 'Copy Link' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied!' })).toBeDefined();
  });

  it('navigates back to dashboard', () => {
    render(<ViralJobBoardGeneratorPage />);

    const backBtn = screen.getByRole('button', { name: /Back to Dashboard/i });
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });
});
