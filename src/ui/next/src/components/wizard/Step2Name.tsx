import React from 'react';
export function Step2Name({ onNext }: { onNext: () => void }) {
   return <div className="p-8"><h2>Give your business a name</h2><input type="text" placeholder="e.g. Maya's Cakes" className="border p-2 w-full mb-4" /><button className="bg-blue-600 text-white p-2 rounded" onClick={onNext}>Next →</button></div>;
}
