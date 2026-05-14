import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget26 from '../MemoryWidget26';

describe('MemoryWidget26', () => {
    it('renders correctly', () => {
        render(<MemoryWidget26 id="test-26" value={5.5} />);
        expect(screen.getByText('Widget 26 - test-26')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
