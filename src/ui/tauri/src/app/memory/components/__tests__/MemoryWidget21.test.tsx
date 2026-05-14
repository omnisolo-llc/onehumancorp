import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget21 from '../MemoryWidget21';

describe('MemoryWidget21', () => {
    it('renders correctly', () => {
        render(<MemoryWidget21 id="test-21" value={5.5} />);
        expect(screen.getByText('Widget 21 - test-21')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
