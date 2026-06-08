import React from 'react';
import { render, screen } from '@testing-library/react';
import { PoweredByOHC } from './PoweredByOHC';
import { describe, it, expect } from 'vitest';
import '@testing-library/jest-dom';

describe('PoweredByOHC Component', () => {
  it('renders correctly with the provided tenantId', () => {
    const testTenantId = 'test-org-123';
    render(<PoweredByOHC tenantId={testTenantId} />);

    const linkElement = screen.getByRole('link', { name: /powered by ohc/i });

    expect(linkElement).toBeInTheDocument();
    expect(linkElement).toHaveAttribute('href', `/api/v1/growth/referrals/click?target=/onboarding&ref=${testTenantId}&source=footer_widget`);
    expect(linkElement).not.toHaveAttribute('target');
    expect(linkElement).not.toHaveAttribute('rel');
  });
});
