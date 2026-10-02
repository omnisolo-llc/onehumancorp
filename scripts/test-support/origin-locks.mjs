// Deterministic origin-wide Web Locks test double; share one instance across simulated tabs.
export function createOriginLockManager(){
 const tails=new Map();
 return {request(name,_options,callback){const next=(tails.get(name)||Promise.resolve()).then(()=>callback({name}));tails.set(name,next.catch(()=>{}));return next;}};
}
