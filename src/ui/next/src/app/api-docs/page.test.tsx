import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ApiDocsPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

vi.mock('swagger-ui-react', () => ({
  default: () => <div data-testid="swagger-ui">SwaggerUI Mock</div>
}));

describe('ApiDocsPage', () => {
  it('renders the advanced warning and swagger ui', () => {
    render(
      <TooltipProvider>
        <ApiDocsPage />
      </TooltipProvider>
    );
    expect(screen.getByText(/This section is for developers directly integrating with our APIs/i)).toBeInTheDocument();
    expect(screen.getByTestId('swagger-ui')).toBeInTheDocument();
  });
});
