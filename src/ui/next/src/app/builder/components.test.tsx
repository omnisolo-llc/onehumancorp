import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { SmartBlock } from './components';

describe('SmartBlock PoweredBy', () => {
    it('renders the Powered by OHC footer when isPremium is false or undefined', () => {
        render(<SmartBlock type="PoweredBy" props={{ tenantId: 'test-tenant' }} />);
        expect(screen.getByText('⚡ Powered by')).toBeTruthy();
        expect(screen.getByText('OHC')).toBeTruthy();
        expect(screen.getByRole('link').getAttribute('href')).toBe('/onboarding?ref=test-tenant');
    });

    it('does not render when isPremium is true', () => {
        const { container } = render(<SmartBlock type="PoweredBy" props={{ tenantId: 'test-tenant', isPremium: true }} />);
        expect(container.firstChild).toBeNull();
    });
});
