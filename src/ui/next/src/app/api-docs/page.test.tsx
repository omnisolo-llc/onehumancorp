import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ApiDocsPage from './page';

// Mock dynamic import
vi.mock('next/dynamic', () => ({
  default: () => {
    return function MockedSwagger() {
      return <div data-testid="swagger-ui-mock">Mocked Swagger UI</div>;
    };
  },
}));

describe('ApiDocsPage', () => {
  it('renders the API documentation title and swagger ui mock', () => {
    render(<ApiDocsPage />);

    expect(screen.getByText('API Documentation')).toBeInTheDocument();
    expect(screen.getByText(/These endpoints allow you to build custom integrations./i)).toBeInTheDocument();
    expect(screen.getByTestId('swagger-ui-mock')).toBeInTheDocument();
  });
});
