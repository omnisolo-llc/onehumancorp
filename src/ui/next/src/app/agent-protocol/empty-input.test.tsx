import {act,fireEvent,render,screen,waitFor} from '@testing-library/react';
import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import Page from './page';
const task={task_id:'actual-task',input:'Review my request'};
beforeEach(()=>{let created=false;vi.stubGlobal('fetch',vi.fn(async(_url,init)=>{if(init?.method==='POST'){created=true;return Response.json(task);}return Response.json({tasks:created?[task]:[],steps:[],checkpoints:[]});}));});afterEach(()=>vi.unstubAllGlobals());
it.each(['','  \n '])('does not enable or dispatch empty protocol input %j',async value=>{
 render(<Page/>);await act(async()=>{});fireEvent.change(screen.getByPlaceholderText('New Task Input...'),{target:{value}});const create=screen.getByRole('button',{name:'Create'});expect(create).toBeDisabled();fireEvent.click(create);expect(vi.mocked(fetch).mock.calls.filter(([,init])=>init?.method==='POST')).toHaveLength(0);
});
it('keeps valid protocol creation and its actual task receipt',async()=>{
 render(<Page/>);await act(async()=>{});fireEvent.change(screen.getByPlaceholderText('New Task Input...'),{target:{value:task.input}});const create=screen.getByRole('button',{name:'Create'});expect(create).toBeEnabled();await act(async()=>fireEvent.click(create));
 await waitFor(()=>expect(vi.mocked(fetch).mock.calls.filter(([,init])=>init?.method==='POST')).toHaveLength(1));expect(JSON.parse(String(vi.mocked(fetch).mock.calls.find(([,init])=>init?.method==='POST')![1]?.body))).toEqual({method:'ap_create_task',params:{input:task.input}});expect(screen.getByPlaceholderText('New Task Input...')).toHaveValue('');expect(create).toBeDisabled();expect(screen.getByText(task.input)).toBeVisible();
});

it.each([{status:202,body:task},{status:200,body:{success:true}},{status:200,body:{...task,success:false}}])('retains task input until a complete terminal receipt: %j',async result=>{
 const original=vi.mocked(fetch).getMockImplementation()!;vi.mocked(fetch).mockImplementation((url,init)=>init?.method==='POST'?Promise.resolve(Response.json(result.body,{status:result.status})):original(url,init));render(<Page/>);await act(async()=>{});fireEvent.change(screen.getByPlaceholderText('New Task Input...'),{target:{value:task.input}});await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Create'})));
 expect(screen.getByPlaceholderText('New Task Input...')).toHaveValue(task.input);expect(screen.getByText(/Task creation was not acknowledged/)).toBeVisible();expect(screen.getByRole('button',{name:'Create'})).toBeDisabled();fireEvent.click(screen.getByRole('button',{name:'Create'}));expect(vi.mocked(fetch).mock.calls.filter(([,init])=>init?.method==='POST')).toHaveLength(1);
});

it('holds a lost task-creation reply and permits only read-only task-list review',async()=>{
 const original=vi.mocked(fetch).getMockImplementation()!;vi.mocked(fetch).mockImplementation((url,init)=>init?.method==='POST'?Promise.reject(new Error('Connection lost')):original(url,init));render(<Page/>);await act(async()=>{});fireEvent.change(screen.getByPlaceholderText('New Task Input...'),{target:{value:task.input}});await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Create'})));
 expect(screen.getByRole('button',{name:'Create'})).toBeDisabled();await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Review existing tasks'})));expect(screen.getByRole('button',{name:'Create'})).toBeDisabled();expect(screen.getByPlaceholderText('New Task Input...')).toHaveValue(task.input);expect(vi.mocked(fetch).mock.calls.filter(([,init])=>init?.method==='POST')).toHaveLength(1);
});
it.each([401,403])('retains an explicit pre-effect auth rejection without an unknown-task claim: %s',async status=>{
 const original=vi.mocked(fetch).getMockImplementation()!;vi.mocked(fetch).mockImplementation((url,init)=>init?.method==='POST'?Promise.resolve(new Response('',{status})):original(url,init));render(<Page/>);await act(async()=>{});fireEvent.change(screen.getByPlaceholderText('New Task Input...'),{target:{value:task.input}});await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Create'})));expect(screen.getByPlaceholderText('New Task Input...')).toHaveValue(task.input);expect(screen.queryByRole('button',{name:'Review existing tasks'})).toBeNull();expect(screen.getByRole('button',{name:'Create'})).toBeEnabled();
});
