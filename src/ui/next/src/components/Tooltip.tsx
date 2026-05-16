'use client';
import { useState, useEffect } from 'react';

export default function Tooltip({ elementId, children }: { elementId: string, children: React.ReactNode }) {
  const [data, setData] = useState<any>(null);
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    fetch(`/api/v1/docs/tooltip?element_id=${elementId}`)
      .then(r => r.json())
      .then(d => {
        if (d.status === 'ok') setData(d.tooltip);
      });
  }, [elementId]);

  return (
    <div
      className="relative inline-block"
      onMouseEnter={() => setVisible(true)}
      onMouseLeave={() => setVisible(false)}
      onTouchStart={() => setVisible(true)}
      onTouchEnd={() => setVisible(false)}
    >
      {children}
      {visible && data && (
        <div className="absolute z-50 bottom-full left-1/2 transform -translate-x-1/2 mb-2 w-64 p-3 bg-white/80 backdrop-blur-xl saturate-200 border border-white/20 rounded-lg shadow-xl text-sm transition-all duration-200 ease-out">
          <strong className="block mb-1 text-gray-900">{data.title}</strong>
          <p className="text-gray-600 leading-tight">{data.description}</p>
        </div>
      )}
    </div>
  );
}
