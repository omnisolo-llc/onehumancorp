import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ApiDocsPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

// Mock SwaggerUI to avoid running an actual parser in tests
vi.mock('swagger-ui-react', () => {
  return {
    default: (props: any) => (
      <div data-testid="swagger-ui-mock">
        Mocked Swagger UI
        {props.spec?.paths?.['/api/help'] && <span>HasHelpPath</span>}
        {props.spec?.paths?.['/api/tooltips'] && <span>HasTooltipsPath</span>}
      </div>
    )
  };
});

describe('ApiDocsPage', () => {
  it('renders the advanced warning and swagger ui mock', async () => {
    global.fetch = vi.fn().mockImplementation(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({
          openapi: "3.0.0",
          paths: {
            '/api/help': {},
            '/api/tooltips': {}
          }
        })
      })
    );

    render(
      <TooltipProvider>
        <ApiDocsPage />
      </TooltipProvider>
    );

    expect(screen.getByText('Advanced:')).toBeInTheDocument();
    expect(screen.getByText('This section is for developers directly integrating with our APIs. Not required for normal use.')).toBeInTheDocument();

    await screen.findByTestId('swagger-ui-mock');

    expect(screen.getByTestId('swagger-ui-mock')).toBeInTheDocument();
    expect(screen.getByText('HasHelpPath')).toBeInTheDocument();
    expect(screen.getByText('HasTooltipsPath')).toBeInTheDocument();
  });
});
