import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ApiDocsPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

// Mock SwaggerUI to avoid running an actual parser in tests
vi.mock('swagger-ui-react', () => {
  return {
    default: () => <div data-testid="swagger-ui-mock">Mocked Swagger UI</div>
  };
});

describe('ApiDocsPage', () => {
  it('renders the advanced warning and swagger ui mock', () => {
    render(
      <TooltipProvider>
        <ApiDocsPage />
      </TooltipProvider>
    );

    expect(screen.getByText('Advanced:')).toBeInTheDocument();
    expect(screen.getByText('This section is for developers directly integrating with our APIs. Not required for normal use.')).toBeInTheDocument();
    expect(screen.getByTestId('swagger-ui-mock')).toBeInTheDocument();
  });
});
