import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralJobBoardGeneratorPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({
    push: vi.fn(),
  })),
}));

describe('ViralJobBoardGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(),
      },
    });
  });

  it('renders correctly', () => {
    render(<ViralJobBoardGeneratorPage />);
    expect(screen.getByText('Viral Job Board Generator 📢')).toBeDefined();
  });

  it('copies generated link to clipboard', () => {
    render(<ViralJobBoardGeneratorPage />);
    const copyButton = screen.getAllByRole('button', { name: /Copy Link/i })[0];
    fireEvent.click(copyButton);
    expect(navigator.clipboard.writeText).toHaveBeenCalled();
  });
});
