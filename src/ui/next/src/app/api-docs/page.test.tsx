import { render, screen, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import ApiDocsPage from './page';
import { describe, it, expect, vi } from 'vitest';

// Mock SwaggerUI since it does not render well in jsdom without issues
vi.mock('swagger-ui-react', () => {
  return {
    default: () => <div data-testid="swagger-ui-mock">SwaggerUI</div>
  };
});

describe('ApiDocsPage', () => {
  it('renders the advanced warning banner', () => {
    render(<ApiDocsPage />);
    expect(screen.getByText(/This section is for developers directly integrating with our APIs/)).toBeInTheDocument();
  });

  it('renders the SwaggerUI component after mounting', async () => {
    render(<ApiDocsPage />);
    await waitFor(() => {
      expect(screen.getByTestId('swagger-ui-mock')).toBeInTheDocument();
    });
  });
});
