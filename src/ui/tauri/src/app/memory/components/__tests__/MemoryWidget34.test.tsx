import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget34 from '../MemoryWidget34';

describe('MemoryWidget34', () => {
    it('renders correctly', () => {
        render(<MemoryWidget34 id="test-34" value={5.5} />);
        expect(screen.getByText('Widget 34 - test-34')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
