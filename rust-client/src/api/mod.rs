//! Zalo API surface: holds session context and exposes endpoint methods.

pub mod client;
pub mod url;

pub use client::{ApiClient, ZpwServiceMap};
pub use url::service_url;

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
pub mod get_alias_list;
pub mod get_all_friends;
pub mod get_all_groups;
pub mod get_archived_chat_list;
pub mod get_auto_delete_chat;
pub mod get_auto_reply_list;
pub mod get_avatar_list;
pub mod get_avatar_url_profile;
pub mod get_biz_account;
pub mod get_catalog_list;
pub mod get_close_friends;
pub mod get_context;
pub mod get_cookie;
pub mod get_friend_board_list;
pub mod get_friend_onlines;
pub mod get_friend_recommendations;
pub mod get_friend_request_status;
pub mod get_full_avatar;
pub mod get_group_blocked_member;
pub mod get_group_chat_history;
pub mod get_group_info;
pub mod get_group_invite_box_info;
pub mod get_group_invite_box_list;
pub mod get_group_link_detail;
pub mod get_group_link_info;
pub mod get_group_members_info;
pub mod get_hidden_conversations;
pub mod get_labels;
pub mod get_list_bank;
pub mod get_list_bank_account;
pub mod get_list_board;
pub mod get_list_device;
pub mod get_list_reminder;
pub mod get_multi_users_by_phones;
pub mod get_mute;
pub mod get_own_id;
pub mod get_pending_group_members;
pub mod get_pin_conversations;
pub mod get_poll_detail;
pub mod get_product_catalog_list;
pub mod get_qr;
pub mod get_quick_message_list;
pub mod get_related_friend_group;
pub mod get_reminder;
pub mod get_reminder_responses;
pub mod get_sent_friend_request;
pub mod get_settings;
pub mod get_sticker_category_detail;
pub mod get_stickers;
pub mod get_stickers_detail;
pub mod get_unread_mark;
pub mod invite_user_to_groups;
pub mod join_group_invite_box;
pub mod join_group_link;
pub mod last_online;
pub mod leave_group;
pub mod lock_poll;
pub mod lost_focus;
pub mod parse_link;
pub mod register_catalog;
pub mod reject_friend_request;
pub mod remove_friend;
pub mod remove_friend_alias;
pub mod remove_group_blocked_member;
pub mod remove_group_deputy;
pub mod remove_quick_message;
pub mod remove_reminder;
pub mod remove_unread_mark;
pub mod remove_user_from_group;
pub mod reset_hidden_convers_pin;
pub mod reuse_avatar;
pub mod review_pending_member_request;
pub mod scan_url;
pub mod search_sticker;
pub mod send_bank_card;
pub mod send_card;
pub mod send_delivered_event;
pub mod send_friend_request;
pub mod send_link;
pub mod send_report;

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

pub use get_alias_list::{AliasItem, GetAliasListResponse};
pub use get_all_friends::GetAllFriendsResponse;
pub use get_all_groups::GetAllGroupsResponse;
pub use get_archived_chat_list::GetArchivedChatListResponse;
pub use get_auto_delete_chat::{AutoDeleteConvers, GetAutoDeleteChatResponse};
pub use get_auto_reply_list::GetAutoReplyListResponse;
pub use get_avatar_list::{AvatarPhoto, GetAvatarListResponse};
pub use get_avatar_url_profile::{AvatarUrlInfo, GetAvatarUrlProfileResponse};
pub use get_biz_account::{BizInfo, GetBizAccountResponse, SettingStartPage};
pub use get_catalog_list::{GetCatalogListPayload, GetCatalogListResponse};
pub use get_close_friends::GetCloseFriendsResponse;
pub use get_context::GetContextResponse;
pub use get_cookie::GetCookieResponse;
pub use get_friend_board_list::GetFriendBoardListResponse;
pub use get_friend_onlines::{GetFriendOnlinesResponse, GetFriendOnlinesStatus};
pub use get_friend_recommendations::GetFriendRecommendationsResponse;
pub use get_friend_request_status::GetFriendRequestStatusResponse;
pub use get_full_avatar::GetFullAvatarResponse;
pub use get_group_blocked_member::GetGroupBlockedMemberResponse;
pub use get_group_chat_history::GetGroupChatHistoryResponse;
pub use get_group_info::{GetGroupInfoResponse, GroupInfoPendingApprove, GroupInfoWithExtra};
pub use get_group_invite_box_info::GetGroupInviteBoxInfoResponse;
pub use get_group_invite_box_list::GetGroupInviteBoxListResponse;
pub use get_group_link_detail::GetGroupLinkDetailResponse;
pub use get_group_link_info::GetGroupLinkInfoResponse;
pub use get_group_members_info::GetGroupMembersInfoResponse;
pub use get_hidden_conversations::GetHiddenConversationsResponse;
pub use get_labels::GetLabelsResponse;
pub use get_list_bank::GetListBankResponse;
pub use get_list_bank_account::GetListBankAccountResponse;
pub use get_list_board::GetListBoardResponse;
pub use get_list_device::GetListDeviceResponse;
pub use get_list_reminder::GetListReminderResponse;
pub use get_multi_users_by_phones::GetMultiUsersByPhonesResponse;
pub use get_mute::GetMuteResponse;
pub use get_own_id::GetOwnIdResponse;
pub use get_pending_group_members::GetPendingGroupMembersResponse;
pub use get_pin_conversations::GetPinConversationsResponse;
pub use get_poll_detail::GetPollDetailResponse;
pub use get_product_catalog_list::GetProductCatalogListResponse;
pub use get_qr::GetQrResponse;
pub use get_quick_message_list::GetQuickMessageListResponse;
pub use get_related_friend_group::GetRelatedFriendGroupResponse;
pub use get_reminder::GetReminderResponse;
pub use get_reminder_responses::GetReminderResponsesResponse;
pub use get_sent_friend_request::GetSentFriendRequestResponse;
pub use get_settings::GetSettingsResponse;
pub use get_sticker_category_detail::GetStickerCategoryDetailResponse;
pub use get_stickers::GetStickersResponse;
pub use get_stickers_detail::GetStickersDetailResponse;
pub use get_unread_mark::GetUnreadMarkResponse;
pub use invite_user_to_groups::InviteUserToGroupsResponse;
pub use join_group_invite_box::JoinGroupInviteBoxResponse;
pub use join_group_link::JoinGroupLinkResponse;
pub use last_online::LastOnlineResponse;
pub use leave_group::LeaveGroupResponse;
pub use lock_poll::LockPollResponse;
pub use lost_focus::LostFocusResponse;
pub use parse_link::ParseLinkResponse;
pub use register_catalog::RegisterCatalogResponse;
pub use reject_friend_request::RejectFriendRequestResponse;
pub use remove_friend::RemoveFriendResponse;
pub use remove_friend_alias::RemoveFriendAliasResponse;
pub use remove_group_blocked_member::RemoveGroupBlockedMemberResponse;
pub use remove_group_deputy::RemoveGroupDeputyResponse;
pub use remove_quick_message::RemoveQuickMessageResponse;
pub use remove_reminder::RemoveReminderResponse;
pub use remove_unread_mark::RemoveUnreadMarkResponse;
pub use remove_user_from_group::RemoveUserFromGroupResponse;
pub use reset_hidden_convers_pin::ResetHiddenConversPinResponse;
pub use reuse_avatar::ReuseAvatarResponse;
pub use review_pending_member_request::{ReviewPendingMemberRequestPayload, ReviewPendingMemberRequestResponse};
pub use scan_url::ScanUrlResponse;
pub use search_sticker::SearchStickerResponse;
pub use send_bank_card::{SendBankCardPayload, SendBankCardResponse};
pub use send_card::{SendCardOptions, SendCardResponse};
pub use send_delivered_event::{SendDeliveredEventMessageParams, SendDeliveredEventResponse};
pub use send_friend_request::SendFriendRequestResponse;
pub use send_link::{SendLinkOptions, SendLinkResponse};
pub use send_report::{ReportReason, SendReportOptions, SendReportResponse};

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


    /// Build an idiomatic [`ApiClient`] bound to this session.
    fn api_client(&self) -> ApiClient {
        ApiClient::new(self.ctx.clone(), self.zpw_service_map.clone())
    }

    // --- existing ---
    pub async fn get_user_info(&self, user_ids: &[&str]) -> Result<UserInfoResponse> {
        let client = self.api_client();
        get_user_info::get_user_info(&client, user_ids).await
    }

    pub async fn keep_alive(&self) -> Result<KeepAliveResponse> {
        let client = self.api_client();
        keep_alive::keep_alive(&client).await
    }

    pub async fn send_message(
        &self,
        message: &str,
        thread_id: &str,
        thread_type: u8,
    ) -> Result<serde_json::Value> {
        let client = self.api_client();
        send_message::send_message(&client, message, thread_id, thread_type).await
    }

    // --- batch 1 ---
    pub async fn accept_friend_request(&self, friend_id: &str) -> Result<AcceptFriendRequestResponse> {
        let client = self.api_client();
        accept_friend_request::accept_friend_request(&client, friend_id).await
    }

    pub async fn add_group_blocked_member(
        &self,
        member_ids: Vec<String>,
        group_id: &str,
    ) -> Result<AddGroupBlockedMemberResponse> {
        let client = self.api_client();
        add_group_blocked_member::add_group_blocked_member(&client, member_ids, group_id).await
    }

    pub async fn add_group_deputy(
        &self,
        member_ids: Vec<String>,
        group_id: &str,
    ) -> Result<AddGroupDeputyResponse> {
        let client = self.api_client();
        add_group_deputy::add_group_deputy(&client, member_ids, group_id).await
    }

    pub async fn add_poll_options(&self, payload: AddPollOptionsPayload) -> Result<AddPollOptionsResponse> {
        let client = self.api_client();
        add_poll_options::add_poll_options(&client, payload).await
    }

    pub async fn add_quick_message(&self, payload: AddQuickMessagePayload) -> Result<AddQuickMessageResponse> {
        let client = self.api_client();
        add_quick_message::add_quick_message(&client, payload).await
    }

    pub async fn add_reaction(&self, icon: Value, dest: AddReactionDestination) -> Result<Value> {
        let client = self.api_client();
        add_reaction::add_reaction(&client, icon, dest).await
    }

    pub async fn add_unread_mark(&self, thread_id: &str, thread_type: ThreadType) -> Result<AddUnreadMarkResponse> {
        let client = self.api_client();
        add_unread_mark::add_unread_mark(&client, thread_id, thread_type).await
    }

    pub async fn add_user_to_group(
        &self,
        member_ids: Vec<String>,
        group_id: &str,
    ) -> Result<AddUserToGroupResponse> {
        let client = self.api_client();
        add_user_to_group::add_user_to_group(&client, member_ids, group_id).await
    }

    pub async fn block_user(&self, user_id: &str) -> Result<BlockUserResponse> {
        let client = self.api_client();
        block_user::block_user(&client, user_id).await
    }

    pub async fn block_view_feed(&self, is_block_feed: bool, user_id: &str) -> Result<BlockViewFeedResponse> {
        let client = self.api_client();
        block_view_feed::block_view_feed(&client, is_block_feed, user_id).await
    }

    pub async fn change_account_avatar(&self, avatar_path: &str) -> Result<ChangeAccountAvatarResponse> {
        let client = self.api_client();
        change_account_avatar::change_account_avatar(&client, avatar_path).await
    }

    pub async fn change_friend_alias(&self, alias: &str, friend_id: &str) -> Result<ChangeFriendAliasResponse> {
        let client = self.api_client();
        change_friend_alias::change_friend_alias(&client, alias, friend_id).await
    }

    pub async fn change_group_avatar(&self, avatar_path: &str, group_id: &str) -> Result<ChangeGroupAvatarResponse> {
        let client = self.api_client();
        change_group_avatar::change_group_avatar(&client, avatar_path, group_id).await
    }

    pub async fn change_group_name(&self, name: &str, group_id: &str) -> Result<ChangeGroupNameResponse> {
        let client = self.api_client();
        change_group_name::change_group_name(&client, name, group_id).await
    }

    pub async fn change_group_owner(&self, member_id: &str, group_id: &str) -> Result<ChangeGroupOwnerResponse> {
        let client = self.api_client();
        change_group_owner::change_group_owner(&client, member_id, group_id).await
    }

    pub async fn create_auto_reply(&self, payload: CreateAutoReplyPayload) -> Result<CreateAutoReplyResponse> {
        let client = self.api_client();
        create_auto_reply::create_auto_reply(&client, payload).await
    }

    pub async fn create_bank_account(&self, payload: CreateBankAccountPayload) -> Result<CreateBankAccountResponse> {
        let client = self.api_client();
        create_bank_account::create_bank_account(&client, payload).await
    }

    pub async fn create_catalog(&self, catalog_name: &str) -> Result<CreateCatalogResponse> {
        let client = self.api_client();
        create_catalog::create_catalog(&client, catalog_name).await
    }

    pub async fn create_group(&self, options: CreateGroupOptions) -> Result<CreateGroupResponse> {
        let client = self.api_client();
        create_group::create_group(&client, options).await
    }

    pub async fn create_note(&self, options: CreateNoteOptions, group_id: &str) -> Result<CreateNoteResponse> {
        let client = self.api_client();
        create_note::create_note(&client, options, group_id).await
    }

    pub async fn create_poll(&self, options: CreatePollOptions, group_id: &str) -> Result<CreatePollResponse> {
        let client = self.api_client();
        create_poll::create_poll(&client, options, group_id).await
    }

    pub async fn create_product_catalog(&self, payload: CreateProductCatalogPayload) -> Result<CreateProductCatalogResponse> {
        let client = self.api_client();
        create_product_catalog::create_product_catalog(&client, payload).await
    }

    pub async fn create_reminder(
        &self,
        options: CreateReminderOptions,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<CreateReminderResponse> {
        let client = self.api_client();
        create_reminder::create_reminder(&client, options, thread_id, thread_type).await
    }

    pub async fn delete_auto_reply(&self, id: i64) -> Result<DeleteAutoReplyResponse> {
        let client = self.api_client();
        delete_auto_reply::delete_auto_reply(&client, id).await
    }

    pub async fn delete_avatar(&self, photo_ids: Vec<String>) -> Result<DeleteAvatarResponse> {
        let client = self.api_client();
        delete_avatar::delete_avatar(&client, photo_ids).await
    }

    pub async fn delete_bank_account(&self, payload: DeleteBankAccountPayload) -> Result<DeleteBankAccountResponse> {
        let client = self.api_client();
        delete_bank_account::delete_bank_account(&client, payload).await
    }

    pub async fn delete_catalog(&self, catalog_id: &str) -> Result<DeleteCatalogResponse> {
        let client = self.api_client();
        delete_catalog::delete_catalog(&client, catalog_id).await
    }

    pub async fn delete_chat(
        &self,
        last_message: DeleteChatLastMessage,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<DeleteChatResponse> {
        let client = self.api_client();
        delete_chat::delete_chat(&client, last_message, thread_id, thread_type).await
    }

    pub async fn delete_group_invite_box(
        &self,
        group_ids: Vec<String>,
        block_future_invite: bool,
    ) -> Result<DeleteGroupInviteBoxResponse> {
        let client = self.api_client();
        delete_group_invite_box::delete_group_invite_box(&client, group_ids, block_future_invite).await
    }

    pub async fn delete_message(&self, dest: DeleteMessageDestination, only_me: bool) -> Result<DeleteMessageResponse> {
        let client = self.api_client();
        delete_message::delete_message(&client, dest, only_me).await
    }

    pub async fn delete_product_catalog(&self, payload: DeleteProductCatalogPayload) -> Result<DeleteProductCatalogResponse> {
        let client = self.api_client();
        delete_product_catalog::delete_product_catalog(&client, payload).await
    }

    pub async fn disable_group_link(&self, group_id: &str) -> Result<DisableGroupLinkResponse> {
        let client = self.api_client();
        disable_group_link::disable_group_link(&client, group_id).await
    }

    pub async fn disperse_group(&self, group_id: &str) -> Result<DisperseGroupResponse> {
        let client = self.api_client();
        disperse_group::disperse_group(&client, group_id).await
    }

    pub async fn edit_note(&self, options: EditNoteOptions, group_id: &str) -> Result<EditNoteResponse> {
        let client = self.api_client();
        edit_note::edit_note(&client, options, group_id).await
    }

    pub async fn edit_reminder(
        &self,
        options: EditReminderOptions,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<EditReminderResponse> {
        let client = self.api_client();
        edit_reminder::edit_reminder(&client, options, thread_id, thread_type).await
    }

    pub async fn enable_group_link(&self, group_id: &str) -> Result<EnableGroupLinkResponse> {
        let client = self.api_client();
        enable_group_link::enable_group_link(&client, group_id).await
    }

    pub async fn fetch_account_info(&self) -> Result<FetchAccountInfoResponse> {
        let client = self.api_client();
        fetch_account_info::fetch_account_info(&client).await
    }

    pub async fn find_user(&self, phone_number: &str, avatar_size: Option<AvatarSize>) -> Result<FindUserResponse> {
        let client = self.api_client();
        find_user::find_user(&client, phone_number, avatar_size).await
    }

    pub async fn find_user_by_username(
        &self,
        username: &str,
        avatar_size: Option<AvatarSize>,
    ) -> Result<FindUserByUsernameResponse> {
        let client = self.api_client();
        find_user_by_username::find_user_by_username(&client, username, avatar_size).await
    }

    pub fn get_own_id(&self) -> Result<GetOwnIdResponse> {
        get_own_id::get_own_id(&self.ctx)
    }

    pub fn get_cookie(&self) -> Result<GetCookieResponse> {
        get_cookie::get_cookie(&self.ctx)
    }

    pub fn get_context(&self) -> Result<GetContextResponse> {
        get_context::get_context(&self.ctx)
    }

    pub async fn get_alias_list(&self, count: Option<i32>, page: Option<i32>) -> Result<GetAliasListResponse> {
        get_alias_list::get_alias_list(&self.ctx, &self.zpw_service_map, count, page).await
    }

    pub async fn get_all_friends(&self, count: Option<i32>, page: Option<i32>, avatar_size: Option<AvatarSize>) -> Result<GetAllFriendsResponse> {
        get_all_friends::get_all_friends(&self.ctx, &self.zpw_service_map, count, page, avatar_size).await
    }

    pub async fn get_all_groups(&self) -> Result<GetAllGroupsResponse> {
        get_all_groups::get_all_groups(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_archived_chat_list(&self) -> Result<GetArchivedChatListResponse> {
        get_archived_chat_list::get_archived_chat_list(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_auto_delete_chat(&self) -> Result<GetAutoDeleteChatResponse> {
        get_auto_delete_chat::get_auto_delete_chat(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_auto_reply_list(&self) -> Result<GetAutoReplyListResponse> {
        get_auto_reply_list::get_auto_reply_list(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_avatar_list(&self, count: Option<i32>, page: Option<i32>) -> Result<GetAvatarListResponse> {
        get_avatar_list::get_avatar_list(&self.ctx, &self.zpw_service_map, count, page).await
    }

    pub async fn get_avatar_url_profile(&self, friend_ids: Vec<String>, avatar_size: Option<AvatarSize>) -> Result<GetAvatarUrlProfileResponse> {
        get_avatar_url_profile::get_avatar_url_profile(&self.ctx, &self.zpw_service_map, friend_ids, avatar_size).await
    }

    pub async fn get_biz_account(&self, friend_id: &str) -> Result<GetBizAccountResponse> {
        get_biz_account::get_biz_account(&self.ctx, &self.zpw_service_map, friend_id).await
    }

    pub async fn get_catalog_list(&self, payload: Option<GetCatalogListPayload>) -> Result<GetCatalogListResponse> {
        get_catalog_list::get_catalog_list(&self.ctx, &self.zpw_service_map, payload).await
    }

    pub async fn get_close_friends(&self) -> Result<GetCloseFriendsResponse> {
        get_close_friends::get_close_friends(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_friend_board_list(&self) -> Result<GetFriendBoardListResponse> {
        get_friend_board_list::get_friend_board_list(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_friend_onlines(&self) -> Result<GetFriendOnlinesResponse> {
        get_friend_onlines::get_friend_onlines(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_friend_recommendations(&self) -> Result<GetFriendRecommendationsResponse> {
        get_friend_recommendations::get_friend_recommendations(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_friend_request_status(&self, friend_id: &str) -> Result<GetFriendRequestStatusResponse> {
        get_friend_request_status::get_friend_request_status(&self.ctx, &self.zpw_service_map, friend_id).await
    }

    pub async fn get_full_avatar(&self, user_id: &str) -> Result<GetFullAvatarResponse> {
        get_full_avatar::get_full_avatar(&self.ctx, &self.zpw_service_map, user_id).await
    }

    pub async fn get_group_blocked_member(&self, group_id: &str) -> Result<GetGroupBlockedMemberResponse> {
        get_group_blocked_member::get_group_blocked_member(&self.ctx, &self.zpw_service_map, group_id).await
    }

    pub async fn get_group_chat_history(&self, group_id: &str, last_id: Option<i64>, count: Option<i32>) -> Result<GetGroupChatHistoryResponse> {
        get_group_chat_history::get_group_chat_history(&self.ctx, &self.zpw_service_map, group_id, last_id, count).await
    }

    pub async fn get_group_info(&self, group_ids: Vec<String>) -> Result<GetGroupInfoResponse> {
        get_group_info::get_group_info(&self.ctx, &self.zpw_service_map, group_ids).await
    }

    pub async fn get_group_invite_box_info(&self, group_id: &str) -> Result<GetGroupInviteBoxInfoResponse> {
        get_group_invite_box_info::get_group_invite_box_info(&self.ctx, &self.zpw_service_map, group_id).await
    }

    pub async fn get_group_invite_box_list(&self) -> Result<GetGroupInviteBoxListResponse> {
        get_group_invite_box_list::get_group_invite_box_list(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_group_link_detail(&self, link: &str) -> Result<GetGroupLinkDetailResponse> {
        get_group_link_detail::get_group_link_detail(&self.ctx, &self.zpw_service_map, link).await
    }

    pub async fn get_group_link_info(&self, group_id: &str) -> Result<GetGroupLinkInfoResponse> {
        get_group_link_info::get_group_link_info(&self.ctx, &self.zpw_service_map, group_id).await
    }

    pub async fn get_group_members_info(&self, group_id: &str, member_ids: Vec<String>) -> Result<GetGroupMembersInfoResponse> {
        get_group_members_info::get_group_members_info(&self.ctx, &self.zpw_service_map, group_id, member_ids).await
    }

    pub async fn get_hidden_conversations(&self) -> Result<GetHiddenConversationsResponse> {
        get_hidden_conversations::get_hidden_conversations(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_labels(&self) -> Result<GetLabelsResponse> {
        get_labels::get_labels(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_list_bank(&self) -> Result<GetListBankResponse> {
        get_list_bank::get_list_bank(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_list_bank_account(&self) -> Result<GetListBankAccountResponse> {
        get_list_bank_account::get_list_bank_account(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_list_board(&self) -> Result<GetListBoardResponse> {
        get_list_board::get_list_board(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_list_device(&self) -> Result<GetListDeviceResponse> {
        get_list_device::get_list_device(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_list_reminder(&self) -> Result<GetListReminderResponse> {
        get_list_reminder::get_list_reminder(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_multi_users_by_phones(&self, phones: Vec<String>) -> Result<GetMultiUsersByPhonesResponse> {
        get_multi_users_by_phones::get_multi_users_by_phones(&self.ctx, &self.zpw_service_map, phones).await
    }

    pub async fn get_mute(&self) -> Result<GetMuteResponse> {
        get_mute::get_mute(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_pending_group_members(&self, group_id: &str) -> Result<GetPendingGroupMembersResponse> {
        get_pending_group_members::get_pending_group_members(&self.ctx, &self.zpw_service_map, group_id).await
    }

    pub async fn get_pin_conversations(&self) -> Result<GetPinConversationsResponse> {
        get_pin_conversations::get_pin_conversations(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_poll_detail(&self, poll_id: i64) -> Result<GetPollDetailResponse> {
        get_poll_detail::get_poll_detail(&self.ctx, &self.zpw_service_map, poll_id).await
    }

    pub async fn get_product_catalog_list(&self, catalog_id: &str) -> Result<GetProductCatalogListResponse> {
        get_product_catalog_list::get_product_catalog_list(&self.ctx, &self.zpw_service_map, catalog_id).await
    }

    // --- batch 3 ---
    pub async fn get_qr(&self, user_ids: Vec<String>) -> Result<GetQrResponse> {
        get_qr::get_qr(&self.ctx, &self.zpw_service_map, &user_ids).await
    }

    pub async fn get_quick_message_list(&self) -> Result<GetQuickMessageListResponse> {
        get_quick_message_list::get_quick_message_list(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_related_friend_group(&self, friend_ids: Vec<String>) -> Result<GetRelatedFriendGroupResponse> {
        get_related_friend_group::get_related_friend_group(&self.ctx, &self.zpw_service_map, &friend_ids).await
    }

    pub async fn get_reminder(&self, reminder_id: &str) -> Result<GetReminderResponse> {
        get_reminder::get_reminder(&self.ctx, &self.zpw_service_map, reminder_id).await
    }

    pub async fn get_reminder_responses(&self, reminder_id: &str) -> Result<GetReminderResponsesResponse> {
        get_reminder_responses::get_reminder_responses(&self.ctx, &self.zpw_service_map, reminder_id).await
    }

    pub async fn get_sent_friend_request(&self) -> Result<GetSentFriendRequestResponse> {
        get_sent_friend_request::get_sent_friend_request(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_settings(&self) -> Result<GetSettingsResponse> {
        get_settings::get_settings(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn get_sticker_category_detail(&self, cate_id: i64) -> Result<GetStickerCategoryDetailResponse> {
        get_sticker_category_detail::get_sticker_category_detail(&self.ctx, &self.zpw_service_map, cate_id).await
    }

    pub async fn get_stickers(&self, keyword: &str) -> Result<GetStickersResponse> {
        get_stickers::get_stickers(&self.ctx, &self.zpw_service_map, keyword).await
    }

    pub async fn get_stickers_detail(&self, sticker_ids: Vec<i64>) -> Result<GetStickersDetailResponse> {
        get_stickers_detail::get_stickers_detail(&self.ctx, &self.zpw_service_map, &sticker_ids).await
    }

    pub async fn get_unread_mark(&self) -> Result<GetUnreadMarkResponse> {
        get_unread_mark::get_unread_mark(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn invite_user_to_groups(&self, user_id: &str, group_ids: Vec<String>) -> Result<InviteUserToGroupsResponse> {
        invite_user_to_groups::invite_user_to_groups(&self.ctx, &self.zpw_service_map, user_id, &group_ids).await
    }

    pub async fn join_group_invite_box(&self, group_id: &str) -> Result<JoinGroupInviteBoxResponse> {
        join_group_invite_box::join_group_invite_box(&self.ctx, &self.zpw_service_map, group_id).await
    }

    pub async fn join_group_link(&self, link: &str) -> Result<JoinGroupLinkResponse> {
        join_group_link::join_group_link(&self.ctx, &self.zpw_service_map, link).await
    }

    pub async fn last_online(&self, uid: &str) -> Result<LastOnlineResponse> {
        last_online::last_online(&self.ctx, &self.zpw_service_map, uid).await
    }

    pub async fn leave_group(&self, group_id: &str, silent: bool) -> Result<LeaveGroupResponse> {
        leave_group::leave_group(&self.ctx, &self.zpw_service_map, group_id, silent).await
    }

    pub async fn lock_poll(&self, poll_id: i64) -> Result<LockPollResponse> {
        lock_poll::lock_poll(&self.ctx, &self.zpw_service_map, poll_id).await
    }

    pub async fn lost_focus(&self) -> Result<LostFocusResponse> {
        lost_focus::lost_focus(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn parse_link(&self, link: &str) -> Result<ParseLinkResponse> {
        parse_link::parse_link(&self.ctx, &self.zpw_service_map, link).await
    }

    pub async fn register_catalog(&self, enable: bool) -> Result<RegisterCatalogResponse> {
        register_catalog::register_catalog(&self.ctx, &self.zpw_service_map, enable).await
    }

    pub async fn reject_friend_request(&self, friend_id: &str) -> Result<RejectFriendRequestResponse> {
        reject_friend_request::reject_friend_request(&self.ctx, &self.zpw_service_map, friend_id).await
    }

    pub async fn remove_friend(&self, friend_id: &str) -> Result<RemoveFriendResponse> {
        remove_friend::remove_friend(&self.ctx, &self.zpw_service_map, friend_id).await
    }

    pub async fn remove_friend_alias(&self, friend_id: &str) -> Result<RemoveFriendAliasResponse> {
        remove_friend_alias::remove_friend_alias(&self.ctx, &self.zpw_service_map, friend_id).await
    }

    pub async fn remove_group_blocked_member(&self, member_ids: Vec<String>, group_id: &str) -> Result<RemoveGroupBlockedMemberResponse> {
        remove_group_blocked_member::remove_group_blocked_member(&self.ctx, &self.zpw_service_map, &member_ids, group_id).await
    }

    pub async fn remove_group_deputy(&self, member_ids: Vec<String>, group_id: &str) -> Result<RemoveGroupDeputyResponse> {
        remove_group_deputy::remove_group_deputy(&self.ctx, &self.zpw_service_map, &member_ids, group_id).await
    }

    pub async fn remove_quick_message(&self, item_ids: Vec<i64>) -> Result<RemoveQuickMessageResponse> {
        remove_quick_message::remove_quick_message(&self.ctx, &self.zpw_service_map, &item_ids).await
    }

    pub async fn remove_reminder(&self, reminder_id: &str, thread_id: &str, thread_type: ThreadType) -> Result<RemoveReminderResponse> {
        remove_reminder::remove_reminder(&self.ctx, &self.zpw_service_map, reminder_id, thread_id, thread_type).await
    }

    pub async fn remove_unread_mark(&self, thread_id: &str, thread_type: ThreadType) -> Result<RemoveUnreadMarkResponse> {
        remove_unread_mark::remove_unread_mark(&self.ctx, &self.zpw_service_map, thread_id, thread_type).await
    }

    pub async fn remove_user_from_group(&self, member_ids: Vec<String>, group_id: &str) -> Result<RemoveUserFromGroupResponse> {
        remove_user_from_group::remove_user_from_group(&self.ctx, &self.zpw_service_map, &member_ids, group_id).await
    }

    pub async fn reset_hidden_convers_pin(&self) -> Result<ResetHiddenConversPinResponse> {
        reset_hidden_convers_pin::reset_hidden_convers_pin(&self.ctx, &self.zpw_service_map).await
    }

    pub async fn reuse_avatar(&self, photo_id: &str) -> Result<ReuseAvatarResponse> {
        reuse_avatar::reuse_avatar(&self.ctx, &self.zpw_service_map, photo_id).await
    }

    pub async fn review_pending_member_request(&self, payload: ReviewPendingMemberRequestPayload, group_id: &str) -> Result<ReviewPendingMemberRequestResponse> {
        review_pending_member_request::review_pending_member_request(&self.ctx, &self.zpw_service_map, payload, group_id).await
    }

    pub async fn scan_url(&self, url: &str) -> Result<ScanUrlResponse> {
        scan_url::scan_url(&self.ctx, &self.zpw_service_map, url).await
    }

    pub async fn search_sticker(&self, keyword: &str, limit: Option<i32>) -> Result<SearchStickerResponse> {
        search_sticker::search_sticker(&self.ctx, &self.zpw_service_map, keyword, limit).await
    }

    pub async fn send_bank_card(&self, payload: SendBankCardPayload, thread_id: &str, thread_type: ThreadType) -> Result<SendBankCardResponse> {
        send_bank_card::send_bank_card(&self.ctx, &self.zpw_service_map, payload, thread_id, thread_type).await
    }

    pub async fn send_card(&self, options: SendCardOptions, thread_id: &str, thread_type: ThreadType) -> Result<SendCardResponse> {
        send_card::send_card(&self.ctx, &self.zpw_service_map, options, thread_id, thread_type).await
    }

    pub async fn send_delivered_event(&self, is_seen: bool, messages: Vec<SendDeliveredEventMessageParams>, thread_type: ThreadType) -> Result<SendDeliveredEventResponse> {
        send_delivered_event::send_delivered_event(&self.ctx, &self.zpw_service_map, is_seen, &messages, thread_type).await
    }

    pub async fn send_friend_request(&self, msg: &str, user_id: &str) -> Result<SendFriendRequestResponse> {
        send_friend_request::send_friend_request(&self.ctx, &self.zpw_service_map, msg, user_id).await
    }

    pub async fn send_link(&self, options: SendLinkOptions, thread_id: &str, thread_type: ThreadType) -> Result<SendLinkResponse> {
        send_link::send_link(&self.ctx, &self.zpw_service_map, options, thread_id, thread_type).await
    }

    pub async fn send_report(&self, options: SendReportOptions, thread_id: &str, thread_type: ThreadType) -> Result<SendReportResponse> {
        send_report::send_report(&self.ctx, &self.zpw_service_map, options, thread_id, thread_type).await
    }

}
