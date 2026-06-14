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
    expect(screen.getByText('Chocolate Cake')).toBeDefined();
    expect(screen.getByText('Vanilla Celebration Cake')).toBeDefined();
    expect(screen.getByText('Wedding Cake Consultation')).toBeDefined();
  });

  it('opens and closes the QR modal', () => {
    render(<ProductsPage />);

    // Click Generate QR Code for the first product
    const generateButtons = screen.getAllByText('Generate QR Code');
    fireEvent.click(generateButtons[0]);

    // Check if modal is open
    expect(screen.getByText('Checkout QR Code')).toBeDefined();

    // Close modal
    const closeButton = screen.getByRole('button', { name: '' }); // the SVG button doesn't have a label, but we can click it or its icon. Wait, the configure button also doesn't have a label for the SVG. Let's just find the close button by SVG or its class.
    // Actually the close button has a path d="M6 18L18 6M6 6l12 12". Let's use a simpler way.
    const buttons = screen.getAllByRole('button');
    // The close button is the first one in the modal.
    fireEvent.click(buttons[buttons.length - 2]); // The close button is before Save / Print

    // Should be closed. We check using queryByText which returns null if not found
    expect(screen.queryByText('Checkout QR Code')).toBeNull();
  });

  it('opens configure modal, toggles inputs and saves', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({}),
    } as Response);

    render(<ProductsPage />);

    const configureButtons = screen.getAllByText('Configure');
    fireEvent.click(configureButtons[0]);

    expect(screen.getByText('Configure Subscription')).toBeDefined();

    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    // Now the frequency and discount inputs should be visible
    expect(screen.getByText('Delivery Frequency (days)')).toBeDefined();

    const saveButton = screen.getByText('Save Configuration');
    fireEvent.click(saveButton);

    await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/subscriptions/configure', expect.any(Object));
        expect(screen.queryByText('Configure Subscription')).toBeNull();
    });
  });
});
