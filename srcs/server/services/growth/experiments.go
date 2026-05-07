package growth

import (
	"database/sql"
	"encoding/json"
	"log"
	"math/rand"
	"net/http"
)

type Experiment struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Variant     string `json:"variant"` // e.g., "A", "B"
	TrafficAllocation float32 `json:"traffic_allocation"` // 0.0 to 1.0
}

type AssignmentRequest struct {
	UserID string `json:"user_id"`
}

type AssignmentResponse struct {
	ExperimentID string `json:"experiment_id"`
	Variant      string `json:"variant"`
}

type ExperimentsService struct {
	db *sql.DB
}

func NewExperimentsService(db *sql.DB) *ExperimentsService {
	_, err := db.Exec(`
		CREATE TABLE IF NOT EXISTS experiments (
			id TEXT PRIMARY KEY,
			name TEXT,
			variant TEXT,
			traffic_allocation REAL
		);
		CREATE TABLE IF NOT EXISTS experiment_assignments (
			user_id TEXT,
			experiment_id TEXT,
			variant TEXT,
			PRIMARY KEY(user_id, experiment_id)
		);
	`)
	if err != nil {
		log.Fatalf("Failed to initialize experiments tables: %v", err)
	}
	return &ExperimentsService{
		db: db,
	}
}

// AddExperiment handles creation of a new experiment
func (s *ExperimentsService) HandleAddExperiment(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req Experiment
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	_, err := s.db.Exec(`
		INSERT INTO experiments (id, name, variant, traffic_allocation)
		VALUES ($1, $2, $3, $4)
	`, req.ID, req.Name, req.Variant, req.TrafficAllocation)

	if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(req)
}


func (s *ExperimentsService) HandleGetAssignment(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	experimentID := r.URL.Query().Get("experiment_id")
	userID := r.URL.Query().Get("user_id")

	if experimentID == "" || userID == "" {
		http.Error(w, "Missing experiment_id or user_id", http.StatusBadRequest)
		return
	}

	// Check if already assigned
	var assignedVariant string
	err := s.db.QueryRow(`
		SELECT variant FROM experiment_assignments
		WHERE user_id = $1 AND experiment_id = $2
	`, userID, experimentID).Scan(&assignedVariant)

	if err == nil {
		// Found existing assignment
		resp := AssignmentResponse{
			ExperimentID: experimentID,
			Variant:      assignedVariant,
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(resp)
		return
	} else if err != sql.ErrNoRows {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

	// Not assigned, determine assignment
	var allocation float32
	var variant string
	err = s.db.QueryRow(`
		SELECT traffic_allocation, variant FROM experiments
		WHERE id = $1
	`, experimentID).Scan(&allocation, &variant)

	if err != nil {
		if err == sql.ErrNoRows {
			http.Error(w, "Experiment not found", http.StatusNotFound)
			return
		}
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

	// Assign variant based on traffic allocation (rand is auto-seeded in Go 1.20+)
	randomVal := rand.Float32()

	finalVariant := "control" // Default to control
	if randomVal < allocation {
		finalVariant = variant
	}

	// Save assignment
	_, err = s.db.Exec(`
		INSERT INTO experiment_assignments (user_id, experiment_id, variant)
		VALUES ($1, $2, $3)
	`, userID, experimentID, finalVariant)

	if err != nil {
		http.Error(w, "Database error saving assignment", http.StatusInternalServerError)
		return
	}

	resp := AssignmentResponse{
		ExperimentID: experimentID,
		Variant:      finalVariant,
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}
