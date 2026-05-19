import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { SmartBlock } from '../components';

describe('SmartBlock', () => {
  it('renders Hero block', () => {
    render(<SmartBlock type="Hero" props={{ headline: 'Hero Title', copy: 'Hero Copy' }} />);
    expect(screen.getByText('Hero Title')).toBeInTheDocument();
    expect(screen.getByText('Hero Copy')).toBeInTheDocument();
  });

  it('renders Catalog block', () => {
    render(<SmartBlock type="Catalog" props={{ items: [{ name: 'Item 1', price: '$10', description: 'Desc 1' }] }} />);
    expect(screen.getByText('Our Services')).toBeInTheDocument();
    expect(screen.getByText('Item 1')).toBeInTheDocument();
    expect(screen.getByText('$10')).toBeInTheDocument();
    expect(screen.getByText('Desc 1')).toBeInTheDocument();
  });

  it('renders Booking block', () => {
    render(<SmartBlock type="Booking" props={{ title: 'Book Now', availability: 'Next available: Tomorrow' }} />);
    expect(screen.getByText('Book Now')).toBeInTheDocument();
    expect(screen.getByText('Next available: Tomorrow')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Select Time' })).toBeInTheDocument();
  });

  it('renders Contact block', () => {
    render(<SmartBlock type="Contact" props={{ email: 'test@example.com', phone: '123-456' }} />);
    expect(screen.getByText('Get in Touch')).toBeInTheDocument();
    expect(screen.getByText('test@example.com')).toBeInTheDocument();
    expect(screen.getByText('123-456')).toBeInTheDocument();
  });

  it('returns null for unknown type', () => {
    const { container } = render(<SmartBlock type="Unknown" props={{}} />);
    expect(container).toBeEmptyDOMElement();
  });
});
