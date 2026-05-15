import React from 'react';
export function Step9Review({ onNext }: { onNext: () => void }) {
   return <div className="p-8"><h2>Review & Launch!</h2><button className="bg-green-600 text-white p-2 rounded" onClick={onNext}>Publish my business →</button></div>;
}
