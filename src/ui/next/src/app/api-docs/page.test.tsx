import '@testing-library/jest-dom';
import * as React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ApiDocsPage from './page';

// Mock SwaggerUI to avoid running an actual parser in tests
vi.mock('swagger-ui-react', () => {
  return {
    default: (props: any) => {
      const spec = props.spec;
      return (
        <div data-testid="swagger-ui-mock">
          Mocked Swagger UI - {spec && spec.info ? spec.info.title : ''}
        </div>
      );
    }
  };
});

describe('ApiDocsPage', () => {
  it('renders the advanced warning and swagger ui mock', async () => {
    // Override window.location
    Object.defineProperty(window, 'location', {
      value: {
        origin: 'http://localhost:3000'
      },
      writable: true
    });

    render(<ApiDocsPage />);

    expect(screen.getByText('Advanced:')).toBeInTheDocument();
    expect(
      screen.getByText('This section is for developers directly integrating with our APIs. Not required for normal use.')
    ).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByTestId('swagger-ui-mock')).toBeInTheDocument();
    });

    expect(screen.getByText('Mocked Swagger UI - OHC Advanced API Reference')).toBeInTheDocument();
  });
});
