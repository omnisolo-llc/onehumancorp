import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ProductsPage from './page';

// Mock AppShell to avoid complex rendering issues
vi.mock('../components/AppShell', () => ({
  AppShell: ({ children, actions }: any) => (
    <div data-testid="app-shell">
      {actions.map((action: any) => (
        <a key={action.label} href={action.href}>{action.label}</a>
      ))}
      {children}
    </div>
  ),
}));

describe('ProductsPage', () => {
  it('renders correctly', () => {
    render(<ProductsPage />);
    expect(screen.getByText('Imported Products')).toBeDefined();
  });

  it('can open and close the Subscribe & Save modal', async () => {
    // Mock the fetch call
    global.fetch = vi.fn(() =>
      Promise.resolve({
        json: () => Promise.resolve([{ title: "Test Product", price_cents: 1000, status: "Active" }]),
        ok: true
      })
    ) as any;

    render(<ProductsPage />);

    // Wait for the mock product to render
    await waitFor(() => {
      expect(screen.getByText('Test Product')).toBeDefined();
    });

    const enableSubscribeButton = screen.getByText('Enable Subscribe & Save');
    expect(enableSubscribeButton).toBeDefined();

    // Open modal
    fireEvent.click(enableSubscribeButton);
    expect(screen.getByText('Subscribe & Save')).toBeDefined();
    expect(screen.getByText('Delivery Frequency')).toBeDefined();
    expect(screen.getByText('Discount %')).toBeDefined();

    // Close modal via save button
    const saveButton = screen.getByText('Save Subscription');
    fireEvent.click(saveButton);

    // Modal should be gone
    await waitFor(() => {
      expect(screen.queryByText('Subscribe & Save')).toBeNull();
    });
  });

});
