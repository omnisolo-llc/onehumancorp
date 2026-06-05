import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import VideoTutorialsPage from './page';

// Mock the VideoTutorialList component
vi.mock('../../../components/VideoTutorialList', () => ({
  VideoTutorialList: () => <div data-testid="video-tutorial-list-mock">Mocked VideoTutorialList</div>,
}));

// Mock Next.js Link
vi.mock('next/link', () => ({
  default: ({ children, href }: { children: React.ReactNode; href: string }) => (
    <a href={href} data-testid="next-link-mock">
      {children}
    </a>
  ),
}));

describe('VideoTutorialsPage', () => {
  it('renders the page title and description', () => {
    render(<VideoTutorialsPage />);

    expect(screen.getByRole('heading', { name: 'Video Guides', level: 1 })).toBeInTheDocument();
    expect(
      screen.getByText('Watch quick, simple tutorials to learn how to manage your store like a pro.')
    ).toBeInTheDocument();
  });

  it('renders the Back to Help Center link', () => {
    render(<VideoTutorialsPage />);

    const backLink = screen.getByRole('link', { name: /Back to Help Center/i });
    expect(backLink).toBeInTheDocument();
    expect(backLink).toHaveAttribute('href', '/help');
  });

  it('renders the VideoTutorialList component', () => {
    render(<VideoTutorialsPage />);

    expect(screen.getByTestId('video-tutorial-list-mock')).toBeInTheDocument();
  });
});
