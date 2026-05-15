import React from 'react';
export function Step6Template({ onNext }: { onNext: () => void }) {
   return <div className="p-8"><h2>Choose a Template</h2><button className="border p-2 mb-4" onClick={onNext}>✨ Modern</button><br/><button className="bg-blue-600 text-white p-2 rounded" onClick={onNext}>Next →</button></div>;
}
