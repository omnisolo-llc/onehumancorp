import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import StorefrontWidgetPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('StorefrontWidgetPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  it('renders the Storefront Widget page correctly', () => {
    render(<StorefrontWidgetPage />);
    expect(screen.getByText('Embed Your Store 🌐')).toBeDefined();
    expect(screen.getByText('Widget Settings')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
    expect(screen.getByRole('button', { name: 'Get Widget' })).toBeDefined();
  });

  it('updates theme correctly', () => {
    render(<StorefrontWidgetPage />);

    const darkButton = screen.getByRole('button', { name: 'Dark' });
    fireEvent.click(darkButton);

    expect(darkButton.className).toContain('bg-white');
  });

  it('opens modal and displays embed code', () => {
    render(<StorefrontWidgetPage />);

    // Default tenant is 'my-store', theme is 'light'
    const getWidgetButton = screen.getByRole('button', { name: 'Get Widget' });
    fireEvent.click(getWidgetButton);

    expect(screen.getByText('Embed Storefront')).toBeDefined();

    const textareas = screen.getAllByRole('textbox');
    const textarea = textareas[1] as HTMLTextAreaElement;
    expect(textarea.value).toContain('<iframe src="https://ohc.app/api/v1/growth/storefront/embed?tenant=my-store&theme=light"');
  });

  it('reflects updated tenant and theme in embed code', () => {
    render(<StorefrontWidgetPage />);

    // Change Tenant
    const tenantInput = screen.getByPlaceholderText('e.g. my-store');
    fireEvent.change(tenantInput, { target: { value: 'cool-shop' } });

    // Change Theme
    const darkButton = screen.getByRole('button', { name: 'Dark' });
    fireEvent.click(darkButton);

    const getWidgetButton = screen.getByRole('button', { name: 'Get Widget' });
    fireEvent.click(getWidgetButton);

    const textareas = screen.getAllByRole('textbox');
    const textarea = textareas[1] as HTMLTextAreaElement;
    expect(textarea.value).toContain('tenant=cool-shop&theme=dark');
  });
});
