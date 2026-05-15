import React from 'react';
export function Step0Welcome({ onNext }: { onNext: () => void }) {
   return <div className="p-8"><h1>Your business, live in minutes.</h1><p>Zero tech skills needed. We do the heavy lifting.</p><button className="bg-blue-600 text-white p-2 rounded" onClick={onNext}>🚀 Start My Business</button><button className="ml-4 text-blue-600" onClick={onNext}>⚡ Instant Build (AI) →</button></div>;
}
