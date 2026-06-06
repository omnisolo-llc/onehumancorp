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

  it('applies custom className correctly', () => {
    const testTenantId = 'test-org-123';
    const customClass = 'my-custom-test-class';
    const { container } = render(<PoweredByOHC tenantId={testTenantId} className={customClass} />);

    // container.firstChild is the outer div
    expect(container.firstChild).toHaveClass(customClass);
    expect(container.firstChild).toHaveClass('flex justify-center items-center mt-8 pb-4');
  });
});
