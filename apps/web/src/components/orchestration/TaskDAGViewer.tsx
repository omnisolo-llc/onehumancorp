import React, { useEffect, useState } from 'react';

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
    <div style={{
      backdropFilter: 'blur(20px) saturate(200%)',
      background: 'rgba(255, 255, 255, 0.03)',
      fontFamily: '"Outfit", "Inter", sans-serif',
      padding: '20px',
      borderRadius: '12px',
      border: '1px solid rgba(255, 255, 255, 0.1)',
      color: '#fff'
    }}>
      <h2>Task DAG Viewer</h2>
      <p>Visual representation of parent-child dependencies and task statuses.</p>
      {loading ? (
        <p>Loading tasks...</p>
      ) : (
        <ul data-testid="task-list">
          {tasks.map((task, idx) => (
            <li key={idx}>
              {task.title} - {task.status}
              <button onClick={() => handlePause(task.id)}>Pause</button>
              <button onClick={() => handleKill(task.id)}>Kill</button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default TaskDAGViewer;
