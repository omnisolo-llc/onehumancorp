import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget31 from '../MemoryWidget31';

describe('MemoryWidget31', () => {
    it('renders correctly', () => {
        render(<MemoryWidget31 id="test-31" value={5.5} />);
        expect(screen.getByText('Widget 31 - test-31')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
