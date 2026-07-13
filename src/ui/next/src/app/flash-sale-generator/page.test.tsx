import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import FlashSaleGeneratorPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('FlashSaleGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  it('renders the Flash Sale Generator page correctly', () => {
    render(<FlashSaleGeneratorPage />);
    expect(screen.getByText('Flash Sale Generator ⚡')).toBeDefined();
    expect(screen.getByText('Widget Settings')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
    expect(screen.getByRole('button', { name: 'Get Widget' })).toBeDefined();
  });

  it('updates theme correctly', () => {
    render(<FlashSaleGeneratorPage />);

    const darkButton = screen.getByRole('button', { name: 'Dark' });
    fireEvent.click(darkButton);

    expect(darkButton.className).toContain('bg-white');
  });

  it('opens modal and displays embed code', () => {
    render(<FlashSaleGeneratorPage />);

    // Default tenant is 'my-store', theme is 'light', default title is 'Weekend Flash Sale!'
    const getWidgetButton = screen.getByRole('button', { name: 'Get Widget' });
    fireEvent.click(getWidgetButton);

    expect(screen.getByText('Embed Flash Sale')).toBeDefined();

    const textareas = screen.getAllByRole('textbox');
    const textarea = textareas.find(ta => (ta as HTMLTextAreaElement).value.includes('<iframe')) as HTMLTextAreaElement;
    expect(textarea.value).toContain('https://ohc.app/api/v1/growth/flash-sale/embed?tenant=my-store');
    expect(textarea.value).toContain('theme=light');
  });

  it('reflects updated title, percent, code, tenant and theme in embed code', () => {
    render(<FlashSaleGeneratorPage />);

    // Change Title
    const titleInput = screen.getByPlaceholderText('e.g. 24-Hour Flash Sale!');
    fireEvent.change(titleInput, { target: { value: 'Super Sale' } });

    // Change Percent
    const percentInput = screen.getByPlaceholderText('20');
    fireEvent.change(percentInput, { target: { value: '50' } });

    // Change Code
    const codeInput = screen.getByPlaceholderText('e.g. FLASH20');
    fireEvent.change(codeInput, { target: { value: 'SUPER50' } });

    // Change Tenant
    const tenantInput = screen.getByPlaceholderText('e.g. my-store');
    fireEvent.change(tenantInput, { target: { value: 'cool-shop' } });

    // Change Theme
    const darkButton = screen.getByRole('button', { name: 'Dark' });
    fireEvent.click(darkButton);

    const getWidgetButton = screen.getByRole('button', { name: 'Get Widget' });
    fireEvent.click(getWidgetButton);

    const textareas = screen.getAllByRole('textbox');
    const textarea = textareas.find(ta => (ta as HTMLTextAreaElement).value.includes('<iframe')) as HTMLTextAreaElement;

    expect(textarea.value).toContain('title=Super%20Sale');
    expect(textarea.value).toContain('code=SUPER50');
    expect(textarea.value).toContain('percent=50');
    expect(textarea.value).toContain('tenant=cool-shop');
    expect(textarea.value).toContain('theme=dark');
  });

  it('shows soft paywall when trying to remove branding without pro', () => {
    localStorage.setItem('has_pro', 'false');
    render(<FlashSaleGeneratorPage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Branding/i);
    fireEvent.click(removeBrandingCheckbox);

    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });

  it('allows removing branding when user has pro', () => {
    localStorage.setItem('has_pro', 'true');
    render(<FlashSaleGeneratorPage />);

    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" Branding/i);
    fireEvent.click(removeBrandingCheckbox);

    expect(removeBrandingCheckbox).toBeChecked();
    expect(screen.queryByText('Upgrade to Remove Branding')).toBeNull();

    const getWidgetButton = screen.getByRole('button', { name: 'Get Widget' });
    fireEvent.click(getWidgetButton);

    const textareas = screen.getAllByRole('textbox');
    const textarea = textareas.find(ta => (ta as HTMLTextAreaElement).value.includes('<iframe')) as HTMLTextAreaElement;
    expect(textarea.value).toContain('branding=false');
  });
});
