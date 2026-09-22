import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import OutcomeFeedHelpPage from './page';

describe('OutcomeFeedHelpPage', () => {
  it('renders correctly', () => {
    render(<OutcomeFeedHelpPage />);
    expect(screen.getByText('Understanding the Owner Outcome Feed')).toBeTruthy();
  });
});
