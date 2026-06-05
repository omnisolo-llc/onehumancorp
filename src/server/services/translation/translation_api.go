package translation

import (
	"context"
	"log"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	pb "github.com/onehumancorp/mono/src/proto/translation"
)

type TranslationServer struct {
	pb.UnimplementedTranslationMeshServer
	service *TranslationMeshService
}

func NewTranslationServer(service *TranslationMeshService) *TranslationServer {
	return &TranslationServer{
		service: service,
	}
}

// Translate handles the gRPC request.
func (s *TranslationServer) Translate(ctx context.Context, req *pb.TranslateRequest) (*pb.TranslateResponse, error) {
	if req.TenantId == "" || req.SourceText == "" || req.TargetLocale == "" {
		return nil, status.Error(codes.InvalidArgument, "tenant_id, source_text, and target_locale are required")
	}

	internalReq := TranslationRequest{
		TenantID:     req.TenantId,
		SourceText:   req.SourceText,
		TargetLocale: req.TargetLocale,
	}

	resp, err := s.service.GetTranslation(ctx, internalReq)
	if err != nil {
		log.Printf("failed to get translation: %v", err)
		return nil, status.Errorf(codes.Internal, "failed to process translation: %v", err)
	}

	return &pb.TranslateResponse{
		TranslatedText: resp.TranslatedText,
		Status:         resp.Status,
	}, nil
}

// Register registers the gRPC server.
func Register(grpcServer *grpc.Server, server *TranslationServer) {
	pb.RegisterTranslationMeshServer(grpcServer, server)
}
