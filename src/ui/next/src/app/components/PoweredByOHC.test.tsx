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
    expect(linkElement).toHaveAttribute('href', `https://ohc.store/join?ref=${testTenantId}&source=footer_widget`);
    expect(linkElement).toHaveAttribute('target', '_blank');
    expect(linkElement).toHaveAttribute('rel', 'noopener noreferrer');
  });
});
