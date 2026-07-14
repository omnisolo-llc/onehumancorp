import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import NewProductPage from './page';

// Mock useRouter and useSearchParams
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
  useSearchParams: () => ({
    get: vi.fn().mockReturnValue('manual'), // mock return value to trigger manual mode
  }),
}));

describe('NewProductPage', () => {
  it('renders correctly in manual mode', () => {
    render(<NewProductPage />);
    expect(screen.getByText('Add Product')).toBeDefined();
  });

  it('can enable subscribe & save and set frequency and discount in manual mode', async () => {
    // mock global fetch
    global.fetch = vi.fn(() =>
        Promise.resolve({
            json: () => Promise.resolve({
                title: "Artisan Coffee Beans",
                price: "18.99",
                item_type: "Product",
                is_subscription: false
            }),
            ok: true
        })
    ) as any;

    render(<NewProductPage />);

    // Click "Manual" button to go to manual mode
    const describeButton = screen.getByText('Or describe your offering');
    fireEvent.click(describeButton);

    const textArea = screen.getByPlaceholderText('e.g., Guitar lessons for beginners, 1 hour');
    fireEvent.change(textArea, { target: { value: 'Guitar lessons' } });

    const generateButton = screen.getByText('Generate');
    fireEvent.click(generateButton);

    await waitFor(() => {
        expect(screen.getByText('Enable Subscribe & Save')).toBeDefined();
    });

    const enableSubscribeCheckbox = screen.getByText('Enable Subscribe & Save');

    // Toggle subscription ON
    fireEvent.click(enableSubscribeCheckbox);

    // The subscription options should now be visible
    expect(screen.getByText('Deliver every')).toBeDefined();
    expect(screen.getByText('Discount %')).toBeDefined();

    // Change frequency
    const select = screen.getByRole('combobox');
    fireEvent.change(select, { target: { value: 'monthly' } });
    expect((select as HTMLSelectElement).value).toBe('monthly');

    // Change discount
    const discountInputs = screen.getAllByRole('spinbutton');
    const discountInput = discountInputs.length > 1 ? discountInputs[1] : discountInputs[0];
    if (discountInput) {
        fireEvent.change(discountInput, { target: { value: '15' } });
        expect((discountInput as HTMLInputElement).value).toBe('15');
    }
  });
});
