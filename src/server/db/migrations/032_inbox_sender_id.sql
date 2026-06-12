-- +goose Up
-- Migration 032: Add sender_id to inbox_messages

ALTER TABLE inbox_messages
ADD COLUMN IF NOT EXISTS sender_id TEXT;

