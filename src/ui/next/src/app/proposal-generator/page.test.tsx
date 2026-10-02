import { render,screen,fireEvent,waitFor } from '@testing-library/react';
import ProposalGeneratorPage from './page';
import { describe,it,expect,beforeEach,vi } from 'vitest';

describe('ProposalGeneratorPage', () => {
  beforeEach(() => {
    localStorage.clear();
  });

  it('renders initial state correctly', () => {
    render(<ProposalGeneratorPage />);
    expect(screen.getByText('Create Professional Proposal')).toBeTruthy();
  });

  it('generates a shareable proposal link on valid input', async () => {
    render(<ProposalGeneratorPage />);

    fireEvent.change(screen.getByPlaceholderText(/e.g. Acme Corp/i), { target: { value: 'Test Client' } });
    fireEvent.change(screen.getByPlaceholderText(/e.g. Website Redesign/i), { target: { value: 'Test Scope' } });
    fireEvent.change(screen.getByPlaceholderText(/e.g. 2500.00/i), { target: { value: '1000' } });
    fireEvent.change(screen.getByPlaceholderText(/e.g. 4-6 Weeks/i), { target: { value: '2 Weeks' } });

    const generateBtn = screen.getByText('Generate Shareable Proposal');
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(screen.getByText('Your Proposal is Ready!')).toBeTruthy();
    });
  });
});

it('holds an incomplete proposal with persistent guidance instead of a browser alert',()=>{
 const alert=vi.spyOn(window,'alert').mockImplementation(()=>{});render(<ProposalGeneratorPage/>);
 const button=screen.getByRole('button',{name:'Generate Shareable Proposal'});expect(button).toBeDisabled();expect(screen.getByRole('status',{name:'Proposal requirements'})).toHaveTextContent(/client.*scope.*amount.*timeline/i);
 fireEvent.click(button);expect(alert).not.toHaveBeenCalled();expect(screen.queryByText('Your Proposal is Ready!')).not.toBeInTheDocument();alert.mockRestore();
});
it('keeps whitespace fields and negative amounts ineligible, then preserves exact valid proposal data',()=>{
 render(<ProposalGeneratorPage/>);const button=screen.getByRole('button',{name:'Generate Shareable Proposal'});
 fireEvent.change(screen.getByPlaceholderText(/e.g. Acme Corp/i),{target:{value:'   '}});fireEvent.change(screen.getByPlaceholderText(/e.g. Website Redesign/i),{target:{value:'Actual scope'}});fireEvent.change(screen.getByPlaceholderText(/e.g. 2500.00/i),{target:{value:'120'}});fireEvent.change(screen.getByPlaceholderText(/e.g. 4-6 Weeks/i),{target:{value:'2 Weeks'}});expect(button).toBeDisabled();
 fireEvent.change(screen.getByPlaceholderText(/e.g. Acme Corp/i),{target:{value:'Actual client'}});fireEvent.change(screen.getByPlaceholderText(/e.g. 2500.00/i),{target:{value:'-5'}});expect(button).toBeDisabled();
 fireEvent.change(screen.getByPlaceholderText(/e.g. 2500.00/i),{target:{value:'0'}});expect(button).toBeEnabled();fireEvent.click(button);
 const encoded=new URL(screen.getByRole('link',{name:'Preview Proposal'}).getAttribute('href')!).searchParams.get('data')!;const value=JSON.parse(atob(encoded.replace(/-/g,'+').replace(/_/g,'/')));expect(value).toMatchObject({clientName:'Actual client',projectScope:'Actual scope',amount:'0',timeline:'2 Weeks'});
});
