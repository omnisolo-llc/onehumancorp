import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget23 from '../MemoryWidget23';

describe('MemoryWidget23', () => {
    it('renders correctly', () => {
        render(<MemoryWidget23 id="test-23" value={5.5} />);
        expect(screen.getByText('Widget 23 - test-23')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
