import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget25 from '../MemoryWidget25';

describe('MemoryWidget25', () => {
    it('renders correctly', () => {
        render(<MemoryWidget25 id="test-25" value={5.5} />);
        expect(screen.getByText('Widget 25 - test-25')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
