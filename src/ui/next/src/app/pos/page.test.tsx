import React from 'react';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { describe, expect, it, beforeEach } from 'vitest';
import POSPage from './page';

describe('POSPage', () => {
  beforeEach(() => {
    Object.defineProperty(navigator, 'onLine', {
      configurable: true,
      value: true,
    });
  });

  it('renders the catalog and adds a product to the cart', async () => {
    render(<POSPage />);

    await waitFor(() => expect(screen.getByText('Custom Cake')).toBeInTheDocument());
    expect(screen.getByRole('heading', { name: 'POS Terminal' })).toBeInTheDocument();
    expect(screen.getByText('Cart (0)')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: /Custom Cake/ }));

    expect(screen.getByText('Cart (1)')).toBeInTheDocument();
    expect(screen.getByText('1x Custom Cake')).toBeInTheDocument();
  });
});
