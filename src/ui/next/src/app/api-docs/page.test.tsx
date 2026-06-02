import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ApiDocsPage from './page';

vi.mock('swagger-ui-react', () => {
  return {
    default: () => <div data-testid="swagger-ui-mock">Mocked Swagger UI</div>
  };
});

describe('ApiDocsPage', () => {
  it('renders the advanced warning and swagger ui mock', () => {
    render(<ApiDocsPage />);

    expect(screen.getByText('Advanced:')).toBeInTheDocument();
    expect(screen.getByTestId('swagger-ui-mock')).toBeInTheDocument();
  });
});
