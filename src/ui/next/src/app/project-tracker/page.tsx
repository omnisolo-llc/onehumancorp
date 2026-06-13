'use client';

import React, { useEffect, useState } from 'react';
import { FiCheckCircle, FiCircle, FiLoader } from 'react-icons/fi';

interface Task {
  id: string;
  title: string;
  status: string;
}

interface Project {
  id: string;
  quote_id: string | null;
  title: string;
  status: string;
  tasks: Task[];
}

interface Invoice {
  id: string;
  status: string;
  total_amount: number;
}

interface ProjectsResponse {
  projects: Project[];
  invoices: Invoice[];
}

export default function ProjectTrackerPage() {
  const [data, setData] = useState<ProjectsResponse | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const res = await fetch('/api/v1/quotes/projects');
        if (res.ok) {
          const json = await res.json();
          setData(json);
        }
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, []);

  if (loading) {
    return (
      <div className="flex h-screen items-center justify-center bg-gray-50 dark:bg-gray-900">
        <FiLoader className="animate-spin text-4xl text-blue-500" />
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-[#0A0A0A] p-4 md:p-8 font-sans pb-24">
      <div className="max-w-3xl mx-auto space-y-6">
        <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Active Projects</h1>

        {(!data || data.projects.length === 0) ? (
          <div className="glassmorphism p-6 rounded-xl border border-white/40 dark:border-white/10 text-center text-gray-500">
            No active projects found.
          </div>
        ) : (
          <div className="space-y-6">
            {data.projects.map((project) => (
              <div key={project.id} className="glassmorphism rounded-2xl p-6 border border-white/40 dark:border-white/10 shadow-sm" data-testid="project-card">
                <div className="flex justify-between items-start mb-4">
                  <h2 className="text-xl font-semibold text-gray-900 dark:text-white">{project.title}</h2>
                  <span className="px-3 py-1 bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300 rounded-full text-xs font-medium uppercase tracking-wider">
                    {project.status}
                  </span>
                </div>

                <div className="space-y-3 mt-4">
                  <h3 className="text-sm font-semibold text-gray-500 uppercase tracking-wider">Task Checklist</h3>
                  <div className="space-y-2">
                    {project.tasks.map(task => (
                      <div key={task.id} className="flex items-center space-x-3 text-sm p-2 rounded-lg hover:bg-black/5 dark:hover:bg-white/5 transition-colors" data-testid="task-item">
                        {task.status === 'DONE' ? (
                          <FiCheckCircle className="text-green-500 flex-shrink-0" />
                        ) : (
                          <FiCircle className="text-gray-400 flex-shrink-0" />
                        )}
                        <span className={`text-gray-700 dark:text-gray-300 ${task.status === 'DONE' ? 'line-through opacity-50' : ''}`}>
                          {task.title}
                        </span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}

        <h1 className="text-2xl font-bold text-gray-900 dark:text-white mt-10">Invoices</h1>
        {(!data || data.invoices.length === 0) ? (
          <div className="glassmorphism p-6 rounded-xl border border-white/40 dark:border-white/10 text-center text-gray-500">
            No invoices found.
          </div>
        ) : (
          <div className="space-y-4">
            {data.invoices.map((invoice) => (
              <div key={invoice.id} className="glassmorphism rounded-xl p-4 border border-white/40 dark:border-white/10 flex justify-between items-center" data-testid="invoice-card">
                <div>
                  <p className="font-medium text-gray-900 dark:text-white">Deposit Invoice</p>
                  <p className="text-sm text-gray-500">${invoice.total_amount.toFixed(2)}</p>
                </div>
                <span className={`px-3 py-1 rounded-full text-xs font-medium uppercase tracking-wider ${invoice.status.toLowerCase() === 'paid' ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800'}`}>
                  {invoice.status}
                </span>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
