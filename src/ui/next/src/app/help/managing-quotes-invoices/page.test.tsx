import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ManagingQuotesInvoicesPage from './page';

vi.mock('next/link', () => ({
  default: ({ children, href }: { children: React.ReactNode; href: string }) => (
    <a href={href} data-testid="mock-link">{children}</a>
  ),
}));

describe('ManagingQuotesInvoicesPage', () => {
  it('renders the page title and content', () => {
    render(<ManagingQuotesInvoicesPage />);
    expect(screen.getByRole('heading', { name: 'How to Send Proposals and Collect Payments Securely' })).toBeInTheDocument();
    expect(screen.getByText(/As a small business owner, it's critical to know/)).toBeInTheDocument();

    const backButton = screen.getByRole('button', { name: /Back to Help Center/i });
    expect(backButton).toBeInTheDocument();
  });
});
