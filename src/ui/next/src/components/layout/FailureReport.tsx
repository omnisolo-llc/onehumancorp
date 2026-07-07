import React from 'react';

export const FailureReport = ({ title, message, errorRateData, latencyData }: {
    title?: string;
    message: string;
    errorRateData?: { time: string; error_rate: number }[];
    latencyData?: { bucket: string; count: number }[];
}) => {
  // Find max values for scaling the bars
  const maxErrorRate = errorRateData && errorRateData.length > 0 ? Math.max(...errorRateData.map(d => d.error_rate)) : 100;
  const maxLatencyCount = latencyData && latencyData.length > 0 ? Math.max(...latencyData.map(d => d.count)) : 100;

  return (
    <div className="p-6 rounded-[16px] backdrop-blur-[30px] backdrop-saturate-[210%] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] shadow-[0_4px_24px_rgba(0,0,0,0.04)] mb-6">
      {title && <h3 className="text-xl font-semibold mb-2 text-red-700 dark:text-red-400 font-outfit">{title}</h3>}
      <p className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-6">{message}</p>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {errorRateData && errorRateData.length > 0 && (
          <div>
            <h4 className="text-sm font-medium mb-4 text-gray-600 dark:text-gray-400">Error Rate Over Time</h4>
            <div className="space-y-2">
              {errorRateData.map((data, index) => (
                <div key={index} className="flex items-center text-xs text-gray-600 dark:text-gray-400">
                  <span className="w-16 flex-shrink-0 truncate">{data.time}</span>
                  <div className="flex-grow mx-2 bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div
                      className="bg-red-500 h-2 rounded-full"
                      style={{ width: `${(data.error_rate / (maxErrorRate || 1)) * 100}%` }}
                    />
                  </div>
                  <span className="w-12 flex-shrink-0 text-right">{data.error_rate}%</span>
                </div>
              ))}
            </div>
          </div>
        )}

        {latencyData && latencyData.length > 0 && (
          <div>
            <h4 className="text-sm font-medium mb-4 text-gray-600 dark:text-gray-400">Latency Histogram</h4>
            <div className="space-y-2 flex items-end h-32 gap-1 justify-between">
              {latencyData.map((data, index) => (
                <div key={index} className="flex flex-col items-center flex-grow group relative">
                    <div className="opacity-0 group-hover:opacity-100 absolute -top-8 text-xs bg-gray-800 text-white rounded px-2 py-1 transition-opacity">
                        {data.count}
                    </div>
                  <div
                    className="w-full bg-blue-500 rounded-t-sm"
                    style={{ height: `${(data.count / (maxLatencyCount || 1)) * 100}%`, minHeight: '4px' }}
                  />
                  <span className="text-[10px] text-gray-500 mt-1 truncate max-w-full" title={data.bucket}>{data.bucket}</span>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
