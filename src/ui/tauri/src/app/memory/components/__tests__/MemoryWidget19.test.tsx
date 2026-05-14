import React from 'react';
import { render, screen } from '@testing-library/react';
import MemoryWidget19 from '../MemoryWidget19';

describe('MemoryWidget19', () => {
    it('renders correctly', () => {
        render(<MemoryWidget19 id="test-19" value={5.5} />);
        expect(screen.getByText('Widget 19 - test-19')).toBeInTheDocument();
        expect(screen.getByText('5.50')).toBeInTheDocument();
    });
});
