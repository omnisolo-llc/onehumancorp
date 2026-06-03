import React from 'react';
import { render, screen } from '@testing-library/react';
import UpsellInsightsCard from './UpsellInsightsCard';
import { describe, it, expect } from 'vitest';

describe('UpsellInsightsCard', () => {
    it('renders the Upsell Insights Card correctly', () => {
        render(<UpsellInsightsCard />);

        expect(screen.getByText('AI Upsell Revenue')).toBeInTheDocument();
        expect(screen.getByText('$845.00')).toBeInTheDocument();
        expect(screen.getByText('Top performing bundle:')).toBeInTheDocument();
    });
});
