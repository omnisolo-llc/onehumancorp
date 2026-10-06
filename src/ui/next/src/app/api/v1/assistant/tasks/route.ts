import { proxyBackendRequest } from '@/lib/auth/backendTransport';
import { privateJson } from '../assistantBackend';
import { toBackendTask, toUiTask } from './taskContract';
export async function GET(request: Request): Promise<Response> {
  const response = await proxyBackendRequest(request, '/api/v1/assistant/tasks');
  if (!response.ok) return response;
  const payload: unknown = await response.json().catch(() => null);
  if (!Array.isArray(payload) || payload.length > 4) return privateJson(502, {error:'invalid assistant task list'});
  const tasks = payload.slice(0,3).map(value => toUiTask(value));
  if (tasks.some(task => task === null)) return privateJson(502, {error:'invalid assistant task receipt'});
  return privateJson(200, {tasks, nextCursor: payload.length > 3 ? tasks.at(-1)?.id : null,
    capabilities:{outputFormats:['Text'],workModes:['Ask'],modelProviders:['Auto']}});
}
export async function POST(request: Request): Promise<Response> {
  const response = await proxyBackendRequest(request, '/api/v1/assistant/tasks', {requestContentType:'application/json',transformRequestBody:toBackendTask});
  if (!response.ok) return response;
  const task = toUiTask(await response.json().catch(() => null));
  return response.status === 202 && task ? privateJson(202,{task}) : privateJson(502,{error:'invalid assistant task receipt'});
}
