import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget12 from '../MemoryWidget12';

describe('MemoryWidget12', () => {
    it('renders correctly', () => {
        render(<MemoryWidget12 id="test-12" value={5.5} />);
        expect(screen.getByText('Widget 12 - test-12')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
