import React from 'react';
import { render, screen, fireEvent, waitFor, cleanup, act } from '@testing-library/react';
import { afterEach, beforeEach, expect, test, vi } from 'vitest';
import Page from './page';
const fetcher=vi.fn();
const empty={tasks:[],alerts:[],staff:[]};
const actual={tasks:[{id:'t',title:'Recorded task',status:'PENDING'}],alerts:[{id:'a',message:'Recorded manager summary',severity:'info'}],staff:[{id:'s',name:'Recorded staff member',role:'Manager'}]};
beforeEach(()=>{fetcher.mockReset();vi.stubGlobal('fetch',fetcher);});
afterEach(()=>{cleanup();vi.unstubAllGlobals();});
test.each([401,503])('failed read%s stays unavailable and never inserts sample business facts',async status=>{
 fetcher.mockResolvedValue(Response.json({error:'unavailable'},{status}));render(<Page/>);
 await waitFor(()=>expect(screen.getByRole('alert')).toHaveTextContent('Location information could not be loaded'));
 for(const text of ['Alice','Restock coffee beans','Fix receipt printer','3 customer complaints','Location A'])expect(screen.queryByText(text,{exact:false})).not.toBeInTheDocument();
});
test('verified empty data stays empty with no invented active staff or tasks',async()=>{
 fetcher.mockResolvedValue(Response.json(empty));render(<Page/>);await waitFor(()=>expect(fetcher).toHaveBeenCalledOnce());
 expect(await screen.findByText('No recorded staff summaries.')).toBeVisible();expect(screen.queryByText('Alice')).not.toBeInTheDocument();
});
test('draft failure is visible, does not invent a complaint, and retains the recorded summary',async()=>{
 fetcher.mockResolvedValueOnce(Response.json(actual)).mockResolvedValueOnce(Response.json({error:'unavailable'},{status:503}));render(<Page/>);
 fireEvent.click(await screen.findByRole('button',{name:'Escalate to Owner'}));
 await waitFor(()=>expect(screen.getByRole('alert')).toHaveTextContent('No escalation draft was confirmed'));
 expect(screen.getByRole('textbox',{name:'Escalation draft'})).toHaveValue('');expect(screen.getByText('Recorded manager summary')).toBeVisible();expect(screen.getByRole('button',{name:'Send to Owner'})).toBeDisabled();
});
test('unavailable send never removes the summary or discards a reviewed draft',async()=>{
 fetcher.mockResolvedValueOnce(Response.json(actual)).mockResolvedValueOnce(Response.json({draft:'Draft from supplied summary'})).mockResolvedValueOnce(Response.json({error:'not implemented'},{status:501}));render(<Page/>);
 fireEvent.click(await screen.findByRole('button',{name:'Escalate to Owner'}));
 await waitFor(()=>expect(screen.getByRole('textbox',{name:'Escalation draft'})).toHaveValue('Draft from supplied summary'));
 fireEvent.click(screen.getByRole('button',{name:'Send to Owner'}));
 await waitFor(()=>expect(screen.getByRole('alert')).toHaveTextContent('No escalation was confirmed'));
 expect(screen.getByText('Recorded manager summary')).toBeVisible();expect(screen.getByRole('textbox',{name:'Escalation draft'})).toHaveValue('Draft from supplied summary');
});
test('cancelled generation cannot reopen a modal or install its late draft',async()=>{
 let complete!:(value:Response)=>void;const pending=new Promise<Response>(r=>{complete=r;});
 fetcher.mockResolvedValueOnce(Response.json(actual)).mockReturnValueOnce(pending);render(<Page/>);fireEvent.click(await screen.findByRole('button',{name:'Escalate to Owner'}));
 fireEvent.click(screen.getByRole('button',{name:'Cancel'}));await act(async()=>complete(Response.json({draft:'Late unwanted draft'})));
 expect(screen.queryByRole('dialog')).not.toBeInTheDocument();expect(screen.queryByText('Late unwanted draft')).not.toBeInTheDocument();
});
