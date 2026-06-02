import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import GettingStartedArticle from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

describe('GettingStartedArticle', () => {
  it('renders the article with the correct title and text', () => {
    render(<GettingStartedArticle />);
    expect(screen.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeInTheDocument();
    expect(screen.getByText(/Welcome to OneHumanCorp!/)).toBeInTheDocument();
  });

  it('navigates back to the help center when the back button is clicked', () => {
    render(<GettingStartedArticle />);
    const backButton = screen.getByRole('button', { name: /Back to Help Center/i });
    fireEvent.click(backButton);
    expect(mockPush).toHaveBeenCalledWith('/help');
  });
});
