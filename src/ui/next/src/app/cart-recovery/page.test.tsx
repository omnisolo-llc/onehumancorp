import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { describe, it, expect, vi } from 'vitest';
import CartRecoveryPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('CartRecoveryPage', () => {
  it('renders correctly', () => {
    render(<CartRecoveryPage />);
    expect(screen.getByText(/Abandoned Cart Recovery/i)).toBeInTheDocument();
  });
});
