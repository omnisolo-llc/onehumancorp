import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget3 from '../MemoryWidget3';

describe('MemoryWidget3', () => {
    it('renders correctly', () => {
        render(<MemoryWidget3 id="test-3" value={5.5} />);
        expect(screen.getByText('Widget 3 - test-3')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
