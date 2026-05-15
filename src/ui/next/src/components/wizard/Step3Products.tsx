import React from 'react';
export function Step3Products({ onNext }: { onNext: () => void }) {
   return <div className="p-8"><h2>What do you sell?</h2><button className="border p-2 mb-4" onClick={onNext}>📦 Physical products</button><br/><button className="bg-blue-600 text-white p-2 rounded" onClick={onNext}>Next →</button></div>;
}
