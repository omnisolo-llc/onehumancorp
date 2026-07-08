use tonic::{Request, Response, Status};
use std::sync::Arc;
use crate::domain::search::{SearchService as DomainSearchService, SearchRequest as DomainSearchRequest};
use crate::proto::search::{search_service_server::SearchService, SearchRequest, SearchResponse, SearchResultItem};

pub struct SearchGrpcService {
    domain_service: Arc<DomainSearchService>,
}

impl SearchGrpcService {
    pub fn new(domain_service: Arc<DomainSearchService>) -> Self {
        Self { domain_service }
    }
}

#[tonic::async_trait]
impl SearchService for SearchGrpcService {
    async fn search(&self, request: Request<SearchRequest>) -> Result<Response<SearchResponse>, Status> {
        let req = request.into_inner();

        let tenant_id = "test-tenant-id".to_string(); // In reality, get from auth context

        let domain_req = DomainSearchRequest {
            query: req.query,
            domain_filter: if req.domain_filter.is_empty() { None } else { Some(req.domain_filter) },
            limit: if req.limit <= 0 { 20 } else { req.limit },
            offset: req.offset,
            tenant_id,
        };

        match self.domain_service.search(domain_req).await {
            Ok(results) => {
                let proto_results: Vec<SearchResultItem> = results.into_iter().map(|item| {
                    SearchResultItem {
                        id: item.id,
                        domain: item.domain,
                        title: item.title,
                        snippet: item.snippet,
                        link: item.link,
                        score: item.score,
                        created_at_unix: item.created_at_unix,
                    }
                }).collect();

                let total_count = proto_results.len() as i32; // Simplified

                Ok(Response::new(SearchResponse {
                    results: proto_results,
                    total_count,
                }))
            },
            Err(e) => {
                Err(Status::internal(format!("Internal error: {}", e)))
            }
        }
    }
}
