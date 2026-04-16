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
      padding: '24px',
      borderRadius: '16px',
      border: '1px solid rgba(255, 255, 255, 0.1)',
      color: '#fff',
      boxShadow: '0 4px 30px rgba(0, 0, 0, 0.1)'
    }}>
      <h2>Task DAG Viewer</h2>
      <p>Visual representation of parent-child dependencies and task statuses.</p>
      {loading ? (
        <p>Loading tasks...</p>
      ) : (
        <ul data-testid="task-list">
          {tasks.length === 0 ? <p>No tasks in DAG.</p> : tasks.map((task, idx) => (
            <li key={idx} style={{ marginBottom: '10px' }}>
              <span style={{
                marginRight: '10px',
                color: task.status === 'PENDING' ? '#f39c12' :
                       task.status === 'COMPLETED' ? '#2ecc71' :
                       task.status === 'EXECUTING' ? '#3498db' : '#fff'
              }}>
                {task.title} - {task.status}
              </span>
              <button onClick={() => handlePause(task.id)} style={{ marginRight: '5px' }}>Pause</button>
              <button onClick={() => handleKill(task.id)}>Kill</button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default TaskDAGViewer;
