use crate::Perform;
use actix_web::web::Data;
use lemmy_api_common::{
  context::LemmyContext,
  utils::get_local_user_view_from_jwt,
  websocket::structs::{
    CommunityJoin,
    CommunityJoinResponse,
    ModJoin,
    ModJoinResponse,
    PostJoin,
    PostJoinResponse,
    UserJoin,
    UserJoinResponse,
  },
};
use lemmy_utils::{error::LemmyError, ConnectionId};

#[async_trait::async_trait(?Send)]
impl Perform for UserJoin {
  type Response = UserJoinResponse;

  #[tracing::instrument(skip(context, websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<UserJoinResponse, LemmyError> {
    let data: &UserJoin = self;
    let local_user_view =
      get_local_user_view_from_jwt(&data.auth, context.pool(), context.secret()).await?;

    if let Some(ws_id) = websocket_id {
      context
        .chat_server()
        .join_user_room(local_user_view.local_user.id, ws_id)?;
    }

    Ok(UserJoinResponse { joined: true })
  }
}

#[async_trait::async_trait(?Send)]
impl Perform for CommunityJoin {
  type Response = CommunityJoinResponse;

  #[tracing::instrument(skip(context, websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<CommunityJoinResponse, LemmyError> {
    let data: &CommunityJoin = self;

    if let Some(ws_id) = websocket_id {
      context
        .chat_server()
        .join_community_room(data.community_id, ws_id)?;
    }

    Ok(CommunityJoinResponse { joined: true })
  }
}

#[async_trait::async_trait(?Send)]
impl Perform for ModJoin {
  type Response = ModJoinResponse;

  #[tracing::instrument(skip(context, websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<ModJoinResponse, LemmyError> {
    let data: &ModJoin = self;

    if let Some(ws_id) = websocket_id {
      context
        .chat_server()
        .join_mod_room(data.community_id, ws_id)?;
    }

    Ok(ModJoinResponse { joined: true })
  }
}

#[async_trait::async_trait(?Send)]
impl Perform for PostJoin {
  type Response = PostJoinResponse;

  #[tracing::instrument(skip(context, websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<PostJoinResponse, LemmyError> {
    let data: &PostJoin = self;

    if let Some(ws_id) = websocket_id {
      context.chat_server().join_post_room(data.post_id, ws_id)?;
    }

    Ok(PostJoinResponse { joined: true })
  }
}
