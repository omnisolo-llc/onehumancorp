import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget18 from '../MemoryWidget18';

describe('MemoryWidget18', () => {
    it('renders correctly', () => {
        render(<MemoryWidget18 id="test-18" value={5.5} />);
        expect(screen.getByText('Widget 18 - test-18')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
