import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { expect, test, vi, beforeEach } from 'vitest';
import DigitizeProduct from './page';

global.fetch = vi.fn();

beforeEach(() => {
  vi.clearAllMocks();
});

test('renders the upload state initially', () => {
  render(<DigitizeProduct />);
  expect(screen.getByText('Digitize Product')).toBeInTheDocument();
  expect(screen.getByText('Tap to Digitize')).toBeInTheDocument();
});

test('handles file upload and digitize simulation', async () => {
  // @ts-ignore
  global.fetch.mockResolvedValueOnce({
    ok: true,
    json: async () => ({
      success: true,
      data: {
        title: 'Mock Cupcake',
        price: '5.99',
        category: 'Dessert',
        description: 'A mock cupcake for testing.'
      }
    })
  });

  render(<DigitizeProduct />);

  const fileInput = document.querySelector('input[type="file"]') as HTMLInputElement;
  const file = new File(['mock'], 'mock.png', { type: 'image/png' });

  fireEvent.change(fileInput, { target: { files: [file] } });

  expect(screen.getByText('Digitizing and extracting metadata...')).toBeInTheDocument();

  await waitFor(() => {
    expect(screen.getByDisplayValue('Mock Cupcake')).toBeInTheDocument();
    expect(screen.getByDisplayValue('5.99')).toBeInTheDocument();
  });
});

test('handles file upload with api error gracefully', async () => {
  // @ts-ignore
  global.fetch.mockRejectedValueOnce(new Error('API Down'));

  render(<DigitizeProduct />);

  const fileInput = document.querySelector('input[type="file"]') as HTMLInputElement;
  const file = new File(['mock'], 'mock.png', { type: 'image/png' });

  fireEvent.change(fileInput, { target: { files: [file] } });

  await waitFor(() => {
    // Should fallback to default data
    expect(screen.getByDisplayValue('Artisan Vanilla Bean Cupcake')).toBeInTheDocument();
  });
});

test('publishes product successfully', async () => {
  // Mock digitize
  // @ts-ignore
  global.fetch.mockResolvedValueOnce({
    ok: true,
    json: async () => ({
      success: true,
      data: {
        title: 'Mock Cupcake',
        price: '5.99',
        category: 'Dessert',
        description: 'A mock cupcake for testing.'
      }
    })
  });

  render(<DigitizeProduct />);

  const fileInput = document.querySelector('input[type="file"]') as HTMLInputElement;
  const file = new File(['mock'], 'mock.png', { type: 'image/png' });
  fireEvent.change(fileInput, { target: { files: [file] } });

  await waitFor(() => {
    expect(screen.getByDisplayValue('Mock Cupcake')).toBeInTheDocument();
  });

  // Mock publish
  // @ts-ignore
  global.fetch.mockResolvedValueOnce({ ok: true });

  const publishBtn = screen.getByText('Publish to Store & Instagram');
  fireEvent.click(publishBtn);

  await waitFor(() => {
    expect(screen.getByText('Product Published!')).toBeInTheDocument();
  });
});
