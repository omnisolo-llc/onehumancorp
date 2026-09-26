import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import OutcomeFeedAuthorityHelpPage from './page';

describe('OutcomeFeedAuthorityHelpPage', () => {
  it('renders correctly', () => {
    render(<OutcomeFeedAuthorityHelpPage />);
    expect(screen.getByText('Managing Standing Authority Limits')).toBeTruthy();
  });
});
