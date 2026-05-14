import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget11 from '../MemoryWidget11';

describe('MemoryWidget11', () => {
    it('renders correctly', () => {
        render(<MemoryWidget11 id="test-11" value={5.5} />);
        expect(screen.getByText('Widget 11 - test-11')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
