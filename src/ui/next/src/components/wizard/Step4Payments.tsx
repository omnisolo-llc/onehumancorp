import React from 'react';
export function Step4Payments({ onNext }: { onNext: () => void }) {
   return <div className="p-8"><h2>How do you want to receive payments?</h2><button className="border p-2 mb-4" onClick={onNext}>🌐 Online only</button><br/><button className="bg-blue-600 text-white p-2 rounded" onClick={onNext}>Next →</button></div>;
}
