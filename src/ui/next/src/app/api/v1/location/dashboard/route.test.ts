import { beforeEach, expect, test, vi } from 'vitest';
const proxy = vi.hoisted(() => vi.fn());
vi.mock('@/lib/auth/backendTransport', () => ({ proxyBackendRequest: proxy }));
import { GET } from './route';
const request = () => new Request('http://localhost/api/v1/location/dashboard');
beforeEach(() => { proxy.mockReset(); });
function replies(tasks:unknown={tasks:[]},summaries:unknown={summaries:[]},staff:unknown={staff:[]}) {
  proxy.mockResolvedValueOnce(Response.json(tasks)).mockResolvedValueOnce(Response.json(summaries)).mockResolvedValueOnce(Response.json(staff));
}
test('real empty arrays remain empty rather than invented staff/tasks/complaints',async()=>{
  replies();const result=await GET(request());expect(result.status).toBe(200);expect(await result.json()).toEqual({tasks:[],alerts:[],staff:[]});
});
test.each([401,403,404,500,503])('preserves failed authenticated read %s instead of successful sample data',async status=>{
  const failed=Response.json({error:'actual error'},{status,headers:{'cache-control':'private, no-store'}});
  proxy.mockResolvedValueOnce(failed).mockResolvedValueOnce(Response.json({summaries:[]})).mockResolvedValueOnce(Response.json({staff:[]}));
  expect(await GET(request())).toBe(failed);
});
test('malformed or contradictory successful data is unavailable, not invented empty success',async()=>{
  for(const payload of [{},{tasks:null},{tasks:[],success:false},{tasks:[],error:'denied'}]){
    replies(payload);const result=await GET(request());expect(result.status).toBe(502);expect(await result.json()).not.toHaveProperty('tasks');
  }
});
test('only returned rows appear and no staff shift status is fabricated',async()=>{
  replies({tasks:[{id:'recorded-task',description:'Count stock',status:'pending'}]},{summaries:[{id:'recorded-summary',summary_text:'Actual manager note'}]},{staff:[{id:'recorded-staff',name:'Actual member',role:'Manager'}]});
  const body=await (await GET(request())).json();expect(body.tasks).toEqual([{id:'recorded-task',title:'Count stock',status:'PENDING'}]);expect(body.alerts).toEqual([{id:'recorded-summary',message:'Actual manager note',severity:'info'}]);expect(body.staff).toEqual([{id:'recorded-staff',name:'Actual member',role:'Manager'}]);
});

test('preserves a recorded task title separately from its description',async()=>{
  replies({tasks:[{id:'recorded',title:'Count recorded stock',description:'Use the back room ledger',status:'pending'}]});
  const body=await (await GET(request())).json();expect(body.tasks[0].title).toBe('Count recorded stock');
});
