'use client';

import {useCallback,useEffect,useRef,useState} from 'react';
import {MutationService} from '../../lib/sync/MutationService';
import {NetworkStatusIndicator} from '../../components/NetworkStatusIndicator';

type Order={id:string;delivery_task_id?:string;fulfillment_mode:string|null;status:string;customer_name:string|null;items:string[];organization_id:string;driver_status?:string|null;driver_lat?:number|null;driver_lng?:number|null;binding_status?:string;label_url?:string|null};
type Queue={to_pack:Order[];awaiting_pickup:Order[]};
const record=(value:unknown):value is Record<string,unknown>=>Boolean(value)&&typeof value==='object'&&!Array.isArray(value);
function parseQueue(value:unknown):Queue{
  if(!record(value)||!Array.isArray(value.to_pack)||!Array.isArray(value.awaiting_pickup))throw new Error('Fulfillment data is unavailable.');
  for(const order of [...value.to_pack,...value.awaiting_pickup]){
    if(!record(order)||typeof order.id!=='string'||!order.id||typeof order.status!=='string'||!Array.isArray(order.items)||!order.items.every(item=>typeof item==='string'))throw new Error('Fulfillment data is unavailable.');
  }
  return value as Queue;
}
const statusLabel=(status:string)=>({LABEL_CREATED:'Label created',PENDING:'Pending',PREPARING:'Preparing',PRE_TRANSIT:'Awaiting carrier pickup',TRANSIT:'In transit',HANDED_OFF:'Handed to driver',ReadyForPickup:'Ready for pickup',DRIVER_CONFIRMED:'Driver confirmed',DRIVER_ENROUTE_TO_PICKUP:'Driver approaching pickup',DRIVER_REQUESTED:'Driver requested',RETURNING:'Returning',FAILURE:'Delivery problem'}[status]||status);
function labelLink(value:unknown):string|null{
  if(typeof value!=='string')return null;
  try{const url=new URL(value);const trusted=url.hostname==='goshippo.com'||url.hostname.endsWith('.goshippo.com')||['shippo-delivery.s3.amazonaws.com','shippo-delivery-east.s3.amazonaws.com','shippo-delivery-west.s3.amazonaws.com'].includes(url.hostname);return url.protocol==='https:'&&trusted&&!url.username&&!url.password?url.href:null;}catch{return null;}
}

export default function FulfillmentHub(){
  const [queue,setQueue]=useState<Queue>({to_pack:[],awaiting_pickup:[]});
  const [loading,setLoading]=useState(true);
  const [loaded,setLoaded]=useState(false);
  const [error,setError]=useState('');
  const [busy,setBusy]=useState<string|null>(null);
  const [queued,setQueued]=useState<string|null>(null);
  const mutationPending=useRef(false);
  const loadQueue=useCallback(async(signal?:AbortSignal)=>{
    setLoading(true);
    try{
      const response=await fetch('/api/v1/fulfillment',{signal});
      const body:unknown=await response.json();
      if(!response.ok)throw new Error(record(body)&&typeof body.error==='string'?body.error:'Fulfillment data is unavailable.');
      setQueue(parseQueue(body));setLoaded(true);setError('');
    }catch(cause){if(!signal?.aborted)setError(cause instanceof Error?cause.message:'Fulfillment data is unavailable.');}
    finally{if(!signal?.aborted)setLoading(false);}
  },[]);
  useEffect(()=>{const controller=new AbortController();void loadQueue(controller.signal);return()=>controller.abort();},[loadQueue]);
  const handleAction=async(order:Order,action:string)=>{
    if(mutationPending.current)return;
    mutationPending.current=true;setBusy(order.id);setError('');
    try{
      if(!navigator.onLine){
        if(action==='request_driver')throw new Error('Connect before requesting a driver. No driver has been requested.');
        await MutationService.getInstance().executeMutation('fulfillment_action',{id:order.id,action},()=>{},()=>{});
        setQueued(order.id);return;
      }
      const response=await fetch(`/api/v1/fulfillment/execute/${encodeURIComponent(order.id)}`,{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({action})});
      const body:unknown=await response.json();
      if(!response.ok||!record(body)||body.success!==true)throw new Error(record(body)&&typeof body.error==='string'?body.error:'The fulfillment action could not be confirmed. Refresh before retrying.');
      await loadQueue();
    }catch(cause){setError(cause instanceof Error?cause.message:'The fulfillment action could not be confirmed.');}
    finally{mutationPending.current=false;setBusy(null);}
  };
  const card=(order:Order)=>{
    const url=labelLink(order.label_url);
    return <article key={order.delivery_task_id||order.id} className="app-card rounded-xl border border-gray-200 bg-white p-4 shadow-sm">
      <p className="text-xs font-medium text-gray-600">{order.fulfillment_mode||'Fulfillment mode not recorded'}</p>
      <h3 className="mt-1 font-semibold">{order.customer_name||`Order ${order.id}`}</h3>
      {order.items.length>0&&<p className="mt-1 text-sm text-gray-600">{order.items.join(', ')}</p>}
      <p className="mt-2 text-sm font-medium">{statusLabel(order.status)}</p>
      {typeof order.driver_lat==='number'&&typeof order.driver_lng==='number'&&<p className="text-sm text-gray-600">Driver location: {order.driver_lat.toFixed(4)}, {order.driver_lng.toFixed(4)}</p>}
      {order.binding_status==='reconciliation_required'&&<p className="mt-2 text-sm text-amber-800">Provider identity needs reconciliation before tracking can update this order.</p>}
      {queued===order.id&&<p role="status" className="mt-2 text-sm text-amber-800">Action queued. The recorded status will change after the server confirms it.</p>}
      <div className="mt-3 flex flex-wrap gap-2">
        {order.fulfillment_mode==='Shipping'&&(url?<a href={url} target="_blank" rel="noopener noreferrer" className="text-blue-700 underline">Open shipping label</a>:<a href={`/orders/${encodeURIComponent(order.id)}`} className="text-blue-700 underline">Manage shipping</a>)}
        {order.fulfillment_mode==='LocalDelivery'&&['PENDING','PREPARING','Preparing'].includes(order.status)&&<button disabled={busy!==null||queued===order.id} onClick={()=>void handleAction(order,'mark_ready')} className="rounded-lg bg-gray-900 px-3 py-2 text-white disabled:opacity-50">Mark ready</button>}
        {order.fulfillment_mode==='LocalDelivery'&&order.status==='ReadyForPickup'&&<button disabled={busy!==null||queued===order.id} onClick={()=>void handleAction(order,'request_driver')} className="rounded-lg bg-indigo-700 px-3 py-2 text-white disabled:opacity-50">Request driver</button>}
        {order.fulfillment_mode==='LocalDelivery'&&['ReadyForPickup','DRIVER_CONFIRMED','DRIVER_ENROUTE_TO_PICKUP'].includes(order.status)&&<button disabled={busy!==null||queued===order.id} onClick={()=>void handleAction(order,'hand_off')} className="rounded-lg bg-green-800 px-3 py-2 text-white disabled:opacity-50">Record handoff</button>}
      </div>
    </article>;
  };
  return <div className="min-h-screen bg-gray-50 p-4"><NetworkStatusIndicator/><main className="mx-auto max-w-2xl space-y-6">
    <header className="flex items-center justify-between"><h1 className="text-2xl font-bold">Fulfillment Hub</h1><button disabled={loading||busy!==null} onClick={()=>void loadQueue()} className="rounded-lg border px-3 py-2">Refresh</button></header>
    {error&&<p role="alert" className="rounded-lg bg-red-50 p-3 text-red-800">{error}</p>}
    {loading&&!loaded&&<p>Loading recorded fulfillment…</p>}
    {loaded&&<><section className="space-y-3"><h2 className="text-lg font-semibold">To pack</h2>{queue.to_pack.length?queue.to_pack.map(card):<p className="text-sm text-gray-600">No recorded orders waiting to be packed.</p>}</section><section className="space-y-3"><h2 className="text-lg font-semibold">Active fulfillment</h2>{queue.awaiting_pickup.length?queue.awaiting_pickup.map(card):<p className="text-sm text-gray-600">No active delivery tasks.</p>}</section></>}
  </main></div>;
}
