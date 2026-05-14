import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget2 from '../MemoryWidget2';

describe('MemoryWidget2', () => {
    it('renders correctly', () => {
        render(<MemoryWidget2 id="test-2" value={5.5} />);
        expect(screen.getByText('Widget 2 - test-2')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
