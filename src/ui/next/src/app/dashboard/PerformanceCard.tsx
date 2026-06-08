import React from 'react';

export default function PerformanceCard() {
  return (
    <div className="performance-card rounded-xl p-4 text-white" style={{
      background: 'rgba(255, 255, 255, 0.05)',
      backdropFilter: 'blur(20px) saturate(200%)',
      border: '1px solid rgba(255, 255, 255, 0.1)'
    }}>
      <h3 className="font-semibold text-lg mb-2">Performance & SEO</h3>
      <div className="flex items-center space-x-2 mb-1">
        <span className="h-3 w-3 rounded-full bg-green-500"></span>
        <span>Edge Cache Active</span>
      </div>
      <div className="flex items-center space-x-2">
        <span className="h-3 w-3 rounded-full bg-green-500"></span>
        <span>SEO Status: Excellent</span>
      </div>
    </div>
  );
}
