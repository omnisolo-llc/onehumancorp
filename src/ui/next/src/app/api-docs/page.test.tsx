import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ApiDocsPage from './page';

vi.mock('swagger-ui-react', () => {
  return {
    default: () => <div data-testid="swagger-ui">Mock Swagger UI</div>,
  };
});

describe('ApiDocsPage', () => {
  it('renders the advanced warning', () => {
    render(<ApiDocsPage />);
    expect(screen.getByText(/Advanced:/)).toBeInTheDocument();
  });

  it('renders SwaggerUI after mount', () => {
    render(<ApiDocsPage />);
    expect(screen.getByTestId('swagger-ui')).toBeInTheDocument();
  });
});
