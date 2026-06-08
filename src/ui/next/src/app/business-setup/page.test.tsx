import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { describe, it, expect } from 'vitest';
import BusinessSetupCompatibilityPage from './page';

describe('BusinessSetupCompatibilityPage', () => {
  it('renders the initial business setup screen with correct content and link', () => {
    render(<BusinessSetupCompatibilityPage />);

    expect(screen.getByRole('main')).toHaveAttribute('id', 'business-setup-screen');
    expect(screen.getByText('OneHuman')).toBeInTheDocument();
    expect(screen.getByText('Your business, live in minutes.')).toBeInTheDocument();
    expect(
      screen.getByText('Start the setup wizard to launch a database-backed OHC storefront and operations workspace.')
    ).toBeInTheDocument();

    const linkElement = screen.getByRole('link', { name: /Start Business Setup/i });
    expect(linkElement).toBeInTheDocument();
    expect(linkElement).toHaveAttribute('href', '/onboarding');
  });
});
