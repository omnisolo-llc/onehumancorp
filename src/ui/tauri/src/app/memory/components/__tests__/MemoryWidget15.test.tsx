import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget15 from '../MemoryWidget15';

describe('MemoryWidget15', () => {
    it('renders correctly', () => {
        render(<MemoryWidget15 id="test-15" value={5.5} />);
        expect(screen.getByText('Widget 15 - test-15')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
