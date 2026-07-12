import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralBeforeAfterSliderPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

describe('ViralBeforeAfterSliderPage', () => {
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
    render(<ViralBeforeAfterSliderPage />);
    expect(screen.getByText('Before & After Slider')).toBeDefined();
    expect(screen.getByText('Widget Title')).toBeDefined();
    expect(screen.getByText('Before Image URL')).toBeDefined();
    expect(screen.getByText('After Image URL')).toBeDefined();
  });

  it('updates form inputs and iframe src', () => {
    render(<ViralBeforeAfterSliderPage />);

    const titleInput = screen.getByDisplayValue('Our Work');
    fireEvent.change(titleInput, { target: { value: 'Kitchen Remodel' } });

    // Using a simpler URL to test the encoding correctly
    const beforeInput = screen.getAllByRole('textbox')[1];
    fireEvent.change(beforeInput, { target: { value: 'https://example.com/before.jpg' } });

    const afterInput = screen.getAllByRole('textbox')[2];
    fireEvent.change(afterInput, { target: { value: 'https://example.com/after.jpg' } });

    // Check if iframe src updates
    const iframe = document.querySelector('iframe');
    expect(iframe?.getAttribute('src')).toContain('title=Kitchen%20Remodel');
    expect(iframe?.getAttribute('src')).toContain('before=https%3A%2F%2Fexample.com%2Fbefore.jpg');
    expect(iframe?.getAttribute('src')).toContain('after=https%3A%2F%2Fexample.com%2Fafter.jpg');
  });

  it('shows embed modal and copies code', async () => {
    render(<ViralBeforeAfterSliderPage />);

    const getWidgetBtn = screen.getByRole('button', { name: 'Get Widget Embed Code' });
    fireEvent.click(getWidgetBtn);

    expect(screen.getByText('Embed Slider')).toBeDefined();

    const copyBtn = screen.getByRole('button', { name: 'Copy Code' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied!' })).toBeDefined();
  });

  it('shows paywall when removing branding without pro', () => {
    render(<ViralBeforeAfterSliderPage />);

    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });
});
