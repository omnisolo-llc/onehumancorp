import React from 'react';
import { render, screen, act, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ShareAndSaveWidgetPage from './page';
import * as navigation from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({ push: vi.fn() })),
}));

describe('ShareAndSaveWidgetPage', () => {
  const mockPush = vi.fn();

  beforeEach(() => {
    (navigation.useRouter as any).mockReturnValue({ push: mockPush });
    vi.clearAllMocks();

    const localStorageMock = {
      getItem: vi.fn().mockImplementation((key) => {
        if (key === 'tenant') return 'test-tenant';
        if (key === 'has_pro') return 'false';
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

  it('renders the widget UI correctly', async () => {
    await act(async () => {
      render(<ShareAndSaveWidgetPage />);
    });

    expect(screen.getByText('Unlock 10% Off!')).toBeDefined();
    expect(screen.getByRole('button', { name: 'Share on X to Unlock' })).toBeDefined();
  });

  it('shows the back to dashboard button', async () => {
    await act(async () => {
      render(<ShareAndSaveWidgetPage />);
    });

    const backButton = screen.getByRole('button', { name: 'Back to Dashboard' });
    expect(backButton).toBeDefined();
  });
});
