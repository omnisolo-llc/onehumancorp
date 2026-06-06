import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import MyStoreHelpPage from './page';
import { TooltipProvider } from '../../../components/TooltipRegistry';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn(), replace: vi.fn(), prefetch: vi.fn() }),
  usePathname: () => '/help/my-store',
}));

describe('My Store Help Page', () => {
  it('renders the My Store help page correctly', () => {
    render(
      <TooltipProvider>
        <MyStoreHelpPage />
      </TooltipProvider>
    );
    expect(screen.getByText('My Store: Adding Products & Managing Inventory')).toBeInTheDocument();
    expect(screen.getByText(/Setting up your storefront is easy/i)).toBeInTheDocument();
    expect(screen.getByText('Add Product')).toBeInTheDocument();
  });
});
