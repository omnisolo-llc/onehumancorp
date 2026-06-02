import { render, screen, waitFor } from '@testing-library/react';
import ArticlePage from './page';
import { expect, test, vi, beforeEach } from 'vitest';

vi.mock('next/link', () => ({
  default: ({ children, href }: any) => <a href={href}>{children}</a>,
}));

beforeEach(() => {
  vi.restoreAllMocks();
});

test('renders article loading state', () => {
  vi.stubGlobal('fetch', vi.fn(() => new Promise(() => {})));

  render(<ArticlePage params={{ id: 'getting-started' }} />);
  expect(screen.getByText('Loading...')).toBeInTheDocument();
});

test('renders article error state', async () => {
  vi.stubGlobal('fetch', vi.fn(() => Promise.resolve({
    ok: false
  })));

  render(<ArticlePage params={{ id: 'does-not-exist' }} />);

  await waitFor(() => {
    expect(screen.getByText('Article not found')).toBeInTheDocument();
  });

  expect(screen.getByRole('link', { name: /Back to Help Center/i })).toBeInTheDocument();
});

test('renders article content', async () => {
  vi.stubGlobal('fetch', vi.fn(() => Promise.resolve({
    ok: true,
    json: () => Promise.resolve({
      title: 'Test Article Title',
      contentHtml: '<p>This is the test article content.</p>'
    })
  })));

  render(<ArticlePage params={{ id: 'test-article' }} />);

  await waitFor(() => {
    expect(screen.getByRole('heading', { name: 'Test Article Title' })).toBeInTheDocument();
  });

  expect(screen.getByText('This is the test article content.')).toBeInTheDocument();
});
