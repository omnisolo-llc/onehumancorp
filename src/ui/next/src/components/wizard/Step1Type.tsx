import React from 'react';
export function Step1Type({ onNext }: { onNext: () => void }) {
   return <div className="p-8"><h2>What kind of business are you building?</h2><div className="flex gap-2 mb-4"><button className="border p-2" onClick={onNext}>🛒 Online Store</button><button className="border p-2" onClick={onNext}>🍕 Restaurant / Food</button><button className="border p-2" onClick={onNext}>🛠️ Service Business</button></div><input type="text" placeholder="e.g. I run a local bakery called Maya's Cakes..." className="border p-2 w-full mb-4" /><button className="bg-blue-600 text-white p-2 rounded" onClick={onNext}>Next →</button></div>;
}
