import { render } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';

// Mock SwaggerUI to avoid CSS import issues with Vitest/Vite/Tailwind
vi.mock('swagger-ui-react', () => ({
  default: () => <div data-testid="mock-swagger-ui">Swagger UI</div>
}));

// Mock the CSS import to prevent PostCSS errors in test
vi.mock('swagger-ui-react/swagger-ui.css', () => ({}));

import ApiDocsPage from './page';

describe('ApiDocsPage', () => {
  it('renders without crashing', () => {
    const { getByTestId } = render(<ApiDocsPage />);
    expect(getByTestId('mock-swagger-ui')).toBeDefined();
  });
});
