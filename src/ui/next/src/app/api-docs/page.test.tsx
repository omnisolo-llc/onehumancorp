import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import ApiDocsPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

// Mock SwaggerUI to avoid running an actual parser in tests
vi.mock('swagger-ui-react', () => {
  return {
    default: () => <div data-testid="swagger-ui-mock">Mocked Swagger UI</div>
  };
});

describe('ApiDocsPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/docs/spec') {
        return Promise.resolve({
          json: () => Promise.resolve({
            openapi: "3.0.0",
            info: { title: "Test API" },
            servers: [{ url: "http://localhost:8080" }]
          })
        });
      }
      return Promise.resolve({ json: () => Promise.resolve({}) });
    }) as any;
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders the advanced warning and swagger ui mock after fetching data', async () => {
    render(
      <TooltipProvider>
        <ApiDocsPage />
      </TooltipProvider>
    );

    expect(screen.getByText('Advanced:')).toBeInTheDocument();
    expect(screen.getByText(/This section is for developers directly integrating with our APIs/)).toBeInTheDocument();

    // Wait for the swagger UI mock to appear after data load
    await waitFor(() => {
      expect(screen.getByTestId('swagger-ui-mock')).toBeInTheDocument();
    });
  });
});
