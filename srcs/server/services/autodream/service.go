package autodream

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	pb "github.com/onehumancorp/mono/srcs/proto/autodream"
)

type AutoDreamService struct {
	pb.UnimplementedAutoDreamServiceServer
	repo *db.AutoDreamRepository
}

func NewAutoDreamService(repo *db.AutoDreamRepository) *AutoDreamService {
	return &AutoDreamService{
		repo: repo,
	}
}

func (s *AutoDreamService) StoreFinding(ctx context.Context, req *pb.StoreFindingRequest) (*pb.StoreFindingResponse, error) {
	if req.Finding == nil {
		return nil, fmt.Errorf("finding is required")
	}

	ts, err := time.Parse(time.RFC3339, req.Finding.Timestamp)
	if err != nil {
		ts = time.Now()
	}

	finding := &db.Finding{
		ID:        req.Finding.Id,
		Timestamp: ts,
		Content:   req.Finding.Content,
		Embedding: req.Finding.Embedding,
	}

	if err := s.repo.Upsert(ctx, finding); err != nil {
		return &pb.StoreFindingResponse{Success: false}, err
	}

	return &pb.StoreFindingResponse{Success: true}, nil
}

func (s *AutoDreamService) SearchFindings(ctx context.Context, req *pb.SearchFindingsRequest) (*pb.SearchFindingsResponse, error) {
	limit := int(req.Limit)
	if limit <= 0 {
		limit = 10
	}

	findings, err := s.repo.Search(ctx, req.QueryEmbedding, limit)
	if err != nil {
		return nil, err
	}

	var pbFindings []*pb.Finding
	for _, f := range findings {
		pbFindings = append(pbFindings, &pb.Finding{
			Id:        f.ID,
			Timestamp: f.Timestamp.Format(time.RFC3339),
			Content:   f.Content,
			Embedding: f.Embedding,
		})
	}

	return &pb.SearchFindingsResponse{Findings: pbFindings}, nil
}
