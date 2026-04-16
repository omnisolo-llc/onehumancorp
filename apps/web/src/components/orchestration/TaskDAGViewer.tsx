import React, { useEffect, useState } from 'react';
import { theme } from '../../styles/theme';

const TaskDAGViewer = () => {
  const [tasks, setTasks] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/api/v1/orchestration/tasks')
      .then((res) => res.json())
      .then((data) => {
        setTasks(data);
        setLoading(false);
      })
      .catch((err) => {
        console.error('Failed to fetch tasks', err);
        setLoading(false);
      });
  }, []);

  const handlePause = (taskId: string) => {
    fetch(`/api/v1/orchestration/tasks/${taskId}/pause`, { method: 'POST' });
  };

  const handleKill = (taskId: string) => {
    fetch(`/api/v1/orchestration/tasks/${taskId}/kill`, { method: 'POST' });
  };

  return (
    <div style={{ ...theme.glassmorphism, ...theme.typography, padding: '24px', borderRadius: '16px', color: theme.colors.text, marginBottom: '24px' }}>
      <h2 style={{ marginBottom: '8px', fontWeight: 600 }}>Task DAG Viewer</h2>
      <p style={{ color: 'rgba(255,255,255,0.7)', marginBottom: '20px', fontSize: '14px' }}>Visual representation of parent-child dependencies and task statuses.</p>
      {loading ? (
        <div style={{ padding: '20px', textAlign: 'center', background: 'rgba(0,0,0,0.2)', borderRadius: '12px' }}>Loading tasks...</div>
      ) : (
        <ul data-testid="task-list" style={{ listStyle: 'none', padding: 0, margin: 0 }}>
          {tasks.length === 0 ? <li style={{ padding: '20px', textAlign: 'center', background: 'rgba(0,0,0,0.2)', borderRadius: '12px', color: 'rgba(255,255,255,0.5)' }}>No tasks in DAG.</li> : tasks.map((task, idx) => (
            <li key={idx} style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
              padding: '12px 16px',
              marginBottom: '10px',
              background: 'rgba(0,0,0,0.2)',
              borderRadius: '8px',
              borderLeft: `4px solid ${task.status === 'PENDING' ? theme.colors.pending : task.status === 'COMPLETED' ? theme.colors.completed : task.status === 'EXECUTING' ? theme.colors.executing : theme.colors.text}`
            }}>
              <div style={{ display: 'flex', alignItems: 'center' }}>
                <span style={{ fontWeight: 500 }}>{task.title}</span>
                <span style={{
                  marginLeft: '12px',
                  fontSize: '12px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  background: 'rgba(255,255,255,0.1)',
                  color: task.status === 'PENDING' ? theme.colors.pending : task.status === 'COMPLETED' ? theme.colors.completed : task.status === 'EXECUTING' ? theme.colors.executing : theme.colors.text
                }}>
                  {task.status}
                </span>
              </div>
              <div>
                <button onClick={() => handlePause(task.id)} style={{
                  marginRight: '8px',
                  padding: '6px 12px',
                  background: 'transparent',
                  border: '1px solid rgba(255,255,255,0.2)',
                  color: theme.colors.text,
                  borderRadius: '6px',
                  cursor: 'pointer'
                }}>Pause</button>
                <button onClick={() => handleKill(task.id)} style={{
                  padding: '6px 12px',
                  background: 'rgba(231, 76, 60, 0.2)',
                  border: '1px solid rgba(231, 76, 60, 0.5)',
                  color: theme.colors.error,
                  borderRadius: '6px',
                  cursor: 'pointer'
                }}>Kill</button>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default TaskDAGViewer;
