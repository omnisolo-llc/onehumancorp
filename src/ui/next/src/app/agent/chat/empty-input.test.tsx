import {fireEvent,render,screen} from '@testing-library/react';
import {expect,it,vi} from 'vitest';
import Page from './page';
it.each(['','  \n '])('holds an empty accountant query %j without fabricated replies',value=>{
 render(<Page/>);fireEvent.change(screen.getByPlaceholderText(/financial query/),{target:{value}});const send=screen.getByRole('button',{name:'Send message'});expect(send).toBeDisabled();fireEvent.click(send);expect(screen.queryByText('A verified balance is unavailable here.',{exact:false})).toBeNull();
});
it('enables a real query and keeps the truthful read-only response',()=>{
 const fetch=vi.fn();vi.stubGlobal('fetch',fetch);render(<Page/>);fireEvent.change(screen.getByPlaceholderText(/financial query/),{target:{value:'What is my balance?'}});const send=screen.getByRole('button',{name:'Send message'});expect(send).toBeEnabled();fireEvent.click(send);expect(screen.getByText(/A verified balance is unavailable here/)).toBeVisible();expect(send).toBeDisabled();expect(fetch).not.toHaveBeenCalled();vi.unstubAllGlobals();
});
