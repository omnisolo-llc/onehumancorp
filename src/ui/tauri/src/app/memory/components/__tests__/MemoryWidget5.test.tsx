import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget5 from '../MemoryWidget5';

describe('MemoryWidget5', () => {
    it('renders correctly', () => {
        render(<MemoryWidget5 id="test-5" value={5.5} />);
        expect(screen.getByText('Widget 5 - test-5')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
