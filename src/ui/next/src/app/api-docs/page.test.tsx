import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import ApiDocsPage from './page';

// Mock SwaggerUI to avoid running an actual parser in tests
vi.mock('swagger-ui-react', () => {
  return {
    default: () => <div data-testid="swagger-ui-mock">Mocked Swagger UI</div>
  };
});

describe('ApiDocsPage', () => {
  beforeEach(() => {
    vi.spyOn(globalThis, 'fetch').mockImplementation(() =>
      Promise.resolve({
        json: () => Promise.resolve({
           servers: [{ url: "http://localhost" }]
        }),
      } as Response)
    );
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the advanced warning and swagger ui mock', async () => {
    render(<ApiDocsPage />);

    expect(screen.getByText('Advanced:')).toBeInTheDocument();
    expect(screen.getByText('This section is for developers directly integrating with our APIs. Not required for normal use.')).toBeInTheDocument();
    await waitFor(() => {
        expect(screen.getByTestId('swagger-ui-mock')).toBeInTheDocument();
    });
  });
});
