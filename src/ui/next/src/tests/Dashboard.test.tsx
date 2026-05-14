import { render, screen, fireEvent } from '@testing-library/react';
import { Dashboard } from '../components/Dashboard';

describe('Dashboard Component', () => {
  test('renders Dashboard title and stats', () => {
    render(<Dashboard />);
    expect(screen.getByText('Business Owner Dashboard')).toBeInTheDocument();
    expect(screen.getByText('Revenue')).toBeInTheDocument();
    expect(screen.getByText('$1,200')).toBeInTheDocument();
    expect(screen.getByText('Orders')).toBeInTheDocument();
    expect(screen.getByText('45')).toBeInTheDocument();
    expect(screen.getByText('Active Customers')).toBeInTheDocument();
    expect(screen.getByText('120')).toBeInTheDocument();
  });

  test('renders AI Agent Status', () => {
    render(<Dashboard />);
    expect(screen.getByText('AI Agent Status')).toBeInTheDocument();
    expect(screen.getByText('✅ Support Agent replied to 3 customers')).toBeInTheDocument();
    expect(screen.getByText('📦 Order Manager updated stock for 12 items')).toBeInTheDocument();
  });

  test('handles drill down for Revenue', () => {
    render(<Dashboard />);
    fireEvent.click(screen.getByText('Revenue'));
    expect(screen.getByText('Drill down view for Revenue showing details...')).toBeInTheDocument();
  });

  test('handles drill down for Orders', () => {
    render(<Dashboard />);
    fireEvent.click(screen.getByText('Orders'));
    expect(screen.getByText('Drill down view for Orders showing details...')).toBeInTheDocument();
  });

  test('handles drill down for Active Customers', () => {
    render(<Dashboard />);
    fireEvent.click(screen.getByText('Active Customers'));
    expect(screen.getByText('Drill down view for Active Customers showing details...')).toBeInTheDocument();
  });
});
