import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget28 from '../MemoryWidget28';

describe('MemoryWidget28', () => {
    it('renders correctly', () => {
        render(<MemoryWidget28 id="test-28" value={5.5} />);
        expect(screen.getByText('Widget 28 - test-28')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
