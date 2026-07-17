
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ApiDocsPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

// Mock SwaggerUI to avoid running an actual parser in tests
vi.mock('swagger-ui-react', () => {
  return {
    default: (props: any) => (
      <div data-testid="swagger-ui-mock">
        Mocked Swagger UI
        {props.spec?.paths?.['/api/v1/help'] && <span>HasHelpPath</span>}
        {props.spec?.paths?.['/api/v1/tooltips'] && <span>HasTooltipsPath</span>}
      </div>
    )
  };
});

describe('ApiDocsPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ paths: { '/api/v1/help': {}, '/api/v1/tooltips': {} } }),
      ok: true
    }) as any;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the advanced warning and swagger ui mock', async () => {
    render(
      <TooltipProvider>
        <ApiDocsPage />
      </TooltipProvider>
    );

    expect(screen.getByText('Advanced:')).toBeTruthy();
    expect(screen.getByText('This section is for developers directly integrating with our APIs. Not required for normal use.')).toBeTruthy();

    await waitFor(() => {
      expect(screen.getByTestId('swagger-ui-mock')).toBeTruthy();
    });

    expect(screen.getByText('HasHelpPath')).toBeTruthy();
    expect(screen.getByText('HasTooltipsPath')).toBeTruthy();
  });
});
