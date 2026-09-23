import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ConnectedAccountsStandingAuthorityArticle from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

describe('ConnectedAccountsStandingAuthorityArticle', () => {
  it('renders the article with the correct title and text', () => {
    render(<ConnectedAccountsStandingAuthorityArticle />);
    expect(screen.getByRole('heading', { name: 'Connected Accounts and Standing Authority' })).toBeInTheDocument();
    expect(screen.getByText(/To set up your business/)).toBeInTheDocument();
  });

  it('navigates back to the help center when the back button is clicked', () => {
    render(<ConnectedAccountsStandingAuthorityArticle />);
    const backButton = screen.getByRole('button', { name: /Back to Help Center/i });
    fireEvent.click(backButton);
    expect(mockPush).toHaveBeenCalledWith('/help');
  });
});
