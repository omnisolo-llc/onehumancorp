import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import ApiDocsPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';
import { describe, it, expect, vi } from 'vitest';

vi.mock('swagger-ui-react', () => {
  return {
    default: () => <div data-testid="swagger-ui-mock">Swagger UI</div>
  };
});

describe('ApiDocsPage', () => {
  it('renders correctly', () => {
    render(
      <TooltipProvider>
        <ApiDocsPage />
      </TooltipProvider>
    );
    expect(screen.getByTestId('swagger-ui-mock')).toBeInTheDocument();
  });
});
