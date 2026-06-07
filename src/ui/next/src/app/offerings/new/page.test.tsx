import { render, screen, fireEvent } from '@testing-library/react';
import { expect, test, vi } from 'vitest';
import NewOfferingPage from './page';

// Mock Next.js router
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
  useSearchParams: () => new URLSearchParams(),
}));

test('renders the unified offering creation page correctly', () => {
  render(<NewOfferingPage />);

  expect(screen.getByText('Add Offering')).toBeInTheDocument();
  expect(screen.getByText('What do you want to offer?')).toBeInTheDocument();

  const generateButton = screen.getByText('Generate');
  expect(generateButton).toBeInTheDocument();
  expect(generateButton).toBeDisabled();
});
