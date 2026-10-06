import { proxyBackendRequest } from '@/lib/auth/backendTransport';
import { privateJson } from '../assistantBackend';
import { toUiTask } from '../tasks/taskContract';
export async function GET(request:Request):Promise<Response>{
  const response=await proxyBackendRequest(request,'/api/v1/assistant/legacy-tasks');
  if(!response.ok)return response;
  const payload:unknown=await response.json().catch(()=>null);
  if(!Array.isArray(payload))return privateJson(502,{error:'invalid legacy task list'});
  const tasks=payload.map(value=>toUiTask(value,true));
  return tasks.some(task=>task===null)?privateJson(502,{error:'invalid legacy task'}):privateJson(200,{tasks,nextCursor:null});
}
