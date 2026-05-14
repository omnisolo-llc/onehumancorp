use tonic::{Request, Response, Status};
use crate::proto::help::{help_service_server::HelpService, *};

pub struct HelpServiceImpl {
    pub repository: crate::domain::help::repository::HelpRepository,
}

impl HelpServiceImpl {
    pub fn new(repository: crate::domain::help::repository::HelpRepository) -> Self {
        Self { repository }
    }
}
    }
}

#[tonic::async_trait]
impl HelpService for HelpServiceImpl {
    async fn get_help_center(
        &self,
        request: Request<GetHelpCenterRequest>,
    ) -> Result<Response<GetHelpCenterResponse>, Status> {
        let query = request.into_inner().query;
        let (answer, related_article_ids) = self.repository.ask_chat(&query).await.unwrap_or_default();

        Ok(Response::new(AskHelpChatResponse {
            answer,
            related_article_ids,
        }))
    }
}
