'use client';
import { useState, useEffect } from 'react';
import { AppShell } from '@/components/AppShell';

interface AgentFeedItem {
  id: string;
  tenant_id: string;
  event_source: string;
  lifecycle_state: string;
  context_payload?: {
    title?: string;
    description?: string;
    department?: string;
  };
  proposed_action?: {
    action_type?: string;
    payload?: any;
  };
}

interface ActionCardProps {
  id: string;
  department: string;
  title: string;
  description: string;
  icon: string;
  colorClass: string;
  onApprove: (id: string) => void;
  status: 'pending' | 'completed';
}

function ActionCard({ id, department, title, description, icon, colorClass, onApprove, status }: ActionCardProps) {
  if (status === 'completed') {
    return (
      <div className={`p-4 rounded-xl border border-white/40 dark:border-white/10 glassmorphism opacity-60 transition-all`}>
        <div className="flex items-center gap-3">
          <div className={`w-8 h-8 rounded-full ${colorClass} flex items-center justify-center text-sm`}>
            ✓
          </div>
          <div>
            <h4 className="font-semibold text-sm line-through text-gray-500">{title}</h4>
            <p className="text-xs text-gray-500">Action completed</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className={`p-5 rounded-2xl border border-white/40 dark:border-white/10 glassmorphism shadow-sm hover:shadow-md transition-all`}>
      <div className="flex items-start gap-4 mb-4">
        <div className={`w-12 h-12 rounded-full flex-shrink-0 ${colorClass} flex items-center justify-center text-xl`}>
          {icon}
        </div>
        <div className="flex-1">
          <div className="text-xs font-medium uppercase tracking-wider text-gray-500 mb-1">{department}</div>
          <h3 className="font-bold text-gray-900 dark:text-white leading-tight mb-2">{title}</h3>
          <p className="text-sm text-gray-600 dark:text-gray-300 leading-relaxed">{description}</p>
        </div>
      </div>

      <div className="flex gap-2 mt-4 pt-4 border-t border-gray-100 dark:border-gray-800">
        <button
          onClick={() => onApprove(id)}
          className="flex-1 bg-black dark:bg-white text-white dark:text-black font-medium py-3 px-4 rounded-xl text-sm transition-transform active:scale-95"
        >
          Approve
        </button>
        <button className="px-4 py-3 rounded-xl font-medium text-gray-600 dark:text-gray-400 bg-gray-100 dark:bg-gray-800/50 text-sm hover:bg-gray-200 dark:hover:bg-gray-800 transition-colors">
          Edit
        </button>
      </div>
    </div>
  );
}

export default function AgentFeedPage() {
  const [items, setItems] = useState<AgentFeedItem[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function fetchFeed() {
      try {
        const res = await fetch('/api/ui/proxy?path=/api/v1/agent-feed');
        if (res.ok) {
          const data = await res.json();
          setItems(data.items || []);
        }
      } catch (err) {
        console.error("Failed to fetch agent feed:", err);
      } finally {
        setIsLoading(false);
      }
    }
    fetchFeed();
  }, []);

  const handleApprove = async (id: string) => {
    try {
      const res = await fetch(`/api/ui/proxy?path=/api/v1/agent-feed/${id}/state`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ state: 'APPROVED' })
      });
      if (res.ok) {
        setItems(prevItems =>
          prevItems.map(item =>
            item.id === id ? { ...item, lifecycle_state: 'APPROVED' } : item
          )
        );
      }
    } catch (err) {
      console.error("Failed to approve item:", err);
    }
  };

  const getCardProps = (item: AgentFeedItem) => {
    const isCompleted = item.lifecycle_state !== 'PENDING';
    const title = item.context_payload?.title || 'System Alert';
    const description = item.context_payload?.description || 'Action required.';
    let department = item.context_payload?.department || 'Operations';
    let icon = '⚙️';
    let colorClass = 'bg-gray-100 text-gray-600 dark:bg-gray-900/30 dark:text-gray-400';

    if (item.event_source.includes('customer') || department.toLowerCase().includes('customer')) {
        department = 'Customer Success';
        icon = '💬';
        colorClass = 'bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400';
    } else if (item.event_source.includes('inventory') || department.toLowerCase().includes('operations')) {
        department = 'Operations';
        icon = '📦';
        colorClass = 'bg-orange-100 text-orange-600 dark:bg-orange-900/30 dark:text-orange-400';
    } else if (item.event_source.includes('sale') || department.toLowerCase().includes('marketing')) {
        department = 'Marketing';
        icon = '⚡';
        colorClass = 'bg-purple-100 text-purple-600 dark:bg-purple-900/30 dark:text-purple-400';
    }

    return {
      id: item.id,
      department,
      title,
      description,
      icon,
      colorClass,
      status: isCompleted ? 'completed' : 'pending' as const,
    };
  };

  return (
    <AppShell title="Agent Feed">
      <div className="max-w-md mx-auto min-h-[calc(100vh-80px)] pb-20">
        <header className="mb-6 pt-2">
          <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-1">Your Feed</h1>
          <p className="text-sm text-gray-500">Review drafted actions from your AI assistants</p>
        </header>

        <div className="flex flex-col gap-4">
          {isLoading ? (
            <div className="text-center py-12 text-gray-500">
              <p>Loading Agent Proposals...</p>
            </div>
          ) : items.length === 0 ? (
            <div className="text-center py-12 text-gray-500">
              <div className="text-4xl mb-3">✨</div>
              <p>All caught up for now!</p>
            </div>
          ) : (
            items.map(item => (
              <ActionCard
                key={item.id}
                {...getCardProps(item)}
                onApprove={handleApprove}
              />
            ))
          )}
        </div>
      </div>
    </AppShell>
  );
}
