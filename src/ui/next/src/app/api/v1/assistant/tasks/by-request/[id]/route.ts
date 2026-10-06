import { proxyBackendRequest } from '@/lib/auth/backendTransport';
import { privateJson } from '../../../assistantBackend';
import { toUiTask } from '../../taskContract';
export async function GET(request:Request,context:{params:Promise<{id:string}>}):Promise<Response>{
  const id=(await context.params).id;
  if(!/^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$/.test(id))return privateJson(400,{error:'invalid request identity'});
  const response=await proxyBackendRequest(request,`/api/v1/assistant/tasks/by-request/${id}`);
  if(!response.ok)return response;
  const payload:unknown=await response.json().catch(()=>null);
  if (payload && typeof payload==='object' && !Array.isArray(payload) && 'cancelled_request' in payload && 'effect' in payload && payload.effect==='none') {
    const cancelled=payload.cancelled_request;
    if (cancelled && typeof cancelled==='object' && 'request_id' in cancelled && cancelled.request_id===id && 'phase' in cancelled && cancelled.phase==='cancelled' && 'tenant_id' in cancelled && typeof cancelled.tenant_id==='string' && 'actor_id' in cancelled && typeof cancelled.actor_id==='string') return privateJson(200,payload);
  }
  const task=toUiTask(payload);
  return task && task.matchedRequestId===id?privateJson(200,{task}):privateJson(502,{error:'invalid assistant task receipt'});
}
