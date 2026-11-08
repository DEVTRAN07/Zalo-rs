//! Zalo API surface: holds session context and exposes endpoint methods.

pub mod accept_friend_request;
pub mod add_group_blocked_member;
pub mod add_group_deputy;
pub mod add_poll_options;
pub mod add_quick_message;
pub mod add_reaction;
pub mod add_unread_mark;
pub mod add_user_to_group;
pub mod block_user;
pub mod block_view_feed;
pub mod change_account_avatar;
pub mod change_friend_alias;
pub mod change_group_avatar;
pub mod change_group_name;
pub mod change_group_owner;
pub mod create_auto_reply;
pub mod create_bank_account;
pub mod create_catalog;
pub mod create_group;
pub mod create_note;
pub mod create_poll;
pub mod create_product_catalog;
pub mod create_reminder;
pub mod custom;
pub mod delete_auto_reply;
pub mod delete_avatar;
pub mod delete_bank_account;
pub mod delete_catalog;
pub mod delete_chat;
pub mod delete_group_invite_box;
pub mod delete_message;
pub mod delete_product_catalog;
pub mod disable_group_link;
pub mod disperse_group;
pub mod edit_note;
pub mod edit_reminder;
pub mod enable_group_link;
pub mod fetch_account_info;
pub mod find_user;
pub mod find_user_by_username;
pub mod get_user_info;
pub mod keep_alive;
pub mod login;
pub mod send_message;

use serde_json::Value;

use crate::context::ContextSession;
use crate::errors::Result;
use crate::models::r#enum::{AvatarSize, ThreadType};
use crate::ws::Listener;

pub use accept_friend_request::AcceptFriendRequestResponse;
pub use add_group_blocked_member::AddGroupBlockedMemberResponse;
pub use add_group_deputy::AddGroupDeputyResponse;
pub use add_poll_options::{AddPollOptionsPayload, AddPollOptionsResponse};
pub use add_quick_message::{AddQuickMessagePayload, AddQuickMessageResponse};
pub use add_reaction::{AddReactionDestination, AddReactionResponse};
pub use add_unread_mark::AddUnreadMarkResponse;
pub use add_user_to_group::AddUserToGroupResponse;
pub use block_user::BlockUserResponse;
pub use block_view_feed::BlockViewFeedResponse;
pub use change_account_avatar::ChangeAccountAvatarResponse;
pub use change_friend_alias::ChangeFriendAliasResponse;
pub use change_group_avatar::ChangeGroupAvatarResponse;
pub use change_group_name::ChangeGroupNameResponse;
pub use change_group_owner::ChangeGroupOwnerResponse;
pub use create_auto_reply::{CreateAutoReplyPayload, CreateAutoReplyResponse};
pub use create_bank_account::{CreateBankAccountPayload, CreateBankAccountResponse};
pub use create_catalog::CreateCatalogResponse;
pub use create_group::{CreateGroupOptions, CreateGroupResponse};
pub use create_note::{CreateNoteOptions, CreateNoteResponse};
pub use create_poll::{CreatePollOptions, CreatePollResponse};
pub use create_product_catalog::{CreateProductCatalogPayload, CreateProductCatalogResponse};
pub use create_reminder::{CreateReminderOptions, CreateReminderResponse};
pub use delete_auto_reply::DeleteAutoReplyResponse;
pub use delete_avatar::DeleteAvatarResponse;
pub use delete_bank_account::{DeleteBankAccountPayload, DeleteBankAccountResponse};
pub use delete_catalog::DeleteCatalogResponse;
pub use delete_chat::{DeleteChatLastMessage, DeleteChatResponse};
pub use delete_group_invite_box::DeleteGroupInviteBoxResponse;
pub use delete_message::{DeleteMessageDestination, DeleteMessageResponse};
pub use delete_product_catalog::{DeleteProductCatalogPayload, DeleteProductCatalogResponse};
pub use disable_group_link::DisableGroupLinkResponse;
pub use disperse_group::DisperseGroupResponse;
pub use edit_note::{EditNoteOptions, EditNoteResponse};
pub use edit_reminder::{EditReminderOptions, EditReminderResponse};
pub use enable_group_link::EnableGroupLinkResponse;
pub use fetch_account_info::FetchAccountInfoResponse;
pub use find_user::FindUserResponse;
pub use find_user_by_username::FindUserByUsernameResponse;
pub use get_user_info::UserInfoResponse;
pub use keep_alive::KeepAliveResponse;

/// High-level API bound to an authenticated session.
#[derive(Clone)]
pub struct ZaloApi {
    pub(crate) ctx: ContextSession,
    pub zpw_service_map: serde_json::Value,
    ws_urls: Vec<String>,
}

impl ZaloApi {
    pub fn new(ctx: ContextSession, zpw_service_map: serde_json::Value, ws_urls: Vec<String>) -> Self {
        Self {
            ctx,
            zpw_service_map,
            ws_urls,
        }
    }

    /// Get the authenticated user ID.
    pub fn uid(&self) -> &str {
        &self.ctx.uid
    }

    /// Access the session context.
    pub fn context(&self) -> &ContextSession {
        &self.ctx
    }

    /// Create a WebSocket listener for real-time events.
    pub fn listener(&self) -> Listener {
        Listener::new(self.ctx.clone(), self.ws_urls.clone())
    }

    // --- existing ---
    pub async fn get_user_info(&self, user_ids: &[&str]) -> Result<UserInfoResponse> {
        get_user_info::get_user_info(&self.ctx, &self.zpw_service_map, user_ids).await
    }

    pub async fn keep_alive(&self) -> Result<KeepAliveResponse> {
        keep_alive::keep_alive(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn send_message(
        &self,
        message: &str,
        thread_id: &str,
        thread_type: u8,
    ) -> Result<serde_json::Value> {
        send_message::send_message(&self.ctx, &self.zpw_service_map, message, thread_id, thread_type).await
    }

    // --- batch 1 ---
    pub async fn accept_friend_request(&self, friend_id: &str) -> Result<AcceptFriendRequestResponse> {
        accept_friend_request::accept_friend_request(&self.ctx, &self.zpw_service_map, friend_id).await
    }

    pub async fn add_group_blocked_member(
        &self,
        member_ids: Vec<String>,
        group_id: &str,
    ) -> Result<AddGroupBlockedMemberResponse> {
        add_group_blocked_member::add_group_blocked_member(&self.ctx, &self.zpw_service_map, member_ids, group_id).await
    }

    pub async fn add_group_deputy(
        &self,
        member_ids: Vec<String>,
        group_id: &str,
    ) -> Result<AddGroupDeputyResponse> {
        add_group_deputy::add_group_deputy(&self.ctx, &self.zpw_service_map, member_ids, group_id).await
    }

    pub async fn add_poll_options(&self, payload: AddPollOptionsPayload) -> Result<AddPollOptionsResponse> {
        add_poll_options::add_poll_options(&self.ctx, &self.zpw_service_map, payload).await
    }

    pub async fn add_quick_message(&self, payload: AddQuickMessagePayload) -> Result<AddQuickMessageResponse> {
        add_quick_message::add_quick_message(&self.ctx, &self.zpw_service_map, payload).await
    }

    pub async fn add_reaction(&self, icon: Value, dest: AddReactionDestination) -> Result<Value> {
        add_reaction::add_reaction(&self.ctx, &self.zpw_service_map, icon, dest).await
    }

    pub async fn add_unread_mark(&self, thread_id: &str, thread_type: ThreadType) -> Result<AddUnreadMarkResponse> {
        add_unread_mark::add_unread_mark(&self.ctx, &self.zpw_service_map, thread_id, thread_type).await
    }

    pub async fn add_user_to_group(
        &self,
        member_ids: Vec<String>,
        group_id: &str,
    ) -> Result<AddUserToGroupResponse> {
        add_user_to_group::add_user_to_group(&self.ctx, &self.zpw_service_map, member_ids, group_id).await
    }

    pub async fn block_user(&self, user_id: &str) -> Result<BlockUserResponse> {
        block_user::block_user(&self.ctx, &self.zpw_service_map, user_id).await
    }

    pub async fn block_view_feed(&self, is_block_feed: bool, user_id: &str) -> Result<BlockViewFeedResponse> {
        block_view_feed::block_view_feed(&self.ctx, &self.zpw_service_map, is_block_feed, user_id).await
    }

    pub async fn change_account_avatar(&self, avatar_path: &str) -> Result<ChangeAccountAvatarResponse> {
        change_account_avatar::change_account_avatar(&self.ctx, &self.zpw_service_map, avatar_path).await
    }

    pub async fn change_friend_alias(&self, alias: &str, friend_id: &str) -> Result<ChangeFriendAliasResponse> {
        change_friend_alias::change_friend_alias(&self.ctx, &self.zpw_service_map, alias, friend_id).await
    }

    pub async fn change_group_avatar(&self, avatar_path: &str, group_id: &str) -> Result<ChangeGroupAvatarResponse> {
        change_group_avatar::change_group_avatar(&self.ctx, &self.zpw_service_map, avatar_path, group_id).await
    }

    pub async fn change_group_name(&self, name: &str, group_id: &str) -> Result<ChangeGroupNameResponse> {
        change_group_name::change_group_name(&self.ctx, &self.zpw_service_map, name, group_id).await
    }

    pub async fn change_group_owner(&self, member_id: &str, group_id: &str) -> Result<ChangeGroupOwnerResponse> {
        change_group_owner::change_group_owner(&self.ctx, &self.zpw_service_map, member_id, group_id).await
    }

    pub async fn create_auto_reply(&self, payload: CreateAutoReplyPayload) -> Result<CreateAutoReplyResponse> {
        create_auto_reply::create_auto_reply(&self.ctx, &self.zpw_service_map, payload).await
    }

    pub async fn create_bank_account(&self, payload: CreateBankAccountPayload) -> Result<CreateBankAccountResponse> {
        create_bank_account::create_bank_account(&self.ctx, &self.zpw_service_map, payload).await
    }

    pub async fn create_catalog(&self, catalog_name: &str) -> Result<CreateCatalogResponse> {
        create_catalog::create_catalog(&self.ctx, &self.zpw_service_map, catalog_name).await
    }

    pub async fn create_group(&self, options: CreateGroupOptions) -> Result<CreateGroupResponse> {
        create_group::create_group(&self.ctx, &self.zpw_service_map, options).await
    }

    pub async fn create_note(&self, options: CreateNoteOptions, group_id: &str) -> Result<CreateNoteResponse> {
        create_note::create_note(&self.ctx, &self.zpw_service_map, options, group_id).await
    }

    pub async fn create_poll(&self, options: CreatePollOptions, group_id: &str) -> Result<CreatePollResponse> {
        create_poll::create_poll(&self.ctx, &self.zpw_service_map, options, group_id).await
    }

    pub async fn create_product_catalog(&self, payload: CreateProductCatalogPayload) -> Result<CreateProductCatalogResponse> {
        create_product_catalog::create_product_catalog(&self.ctx, &self.zpw_service_map, payload).await
    }

    pub async fn create_reminder(
        &self,
        options: CreateReminderOptions,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<CreateReminderResponse> {
        create_reminder::create_reminder(&self.ctx, &self.zpw_service_map, options, thread_id, thread_type).await
    }

    pub async fn delete_auto_reply(&self, id: i64) -> Result<DeleteAutoReplyResponse> {
        delete_auto_reply::delete_auto_reply(&self.ctx, &self.zpw_service_map, id).await
    }

    pub async fn delete_avatar(&self, photo_ids: Vec<String>) -> Result<DeleteAvatarResponse> {
        delete_avatar::delete_avatar(&self.ctx, &self.zpw_service_map, photo_ids).await
    }

    pub async fn delete_bank_account(&self, payload: DeleteBankAccountPayload) -> Result<DeleteBankAccountResponse> {
        delete_bank_account::delete_bank_account(&self.ctx, &self.zpw_service_map, payload).await
    }

    pub async fn delete_catalog(&self, catalog_id: &str) -> Result<DeleteCatalogResponse> {
        delete_catalog::delete_catalog(&self.ctx, &self.zpw_service_map, catalog_id).await
    }

    pub async fn delete_chat(
        &self,
        last_message: DeleteChatLastMessage,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<DeleteChatResponse> {
        delete_chat::delete_chat(&self.ctx, &self.zpw_service_map, last_message, thread_id, thread_type).await
    }

    pub async fn delete_group_invite_box(
        &self,
        group_ids: Vec<String>,
        block_future_invite: bool,
    ) -> Result<DeleteGroupInviteBoxResponse> {
        delete_group_invite_box::delete_group_invite_box(&self.ctx, &self.zpw_service_map, group_ids, block_future_invite).await
    }

    pub async fn delete_message(&self, dest: DeleteMessageDestination, only_me: bool) -> Result<DeleteMessageResponse> {
        delete_message::delete_message(&self.ctx, &self.zpw_service_map, dest, only_me).await
    }

    pub async fn delete_product_catalog(&self, payload: DeleteProductCatalogPayload) -> Result<DeleteProductCatalogResponse> {
        delete_product_catalog::delete_product_catalog(&self.ctx, &self.zpw_service_map, payload).await
    }

    pub async fn disable_group_link(&self, group_id: &str) -> Result<DisableGroupLinkResponse> {
        disable_group_link::disable_group_link(&self.ctx, &self.zpw_service_map, group_id).await
    }

    pub async fn disperse_group(&self, group_id: &str) -> Result<DisperseGroupResponse> {
        disperse_group::disperse_group(&self.ctx, &self.zpw_service_map, group_id).await
    }

    pub async fn edit_note(&self, options: EditNoteOptions, group_id: &str) -> Result<EditNoteResponse> {
        edit_note::edit_note(&self.ctx, &self.zpw_service_map, options, group_id).await
    }

    pub async fn edit_reminder(
        &self,
        options: EditReminderOptions,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<EditReminderResponse> {
        edit_reminder::edit_reminder(&self.ctx, &self.zpw_service_map, options, thread_id, thread_type).await
    }

    pub async fn enable_group_link(&self, group_id: &str) -> Result<EnableGroupLinkResponse> {
        enable_group_link::enable_group_link(&self.ctx, &self.zpw_service_map, group_id).await
    }

    pub async fn fetch_account_info(&self) -> Result<FetchAccountInfoResponse> {
        fetch_account_info::fetch_account_info(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn find_user(&self, phone_number: &str, avatar_size: Option<AvatarSize>) -> Result<FindUserResponse> {
        find_user::find_user(&self.ctx, &self.zpw_service_map, phone_number, avatar_size).await
    }

    pub async fn find_user_by_username(
        &self,
        username: &str,
        avatar_size: Option<AvatarSize>,
    ) -> Result<FindUserByUsernameResponse> {
        find_user_by_username::find_user_by_username(&self.ctx, &self.zpw_service_map, username, avatar_size).await
    }
}
