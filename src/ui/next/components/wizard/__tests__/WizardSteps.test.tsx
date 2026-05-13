
import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import WelcomeStep from '../WelcomeStep';
import BusinessTypeStep from '../BusinessTypeStep';
import CompanyNameStep from '../CompanyNameStep';
import SellingStep from '../SellingStep';

describe('Wizard Steps Components', () => {
  it('WelcomeStep renders correctly and triggers next', () => {
    const mockNext = jest.fn();
    render(<WelcomeStep next={mockNext} />);
    expect(screen.getByText('Your business, live in minutes')).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: /Launch My Business/i }));
    expect(mockNext).toHaveBeenCalledTimes(1);
  });

  it('BusinessTypeStep renders options and handles selection', () => {
    const mockUpdate = jest.fn();
    const mockNext = jest.fn();
    const mockPrev = jest.fn();
    const state = { businessType: '' } as any;

    render(<BusinessTypeStep state={state} update={mockUpdate} next={mockNext} prev={mockPrev} />);

    expect(screen.getByText('What kind of business are you building?')).toBeInTheDocument();

    const storeBtn = screen.getByText('Online Store');
    fireEvent.click(storeBtn);
    expect(mockUpdate).toHaveBeenCalledWith('businessType', 'Online Store');

    expect(screen.getByRole('button', { name: 'Next' })).toBeDisabled();

    fireEvent.click(screen.getByRole('button', { name: 'Back' }));
    expect(mockPrev).toHaveBeenCalledTimes(1);
  });

  it('CompanyNameStep renders inputs and handles updates', () => {
    const mockUpdate = jest.fn();
    const state = { name: '', desc: '' } as any;

    render(<CompanyNameStep state={state} update={mockUpdate} next={jest.fn()} prev={jest.fn()} />);

    const nameInput = screen.getByPlaceholderText("e.g. Maya's Cakes");
    fireEvent.change(nameInput, { target: { value: 'New Biz' } });
    expect(mockUpdate).toHaveBeenCalledWith('name', 'New Biz');

    const descInput = screen.getByPlaceholderText("Description...");
    fireEvent.change(descInput, { target: { value: 'A great place' } });
    expect(mockUpdate).toHaveBeenCalledWith('desc', 'A great place');
  });

  it('SellingStep handles multi-select logic', () => {
    const mockUpdate = jest.fn();
    const state = { sellingCats: ['Physical products'] } as any;

    render(<SellingStep state={state} update={mockUpdate} next={jest.fn()} prev={jest.fn()} />);

    // Check an unselected item
    const digitalCheck = screen.getByLabelText('Digital downloads');
    fireEvent.click(digitalCheck);
    expect(mockUpdate).toHaveBeenCalledWith('sellingCats', ['Physical products', 'Digital downloads']);

    // Uncheck a selected item
    const physicalCheck = screen.getByLabelText('Physical products');
    fireEvent.click(physicalCheck);
    expect(mockUpdate).toHaveBeenCalledWith('sellingCats', []);
  });
});
