import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import '@testing-library/jest-dom';

vi.mock('swagger-ui-react', () => {
  return {
    default: () => <div data-testid="swagger-ui-mock">Mock Swagger UI</div>,
  };
});

import ApiDocsPage from './page';

describe('ApiDocsPage', () => {
  it('renders the advanced warning', () => {
    render(<ApiDocsPage />);
    expect(screen.getByText(/Advanced:/)).toBeInTheDocument();
    expect(screen.getByText(/This section is for developers directly integrating with our APIs/)).toBeInTheDocument();
  });
});