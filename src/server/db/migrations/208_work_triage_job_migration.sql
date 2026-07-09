-- +goose Up

UPDATE ohc_job_queue SET job_type = 'work_triage' WHERE job_type = 'message_triage';

-- +goose Down
UPDATE ohc_job_queue SET job_type = 'message_triage' WHERE job_type = 'work_triage';
