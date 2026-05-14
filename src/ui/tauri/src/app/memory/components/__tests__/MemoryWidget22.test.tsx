import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget22 from '../MemoryWidget22';

describe('MemoryWidget22', () => {
    it('renders correctly', () => {
        render(<MemoryWidget22 id="test-22" value={5.5} />);
        expect(screen.getByText('Widget 22 - test-22')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
