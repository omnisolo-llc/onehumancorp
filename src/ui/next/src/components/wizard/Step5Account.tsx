import React from 'react';
export function Step5Account({ onNext }: { onNext: () => void }) {
   return <div className="p-8"><h2>Create your account</h2><input type="text" placeholder="e.g. Maya Smith" className="border p-2 block mb-2" /><input type="email" placeholder="you@email.com" className="border p-2 block mb-2" /><input type="password" placeholder="Password" className="border p-2 block mb-2" /><div className="mb-4 text-red-500">Strength: Weak</div><button className="bg-blue-600 text-white p-2 rounded" onClick={onNext}>Next →</button></div>;
}
