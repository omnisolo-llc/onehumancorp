-- Drop redundant tables now that dependencies are compressed within the main tables
DROP TABLE IF EXISTS task_dependencies;
DROP TABLE IF EXISTS swarm_task_dependencies;
