import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import CartRecoveryPage from './page';

describe('CartRecoveryPage', () => {
  it('renders a heading', () => {
    render(<CartRecoveryPage />);
    const heading = screen.getByRole('heading', { name: /cart recovery/i });
    expect(heading).toBeInTheDocument();
  });
});
