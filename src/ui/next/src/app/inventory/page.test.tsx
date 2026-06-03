import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import InventoryPage from './page';

describe('InventoryPage', () => {
  it('renders the AI restock alert', () => {
    render(<InventoryPage />);
    expect(screen.getByText(/✨ Heads up Priya/)).toBeInTheDocument();
  });

  it('renders inventory items', () => {
    render(<InventoryPage />);
    expect(screen.getByText('Blue Summer Dress (Size M)')).toBeInTheDocument();
    expect(screen.getByText('Low Stock')).toBeInTheDocument();
  });
});
