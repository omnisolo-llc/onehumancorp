import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import BrandColorsStep from '../BrandColorsStep';
import FirstProductStep from '../FirstProductStep';
import AgentStep from '../AgentStep';
import AgentScheduleStep from '../AgentScheduleStep';
import PromptTuningStep from '../PromptTuningStep';
import WizardPage from '../../app/wizard/page';

describe('Remaining Wizard Components', () => {
  it('BrandColorsStep works', () => {
    const mockUpdate = jest.fn();
    const state = { colors: [] } as any;
    render(<BrandColorsStep state={state} update={mockUpdate} next={jest.fn()} prev={jest.fn()} />);
    fireEvent.click(screen.getByText('✨ AI Background Removal'));
  });

  it('FirstProductStep works', () => {
    const mockUpdate = jest.fn();
    const state = { products: [] } as any;
    render(<FirstProductStep state={state} update={mockUpdate} next={jest.fn()} prev={jest.fn()} />);
    fireEvent.change(screen.getByPlaceholderText('Product Name'), { target: { value: 'Test' } });
    expect(mockUpdate).toHaveBeenCalled();
  });

  it('AgentStep works', () => {
    const mockUpdate = jest.fn();
    const state = { agents: [] } as any;
    render(<AgentStep state={state} update={mockUpdate} next={jest.fn()} prev={jest.fn()} />);
    fireEvent.click(screen.getByLabelText('Customer Support'));
    expect(mockUpdate).toHaveBeenCalled();
  });

  it('AgentScheduleStep works', () => {
    const mockUpdate = jest.fn();
    const state = { agentSchedule: 1 } as any;
    render(<AgentScheduleStep state={state} update={mockUpdate} next={jest.fn()} prev={jest.fn()} />);
    fireEvent.change(screen.getByRole('slider'), { target: { value: '2' } });
    expect(mockUpdate).toHaveBeenCalled();
  });

  it('PromptTuningStep works', () => {
    const mockUpdate = jest.fn();
    const state = { agentTone: '', agentFocus: [] } as any;
    render(<PromptTuningStep state={state} update={mockUpdate} next={jest.fn()} prev={jest.fn()} />);
    fireEvent.change(screen.getByRole('combobox'), { target: { value: 'Professional' } });
    expect(mockUpdate).toHaveBeenCalledWith('agentTone', 'Professional');
  });

  it('WizardPage wrapper renders', () => {
    render(<WizardPage />);
    expect(screen.getByText('Advanced Mode')).toBeInTheDocument();
  });
});