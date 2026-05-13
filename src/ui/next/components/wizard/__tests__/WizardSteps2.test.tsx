
import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import PaymentStep from '../PaymentStep';
import AdminStep from '../AdminStep';
import TemplateStep from '../TemplateStep';
import DomainStep from '../DomainStep';
import LaunchStep from '../LaunchStep';

describe('More Wizard Steps Components', () => {
  it('PaymentStep handles selection', () => {
    const mockUpdate = jest.fn();
    const state = { payment: '' } as any;
    render(<PaymentStep state={state} update={mockUpdate} next={jest.fn()} prev={jest.fn()} />);

    fireEvent.click(screen.getByText('Online only'));
    expect(mockUpdate).toHaveBeenCalledWith('payment', 'Online only');
  });

  it('AdminStep updates admin fields', () => {
    const mockUpdate = jest.fn();
    const state = { adminName: '', adminEmail: '', adminPass: '' } as any;
    render(<AdminStep state={state} update={mockUpdate} next={jest.fn()} prev={jest.fn()} />);

    fireEvent.change(screen.getByPlaceholderText('Name'), { target: { value: 'Admin' } });
    expect(mockUpdate).toHaveBeenCalledWith('adminName', 'Admin');

    fireEvent.change(screen.getByPlaceholderText('you@email.com'), { target: { value: 'a@a.com' } });
    expect(mockUpdate).toHaveBeenCalledWith('adminEmail', 'a@a.com');

    fireEvent.change(screen.getByPlaceholderText('Password'), { target: { value: 'pass' } });
    expect(mockUpdate).toHaveBeenCalledWith('adminPass', 'pass');
  });

  it('TemplateStep handles template selection', () => {
    const mockUpdate = jest.fn();
    const state = { template: '' } as any;
    render(<TemplateStep state={state} update={mockUpdate} next={jest.fn()} prev={jest.fn()} />);

    fireEvent.click(screen.getByText('Use this template Modern'));
    expect(mockUpdate).toHaveBeenCalledWith('template', 'Modern');
  });

  it('DomainStep handles domain choice', () => {
    const mockUpdate = jest.fn();
    const state = { domain: '' } as any;
    render(<DomainStep state={state} update={mockUpdate} next={jest.fn()} prev={jest.fn()} />);

    fireEvent.click(screen.getByText('Buy a domain'));
    expect(mockUpdate).toHaveBeenCalledWith('domain', 'Buy a domain');
  });

  it('LaunchStep triggers launch correctly', () => {
    const mockLaunch = jest.fn();
    const state = { name: 'My Biz', agents: [], businessType: 'Store', template: 'Modern', payment: 'Online' } as any;
    render(<LaunchStep state={state} launch={mockLaunch} prev={jest.fn()} launching={false} launched={false} />);

    fireEvent.click(screen.getByRole('button', { name: /Launch My Business/ }));
    expect(mockLaunch).toHaveBeenCalledTimes(1);
  });

  it('LaunchStep shows loading state', () => {
    const mockLaunch = jest.fn();
    const state = { name: 'My Biz', agents: [], businessType: 'Store', template: 'Modern', payment: 'Online' } as any;
    render(<LaunchStep state={state} launch={mockLaunch} prev={jest.fn()} launching={true} launched={false} />);

    expect(screen.getByRole('button', { name: /Provisioning your tenant/ })).toBeDisabled();
  });

  it('LaunchStep shows launched state', () => {
    const mockLaunch = jest.fn();
    const state = { name: 'My Biz', agents: [], businessType: 'Store', template: 'Modern', payment: 'Online' } as any;
    render(<LaunchStep state={state} launch={mockLaunch} prev={jest.fn()} launching={false} launched={true} />);

    expect(screen.getByText('Onboarding Complete!')).toBeInTheDocument();
  });
});
