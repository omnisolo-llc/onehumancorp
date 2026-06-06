import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import NewProductPage from './page';

global.fetch = vi.fn(() =>
  Promise.resolve({
    ok: true,
    json: () => Promise.resolve({ success: true }),
  })
) as jest.Mock;

describe('NewProductPage', () => {
  it('should render the Require Deposit toggle after product data is loaded and update state', async () => {
    const { container } = render(<NewProductPage />);

    // Simulate uploading a file to set productData (which triggers the UI with the toggle to appear)
    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(['dummy content'], 'example.png', { type: 'image/png' });
    fireEvent.change(fileInput, { target: { files: [file] } });

    // Wait for the upload mock to resolve and the UI to update
    await waitFor(() => {
      expect(screen.getByText('Require Deposit')).toBeInTheDocument();
    });

    const label = screen.getByText('Require Deposit');
    const checkbox = container.querySelector('input[type="checkbox"][class*="sr-only"]') as HTMLInputElement;
    expect(checkbox).not.toBeNull();

    // Check initial state
    expect(checkbox.checked).toBe(false);

    // Click the label to toggle it
    fireEvent.click(label);

    // Check if state is updated
    await waitFor(() => {
      expect(checkbox.checked).toBe(true);
    });
  });
});
