use crate::Perform;
use actix_web::web::Data;
use lemmy_api_common::{
  site::{GetSiteTaglines, GetSiteTaglinesResponse},
  utils::{blocking, get_local_user_view_from_jwt_opt, is_admin},
};
use lemmy_db_schema::source::tagline::Tagline;
use lemmy_utils::{error::LemmyError, ConnectionId};
use lemmy_websocket::LemmyContext;

#[async_trait::async_trait(?Send)]
impl Perform for GetSiteTaglines {
  type Response = GetSiteTaglinesResponse;

  #[tracing::instrument(skip(context, _websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<GetSiteTaglinesResponse, LemmyError> {
    let data: &GetSiteTaglines = self;

    let local_user_view =
      get_local_user_view_from_jwt_opt(data.auth.as_ref(), context.pool(), context.secret())
        .await?;
    let is_admin = match local_user_view {
      Some(s) => (is_admin(&s).is_ok()),
      None => (false),
    };

    // Only let admins read this
    if !is_admin {
      return Err(LemmyError::from_message("not_an_admin"));
    }

    let taglines = blocking(context.pool(), Tagline::get_all).await??;

    Ok(GetSiteTaglinesResponse { taglines })
  }
}
