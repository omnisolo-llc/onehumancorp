import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { ShareAndSaveWidget } from './ShareAndSaveWidget';
import React from 'react';

Object.assign(window, {
  open: vi.fn(),
});

describe('ShareAndSaveWidget', () => {
  const mockOnShareComplete = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly with discount percentage', () => {
    render(<ShareAndSaveWidget tenantId="test-tenant" discountPercentage={15} onShareComplete={mockOnShareComplete} />);

    expect(screen.getByText('Share This Store')).toBeDefined();
    expect(screen.getByText(/Share our store with your friends/)).toBeDefined();
    expect(screen.getByTestId('share-x-btn')).toBeDefined();
    expect(screen.getByTestId('share-wa-btn')).toBeDefined();
  });

  it('opens X without applying an unverified discount', () => {
    render(<ShareAndSaveWidget tenantId="test-tenant" discountPercentage={10} onShareComplete={mockOnShareComplete} />);

    const xButton = screen.getByTestId('share-x-btn');
    fireEvent.click(xButton);

    expect(window.open).toHaveBeenCalledWith(
      expect.stringContaining('twitter.com/intent/tweet'),
      '_blank'
    );
    expect(mockOnShareComplete).not.toHaveBeenCalled();
    expect(screen.getByText(/No 10% discount has been applied/)).toBeDefined();
  });

  it('opens WhatsApp without applying an unverified discount', () => {
    render(<ShareAndSaveWidget tenantId="test-tenant" discountPercentage={20} onShareComplete={mockOnShareComplete} />);

    const waButton = screen.getByTestId('share-wa-btn');
    fireEvent.click(waButton);

    expect(window.open).toHaveBeenCalledWith(
      expect.stringContaining('wa.me/'),
      '_blank'
    );
    expect(mockOnShareComplete).not.toHaveBeenCalled();
    expect(screen.getByText(/No 20% discount has been applied/)).toBeDefined();
  });
});
