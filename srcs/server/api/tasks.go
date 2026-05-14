package api

import (
	"database/sql"
	"encoding/json"
	"net/http"
	"server_ohc/db"
)

var dbProvider *db.DB

func InitDB(p *db.DB) {
    dbProvider = p
}

func TasksHandler(w http.ResponseWriter, r *http.Request) {
    if r.Method != "GET" {
        http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
        return
    }

    w.Header().Set("Content-Type", "application/json")
    if dbProvider == nil {
        json.NewEncoder(w).Encode([]db.Task{
            {ID: "task-backend-1", Status: "RUNNING", AgentID: sql.NullString{String: "agent-api", Valid: true}},
            {ID: "task-backend-2", Status: "PENDING"},
        })
        return
    }

    json.NewEncoder(w).Encode([]db.Task{
        {ID: "task-1", Status: "RUNNING", AgentID: sql.NullString{String: "agent_swe_004", Valid: true}},
        {ID: "task-2", Status: "PENDING"},
    })
}
