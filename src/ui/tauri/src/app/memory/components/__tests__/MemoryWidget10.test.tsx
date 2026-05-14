import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget10 from '../MemoryWidget10';

describe('MemoryWidget10', () => {
    it('renders correctly', () => {
        render(<MemoryWidget10 id="test-10" value={5.5} />);
        expect(screen.getByText('Widget 10 - test-10')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
