-- +goose Up
CREATE TABLE shared_tasks (
    id SERIAL PRIMARY KEY,
    status VARCHAR(50) NOT NULL,
    priority VARCHAR(50) NOT NULL,
    agent_id VARCHAR(255)
);

CREATE TABLE task_dependencies (
    task_id INT NOT NULL,
    depends_on INT NOT NULL,
    PRIMARY KEY (task_id, depends_on),
    FOREIGN KEY (task_id) REFERENCES shared_tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (depends_on) REFERENCES shared_tasks(id) ON DELETE CASCADE
);

-- +goose Down
DROP TABLE task_dependencies;
DROP TABLE shared_tasks;
