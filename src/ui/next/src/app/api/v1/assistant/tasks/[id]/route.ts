import { proxyBackendRequest } from '@/lib/auth/backendTransport';
import { privateJson, taskBackendPath } from '../../assistantBackend';
import { toUiTask } from '../taskContract';
async function forward(request:Request,context:{params:Promise<{id:string}>}):Promise<Response>{
  let path:string;
  try {path=taskBackendPath((await context.params).id);} catch {return privateJson(400,{error:'invalid task ID'});}
  const response=await proxyBackendRequest(request,path);
  if(!response.ok)return response;
  const task=toUiTask(await response.json().catch(()=>null));
  return task?privateJson(response.status,{task}):privateJson(502,{error:'invalid assistant task receipt'});
}
export const GET=forward;
export const PATCH=forward;
