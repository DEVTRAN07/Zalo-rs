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
pub mod send_sticker;
pub mod send_typing_event;
pub mod send_video;
pub mod send_voice;

pub mod forward_message;
pub mod login_qr;
pub mod send_seen_event;
pub mod set_hidden_conversations;
pub mod set_mute;
pub mod set_pinned_conversations;
pub mod share_poll;
pub mod unblock_user;
pub mod undo;
pub mod undo_friend_request;
pub mod update_active_status;
pub mod update_archived_chat_list;
pub mod update_auto_delete_chat;
pub mod update_auto_reply;
pub mod update_bank_account;
pub mod update_catalog;
pub mod update_group_settings;

pub mod listen;
pub mod upgrade_group_to_community;
pub mod upload_attachment;
pub mod upload_product_photo;
pub mod update_hidden_convers_pin;
pub mod update_labels;
pub mod update_lang;
pub mod update_product_catalog;
pub mod update_profile;
pub mod update_profile_bio;
pub mod update_quick_message;
pub mod update_settings;
pub mod vote_poll;

pub mod get_user_info;
pub mod keep_alive;
pub mod login;
pub mod send_message;

use serde_json::Value;

use crate::context::ContextSession;
use crate::errors::Result;
use crate::models::r#enum::{AvatarSize, DestType, ThreadType};
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
pub use review_pending_member_request::{ReviewPendingMemberRequestType, ReviewPendingMemberRequestResponse};
pub use scan_url::ScanUrlResponse;
pub use search_sticker::SearchStickerResponse;
pub use send_bank_card::{SendBankCardPayload, SendBankCardResponse};
pub use send_card::{SendCardOptions, SendCardResponse};
pub use send_delivered_event::{DeliveredMessage, SendDeliveredEventResponse};
pub use send_friend_request::SendFriendRequestResponse;
pub use send_link::{SendLinkOptions, SendLinkResponse};
pub use send_report::{ReportOptions, SendReportResponse};
pub use send_sticker::{SendStickerPayload, SendStickerResponse};
pub use send_typing_event::SendTypingEventResponse;
pub use send_video::{SendVideoOptions, SendVideoResponse};
pub use send_voice::{SendVoiceOptions, SendVoiceResponse};
pub use send_message::{SendMessageOptions, Urgency};


pub use forward_message::{ForwardMessagePayload, ForwardMessageResponse, ForwardMessageSuccess, ForwardMessageFail};
pub use login_qr::{LoginQrResult, QrGenerateResponse, QrScanResponse, QrConfirmResponse, QrUserInfoResponse, login_qr_start, load_login_page, get_login_info as get_login_info_qr, verify_client as verify_client_qr, generate_qr, waiting_scan, waiting_confirm, check_session as check_session_qr, get_user_info_qr};
pub use send_seen_event::{SendSeenEventMessageParams, SendSeenEventResponse};
pub use set_hidden_conversations::SetHiddenConversationsResponse;
pub use set_mute::{MuteAction, MuteDuration, MuteDurationPreset, SetMuteResponse};
pub use set_pinned_conversations::SetPinnedConversationsResponse;
pub use share_poll::SharePollResponse;
pub use unblock_user::UnblockUserResponse;
pub use undo::{UndoPayload, UndoResponse};
pub use undo_friend_request::UndoFriendRequestResponse;
pub use update_active_status::UpdateActiveStatusResponse;
pub use update_archived_chat_list::{UpdateArchivedChatListTarget, UpdateArchivedChatListResponse};
pub use update_auto_delete_chat::{ChatTTL, UpdateAutoDeleteChatResponse};
pub use update_auto_reply::{UpdateAutoReplyPayload, UpdateAutoReplyResponse};
pub use update_bank_account::{UpdateBankAccountPayload, UpdateBankAccountResponse};
pub use update_catalog::{UpdateCatalogPayload, UpdateCatalogResponse};
pub use update_group_settings::{UpdateGroupSettingsOptions, UpdateGroupSettingsResponse};
pub use listen::{ListenOptions, ListenResponse};
pub use upgrade_group_to_community::UpgradeGroupToCommunityResponse;
pub use upload_attachment::{AttachmentSource, UploadAttachmentFileResponse, UploadAttachmentImageResponse, UploadAttachmentResponse, UploadAttachmentType, UploadAttachmentVideoResponse};
pub use upload_product_photo::{ProductPhotoSource, UploadProductPhotoPayload, UploadProductPhotoResponse};
pub use update_hidden_convers_pin::UpdateHiddenConversPinResponse;
pub use update_labels::{UpdateLabelsPayload, UpdateLabelsResponse};
pub use update_lang::{UpdateLangAvailableLanguages, UpdateLangResponse};
pub use update_product_catalog::{UpdateProductCatalogPayload, UpdateProductCatalogResponse};
pub use update_profile::{UpdateProfileBiz, UpdateProfileInfo, UpdateProfilePayload, UpdateProfileResponse};
pub use update_profile_bio::UpdateProfileBioResponse;
pub use update_quick_message::{UpdateQuickMessagePayload, UpdateQuickMessageResponse};
pub use update_settings::{UpdateSettingsResponse, UpdateSettingsType};
pub use vote_poll::VotePollResponse;

/// High-level API bound to an authenticated session.
#[derive(Clone)]
pub struct ZaloApi {
    api_client: ApiClient,
    ws_urls: Vec<String>,
}

impl ZaloApi {
    pub fn new(ctx: ContextSession, zpw_service_map: serde_json::Value, ws_urls: Vec<String>) -> Self {
        Self {
            api_client: ApiClient::new(ctx, zpw_service_map, ws_urls.clone()),
            ws_urls,
        }
    }

    pub fn api_client(&self) -> &ApiClient {
        &self.api_client
    }

    /// Get the authenticated user ID.
    pub fn uid(&self) -> &str {
        &self.api_client.ctx().uid
    }

    /// Access the session context.
    pub fn context(&self) -> &ContextSession {
        self.api_client.ctx()
    }

    /// Create a WebSocket listener for real-time events.
    pub fn listener(&self) -> Listener {
        Listener::new(self.api_client.ctx().clone(), self.ws_urls.clone())
    }

    // --- existing ---
    pub async fn get_user_info(&self, user_ids: &[&str]) -> Result<UserInfoResponse> {
        get_user_info::get_user_info(self.api_client(), user_ids, crate::models::r#enum::AvatarSize::Small).await
    }

    pub async fn keep_alive(&self) -> Result<KeepAliveResponse> {
        keep_alive::keep_alive(self.api_client()).await
    }

    pub async fn send_message(
        &self,
        message: &str,
        thread_id: &str,
        thread_type: u8,
    ) -> Result<serde_json::Value> {
        send_message::send_message(self.api_client(), message, thread_id, thread_type).await
    }

    pub async fn send_message_options(
        &self,
        options: send_message::SendMessageOptions,
    ) -> Result<serde_json::Value> {
        send_message::send_message_options(self.api_client(), options).await
    }

    // --- batch 1 ---
    pub async fn accept_friend_request(&self, friend_id: &str) -> Result<AcceptFriendRequestResponse> {
        accept_friend_request::accept_friend_request(self.api_client(), friend_id).await
    }

    pub async fn add_group_blocked_member(
        &self,
        member_ids: Vec<String>,
        group_id: &str,
    ) -> Result<AddGroupBlockedMemberResponse> {
        add_group_blocked_member::add_group_blocked_member(self.api_client(), member_ids, group_id).await
    }

    pub async fn add_group_deputy(
        &self,
        member_ids: Vec<String>,
        group_id: &str,
    ) -> Result<AddGroupDeputyResponse> {
        add_group_deputy::add_group_deputy(self.api_client(), member_ids, group_id).await
    }

    pub async fn add_poll_options(&self, payload: AddPollOptionsPayload) -> Result<AddPollOptionsResponse> {
        add_poll_options::add_poll_options(self.api_client(), payload).await
    }

    pub async fn add_quick_message(&self, payload: AddQuickMessagePayload) -> Result<AddQuickMessageResponse> {
        add_quick_message::add_quick_message(self.api_client(), payload).await
    }

    pub async fn add_reaction(&self, icon: Value, dest: AddReactionDestination) -> Result<Value> {
        add_reaction::add_reaction(self.api_client(), icon, dest).await
    }

    pub async fn add_unread_mark(&self, thread_id: &str, thread_type: ThreadType) -> Result<AddUnreadMarkResponse> {
        add_unread_mark::add_unread_mark(self.api_client(), thread_id, thread_type).await
    }

    pub async fn add_user_to_group(
        &self,
        member_ids: Vec<String>,
        group_id: &str,
    ) -> Result<AddUserToGroupResponse> {
        add_user_to_group::add_user_to_group(self.api_client(), member_ids, group_id).await
    }

    pub async fn block_user(&self, user_id: &str) -> Result<BlockUserResponse> {
        block_user::block_user(self.api_client(), user_id).await
    }

    pub async fn block_view_feed(&self, is_block_feed: bool, user_id: &str) -> Result<BlockViewFeedResponse> {
        block_view_feed::block_view_feed(self.api_client(), is_block_feed, user_id).await
    }

    pub async fn change_account_avatar(&self, avatar_path: &str) -> Result<ChangeAccountAvatarResponse> {
        change_account_avatar::change_account_avatar(self.api_client(), avatar_path).await
    }

    pub async fn change_friend_alias(&self, alias: &str, friend_id: &str) -> Result<ChangeFriendAliasResponse> {
        change_friend_alias::change_friend_alias(self.api_client(), alias, friend_id).await
    }

    pub async fn change_group_avatar(&self, avatar_path: &str, group_id: &str) -> Result<ChangeGroupAvatarResponse> {
        change_group_avatar::change_group_avatar(self.api_client(), avatar_path, group_id).await
    }

    pub async fn change_group_name(&self, name: &str, group_id: &str) -> Result<ChangeGroupNameResponse> {
        change_group_name::change_group_name(self.api_client(), name, group_id).await
    }

    pub async fn change_group_owner(&self, member_id: &str, group_id: &str) -> Result<ChangeGroupOwnerResponse> {
        change_group_owner::change_group_owner(self.api_client(), member_id, group_id).await
    }

    pub async fn create_auto_reply(&self, payload: CreateAutoReplyPayload) -> Result<CreateAutoReplyResponse> {
        create_auto_reply::create_auto_reply(self.api_client(), payload).await
    }

    pub async fn create_bank_account(&self, payload: CreateBankAccountPayload) -> Result<CreateBankAccountResponse> {
        create_bank_account::create_bank_account(self.api_client(), payload).await
    }

    pub async fn create_catalog(&self, catalog_name: &str) -> Result<CreateCatalogResponse> {
        create_catalog::create_catalog(self.api_client(), catalog_name).await
    }

    pub async fn create_group(&self, options: CreateGroupOptions) -> Result<CreateGroupResponse> {
        create_group::create_group(self.api_client(), options).await
    }

    pub async fn create_note(&self, options: CreateNoteOptions, group_id: &str) -> Result<CreateNoteResponse> {
        create_note::create_note(self.api_client(), options, group_id).await
    }

    pub async fn create_poll(&self, options: CreatePollOptions, group_id: &str) -> Result<CreatePollResponse> {
        create_poll::create_poll(self.api_client(), options, group_id).await
    }

    pub async fn create_product_catalog(&self, payload: CreateProductCatalogPayload) -> Result<CreateProductCatalogResponse> {
        create_product_catalog::create_product_catalog(self.api_client(), payload).await
    }

    pub async fn create_reminder(
        &self,
        options: CreateReminderOptions,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<CreateReminderResponse> {
        create_reminder::create_reminder(self.api_client(), options, thread_id, thread_type).await
    }

    pub async fn delete_auto_reply(&self, id: i64) -> Result<DeleteAutoReplyResponse> {
        delete_auto_reply::delete_auto_reply(self.api_client(), id).await
    }

    pub async fn delete_avatar(&self, photo_ids: Vec<String>) -> Result<DeleteAvatarResponse> {
        delete_avatar::delete_avatar(self.api_client(), photo_ids).await
    }

    pub async fn delete_bank_account(&self, payload: DeleteBankAccountPayload) -> Result<DeleteBankAccountResponse> {
        delete_bank_account::delete_bank_account(self.api_client(), payload).await
    }

    pub async fn delete_catalog(&self, catalog_id: &str) -> Result<DeleteCatalogResponse> {
        delete_catalog::delete_catalog(self.api_client(), catalog_id).await
    }

    pub async fn delete_chat(
        &self,
        last_message: DeleteChatLastMessage,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<DeleteChatResponse> {
        delete_chat::delete_chat(self.api_client(), last_message, thread_id, thread_type).await
    }

    pub async fn delete_group_invite_box(
        &self,
        group_ids: Vec<String>,
        block_future_invite: bool,
    ) -> Result<DeleteGroupInviteBoxResponse> {
        delete_group_invite_box::delete_group_invite_box(self.api_client(), group_ids, block_future_invite).await
    }

    pub async fn delete_message(&self, dest: DeleteMessageDestination, only_me: bool) -> Result<DeleteMessageResponse> {
        delete_message::delete_message(self.api_client(), dest, only_me).await
    }

    pub async fn delete_product_catalog(&self, payload: DeleteProductCatalogPayload) -> Result<DeleteProductCatalogResponse> {
        delete_product_catalog::delete_product_catalog(self.api_client(), payload).await
    }

    pub async fn disable_group_link(&self, group_id: &str) -> Result<DisableGroupLinkResponse> {
        disable_group_link::disable_group_link(self.api_client(), group_id).await
    }

    pub async fn disperse_group(&self, group_id: &str) -> Result<DisperseGroupResponse> {
        disperse_group::disperse_group(self.api_client(), group_id).await
    }

    pub async fn edit_note(&self, options: EditNoteOptions, group_id: &str) -> Result<EditNoteResponse> {
        edit_note::edit_note(self.api_client(), options, group_id).await
    }

    pub async fn edit_reminder(
        &self,
        options: EditReminderOptions,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<EditReminderResponse> {
        edit_reminder::edit_reminder(self.api_client(), options, thread_id, thread_type).await
    }

    pub async fn enable_group_link(&self, group_id: &str) -> Result<EnableGroupLinkResponse> {
        enable_group_link::enable_group_link(self.api_client(), group_id).await
    }

    pub async fn fetch_account_info(&self) -> Result<FetchAccountInfoResponse> {
        fetch_account_info::fetch_account_info(self.api_client()).await
    }

    pub async fn find_user(&self, phone_number: &str, avatar_size: Option<AvatarSize>) -> Result<FindUserResponse> {
        find_user::find_user(self.api_client(), phone_number, avatar_size).await
    }

    pub async fn find_user_by_username(
        &self,
        username: &str,
        avatar_size: Option<AvatarSize>,
    ) -> Result<FindUserByUsernameResponse> {
        find_user_by_username::find_user_by_username(self.api_client(), username, avatar_size).await
    }

    pub fn get_own_id(&self) -> Result<GetOwnIdResponse> {
        get_own_id::get_own_id(self.api_client())
    }

    pub async fn get_cookie(&self) -> Result<GetCookieResponse> {
        get_cookie::get_cookie(self.api_client()).await
    }

    pub fn get_context(&self) -> Result<GetContextResponse> {
        get_context::get_context(self.api_client())
    }

    pub async fn get_alias_list(&self, count: Option<i32>, page: Option<i32>) -> Result<GetAliasListResponse> {
        get_alias_list::get_alias_list(self.api_client(), count, page).await
    }

    pub async fn get_all_friends(&self, count: Option<i32>, page: Option<i32>, avatar_size: Option<AvatarSize>) -> Result<GetAllFriendsResponse> {
        get_all_friends::get_all_friends(self.api_client(), count, page, avatar_size).await
    }

    pub async fn get_all_groups(&self) -> Result<GetAllGroupsResponse> {
        get_all_groups::get_all_groups(self.api_client()).await
    }

    pub async fn get_archived_chat_list(&self) -> Result<GetArchivedChatListResponse> {
        get_archived_chat_list::get_archived_chat_list(self.api_client()).await
    }

    pub async fn get_auto_delete_chat(&self) -> Result<GetAutoDeleteChatResponse> {
        get_auto_delete_chat::get_auto_delete_chat(self.api_client()).await
    }

    pub async fn get_auto_reply_list(&self) -> Result<GetAutoReplyListResponse> {
        get_auto_reply_list::get_auto_reply_list(self.api_client()).await
    }

    pub async fn get_avatar_list(&self, count: Option<i32>, page: Option<i32>) -> Result<GetAvatarListResponse> {
        get_avatar_list::get_avatar_list(self.api_client(), count, page).await
    }

    pub async fn get_avatar_url_profile(&self, friend_ids: Vec<String>, avatar_size: Option<AvatarSize>) -> Result<GetAvatarUrlProfileResponse> {
        get_avatar_url_profile::get_avatar_url_profile(self.api_client(), friend_ids, avatar_size).await
    }

    pub async fn get_biz_account(&self, friend_id: &str) -> Result<GetBizAccountResponse> {
        get_biz_account::get_biz_account(self.api_client(), friend_id).await
    }

    pub async fn get_catalog_list(&self, payload: Option<GetCatalogListPayload>) -> Result<GetCatalogListResponse> {
        get_catalog_list::get_catalog_list(self.api_client(), payload).await
    }

    pub async fn get_close_friends(&self) -> Result<GetCloseFriendsResponse> {
        get_close_friends::get_close_friends(self.api_client()).await
    }

    pub async fn get_friend_board_list(&self) -> Result<GetFriendBoardListResponse> {
        get_friend_board_list::get_friend_board_list(self.api_client()).await
    }

    pub async fn get_friend_onlines(&self) -> Result<GetFriendOnlinesResponse> {
        get_friend_onlines::get_friend_onlines(self.api_client()).await
    }

    pub async fn get_friend_recommendations(&self) -> Result<GetFriendRecommendationsResponse> {
        get_friend_recommendations::get_friend_recommendations(self.api_client()).await
    }

    pub async fn get_friend_request_status(&self, friend_id: &str) -> Result<GetFriendRequestStatusResponse> {
        get_friend_request_status::get_friend_request_status(self.api_client(), friend_id).await
    }

    pub async fn get_full_avatar(&self, user_id: &str) -> Result<GetFullAvatarResponse> {
        get_full_avatar::get_full_avatar(self.api_client(), user_id).await
    }

    pub async fn get_group_blocked_member(&self, group_id: &str) -> Result<GetGroupBlockedMemberResponse> {
        get_group_blocked_member::get_group_blocked_member(self.api_client(), group_id).await
    }

    pub async fn get_group_chat_history(&self, group_id: &str, last_id: Option<i64>, count: Option<i32>) -> Result<GetGroupChatHistoryResponse> {
        get_group_chat_history::get_group_chat_history(self.api_client(), group_id, last_id, count).await
    }

    pub async fn get_group_info(&self, group_ids: Vec<String>) -> Result<GetGroupInfoResponse> {
        get_group_info::get_group_info(self.api_client(), group_ids).await
    }

    pub async fn get_group_invite_box_info(&self, group_id: &str) -> Result<GetGroupInviteBoxInfoResponse> {
        get_group_invite_box_info::get_group_invite_box_info(self.api_client(), group_id).await
    }

    pub async fn get_group_invite_box_list(&self) -> Result<GetGroupInviteBoxListResponse> {
        get_group_invite_box_list::get_group_invite_box_list(self.api_client()).await
    }

    pub async fn get_group_link_detail(&self, link: &str) -> Result<GetGroupLinkDetailResponse> {
        get_group_link_detail::get_group_link_detail(self.api_client(), link).await
    }

    pub async fn get_group_link_info(&self, group_id: &str) -> Result<GetGroupLinkInfoResponse> {
        get_group_link_info::get_group_link_info(self.api_client(), group_id).await
    }

    pub async fn get_group_members_info(&self, group_id: &str, member_ids: Vec<String>) -> Result<GetGroupMembersInfoResponse> {
        get_group_members_info::get_group_members_info(self.api_client(), group_id, member_ids).await
    }

    pub async fn get_hidden_conversations(&self) -> Result<GetHiddenConversationsResponse> {
        get_hidden_conversations::get_hidden_conversations(self.api_client()).await
    }

    pub async fn get_labels(&self) -> Result<GetLabelsResponse> {
        get_labels::get_labels(self.api_client()).await
    }

    pub async fn get_list_bank(&self) -> Result<GetListBankResponse> {
        get_list_bank::get_list_bank(self.api_client()).await
    }

    pub async fn get_list_bank_account(&self) -> Result<GetListBankAccountResponse> {
        get_list_bank_account::get_list_bank_account(self.api_client()).await
    }

    pub async fn get_list_board(&self) -> Result<GetListBoardResponse> {
        get_list_board::get_list_board(self.api_client()).await
    }

    pub async fn get_list_device(&self) -> Result<GetListDeviceResponse> {
        get_list_device::get_list_device(self.api_client()).await
    }

    pub async fn get_list_reminder(&self) -> Result<GetListReminderResponse> {
        get_list_reminder::get_list_reminder(self.api_client()).await
    }

    pub async fn get_multi_users_by_phones(&self, phones: Vec<String>) -> Result<GetMultiUsersByPhonesResponse> {
        get_multi_users_by_phones::get_multi_users_by_phones(self.api_client(), phones).await
    }

    pub async fn get_mute(&self) -> Result<GetMuteResponse> {
        get_mute::get_mute(self.api_client()).await
    }

    pub async fn get_pending_group_members(&self, group_id: &str) -> Result<GetPendingGroupMembersResponse> {
        get_pending_group_members::get_pending_group_members(self.api_client(), group_id).await
    }

    pub async fn get_pin_conversations(&self) -> Result<GetPinConversationsResponse> {
        get_pin_conversations::get_pin_conversations(self.api_client()).await
    }

    pub async fn get_poll_detail(&self, poll_id: i64) -> Result<GetPollDetailResponse> {
        get_poll_detail::get_poll_detail(self.api_client(), poll_id).await
    }

    pub async fn get_product_catalog_list(&self, catalog_id: &str) -> Result<GetProductCatalogListResponse> {
        get_product_catalog_list::get_product_catalog_list(self.api_client(), catalog_id).await
    }

    // --- batch 3 ---
    pub async fn get_qr(&self, user_ids: Vec<String>) -> Result<GetQrResponse> {
        get_qr::get_qr(self.api_client(), &user_ids).await
    }

    pub async fn get_quick_message_list(&self) -> Result<GetQuickMessageListResponse> {
        get_quick_message_list::get_quick_message_list(self.api_client()).await
    }

    pub async fn get_related_friend_group(&self, friend_ids: Vec<String>) -> Result<GetRelatedFriendGroupResponse> {
        get_related_friend_group::get_related_friend_group(self.api_client(), &friend_ids).await
    }

    pub async fn get_reminder(&self, reminder_id: &str) -> Result<GetReminderResponse> {
        get_reminder::get_reminder(self.api_client(), reminder_id).await
    }

    pub async fn get_reminder_responses(&self, reminder_id: &str) -> Result<GetReminderResponsesResponse> {
        get_reminder_responses::get_reminder_responses(self.api_client(), reminder_id).await
    }

    pub async fn get_sent_friend_request(&self) -> Result<GetSentFriendRequestResponse> {
        get_sent_friend_request::get_sent_friend_request(self.api_client()).await
    }

    pub async fn get_settings(&self) -> Result<GetSettingsResponse> {
        get_settings::get_settings(self.api_client()).await
    }

    pub async fn get_sticker_category_detail(&self, cate_id: i64) -> Result<GetStickerCategoryDetailResponse> {
        get_sticker_category_detail::get_sticker_category_detail(self.api_client(), cate_id).await
    }

    pub async fn get_stickers(&self, keyword: &str) -> Result<GetStickersResponse> {
        get_stickers::get_stickers(self.api_client(), keyword).await
    }

    pub async fn get_stickers_detail(&self, sticker_ids: Vec<i64>) -> Result<GetStickersDetailResponse> {
        get_stickers_detail::get_stickers_detail(self.api_client(), &sticker_ids).await
    }

    pub async fn get_unread_mark(&self) -> Result<GetUnreadMarkResponse> {
        get_unread_mark::get_unread_mark(self.api_client()).await
    }

    pub async fn invite_user_to_groups(&self, user_id: &str, group_ids: Vec<String>) -> Result<InviteUserToGroupsResponse> {
        invite_user_to_groups::invite_user_to_groups(self.api_client(), user_id, &group_ids).await
    }

    pub async fn join_group_invite_box(&self, group_id: &str) -> Result<JoinGroupInviteBoxResponse> {
        join_group_invite_box::join_group_invite_box(self.api_client(), group_id).await
    }

    pub async fn join_group_link(&self, link: &str) -> Result<JoinGroupLinkResponse> {
        join_group_link::join_group_link(self.api_client(), link).await
    }

    pub async fn last_online(&self, uid: &str) -> Result<LastOnlineResponse> {
        last_online::last_online(self.api_client(), uid).await
    }

    pub async fn leave_group(&self, group_id: &str, silent: Option<bool>) -> Result<LeaveGroupResponse> {
        leave_group::leave_group(self.api_client(), group_id, silent).await
    }

    pub async fn lock_poll(&self, poll_id: i64) -> Result<LockPollResponse> {
        lock_poll::lock_poll(self.api_client(), poll_id).await
    }

    pub async fn lost_focus(&self) -> Result<LostFocusResponse> {
        lost_focus::lost_focus(self.api_client()).await
    }

    pub async fn parse_link(&self, link: &str) -> Result<ParseLinkResponse> {
        parse_link::parse_link(self.api_client(), link).await
    }

    pub async fn register_catalog(&self, name: &str) -> Result<RegisterCatalogResponse> {
        register_catalog::register_catalog(self.api_client(), name).await
    }

    pub async fn reject_friend_request(&self, friend_id: &str) -> Result<RejectFriendRequestResponse> {
        reject_friend_request::reject_friend_request(self.api_client(), friend_id).await
    }

    pub async fn remove_friend(&self, friend_id: &str) -> Result<RemoveFriendResponse> {
        remove_friend::remove_friend(self.api_client(), friend_id).await
    }

    pub async fn remove_friend_alias(&self, friend_id: &str) -> Result<RemoveFriendAliasResponse> {
        remove_friend_alias::remove_friend_alias(self.api_client(), friend_id).await
    }

    pub async fn remove_group_blocked_member(&self, member_id: &str, group_id: &str) -> Result<RemoveGroupBlockedMemberResponse> {
        remove_group_blocked_member::remove_group_blocked_member(self.api_client(), member_id, group_id).await
    }

    pub async fn remove_group_deputy(&self, member_id: &str, group_id: &str) -> Result<RemoveGroupDeputyResponse> {
        remove_group_deputy::remove_group_deputy(self.api_client(), member_id, group_id).await
    }

    pub async fn remove_quick_message(&self, item_ids: Vec<i64>) -> Result<RemoveQuickMessageResponse> {
        remove_quick_message::remove_quick_message(self.api_client(), &item_ids).await
    }

    pub async fn remove_reminder(&self, topic_id: &str, thread_id: &str, thread_type: ThreadType) -> Result<RemoveReminderResponse> {
        remove_reminder::remove_reminder(self.api_client(), topic_id, thread_id, thread_type).await
    }

    pub async fn remove_unread_mark(&self, thread_id: &str, thread_type: ThreadType) -> Result<RemoveUnreadMarkResponse> {
        remove_unread_mark::remove_unread_mark(self.api_client(), thread_id, thread_type).await
    }

    pub async fn remove_user_from_group(&self, member_ids: Vec<String>, group_id: &str) -> Result<RemoveUserFromGroupResponse> {
        remove_user_from_group::remove_user_from_group(self.api_client(), &member_ids, group_id).await
    }

    pub async fn reset_hidden_convers_pin(&self) -> Result<ResetHiddenConversPinResponse> {
        reset_hidden_convers_pin::reset_hidden_convers_pin(self.api_client()).await
    }

    pub async fn reuse_avatar(&self, photo_id: &str, is_hd: Option<bool>) -> Result<ReuseAvatarResponse> {
        reuse_avatar::reuse_avatar(self.api_client(), photo_id, is_hd).await
    }

    pub async fn review_pending_member_request(&self, payload: (review_pending_member_request::ReviewPendingMemberRequestType, Vec<String>, String)) -> Result<ReviewPendingMemberRequestResponse> {
        let (req_type, members, group_id) = payload;
        review_pending_member_request::review_pending_member_request(self.api_client(), (req_type, &members, &group_id)).await
    }

    pub async fn scan_url(&self, url: &str) -> Result<ScanUrlResponse> {
        scan_url::scan_url(self.api_client(), url).await
    }

    pub async fn search_sticker(&self, keyword: &str, limit: Option<i32>) -> Result<SearchStickerResponse> {
        search_sticker::search_sticker(self.api_client(), keyword, limit).await
    }

    pub async fn send_bank_card(&self, payload: SendBankCardPayload, thread_id: &str, thread_type: ThreadType) -> Result<SendBankCardResponse> {
        send_bank_card::send_bank_card(self.api_client(), payload, thread_id, thread_type).await
    }

    pub async fn send_card(&self, options: SendCardOptions, thread_id: &str, thread_type: ThreadType) -> Result<SendCardResponse> {
        send_card::send_card(self.api_client(), options, thread_id, thread_type).await
    }

    pub async fn send_delivered_event(&self, is_seen: bool, messages: Vec<DeliveredMessage>, thread_type: ThreadType) -> Result<SendDeliveredEventResponse> {
        send_delivered_event::send_delivered_event(self.api_client(), is_seen, &messages, thread_type).await
    }

    pub async fn send_friend_request(&self, msg: &str, user_id: &str) -> Result<SendFriendRequestResponse> {
        send_friend_request::send_friend_request(self.api_client(), msg, user_id).await
    }

    pub async fn send_link(&self, options: SendLinkOptions, thread_id: &str, thread_type: ThreadType) -> Result<SendLinkResponse> {
        send_link::send_link(self.api_client(), options, thread_id, thread_type).await
    }

    pub async fn send_report(&self, options: ReportOptions, thread_id: &str, thread_type: ThreadType) -> Result<SendReportResponse> {
        send_report::send_report(self.api_client(), options, thread_id, thread_type).await
    }


    // --- batch 4 ---
    pub async fn forward_message(
        &self,
        payload: ForwardMessagePayload,
        thread_ids: &[String],
        thread_type: ThreadType,
    ) -> Result<ForwardMessageResponse> {
        forward_message::forward_message(self.api_client(), payload, thread_ids, thread_type).await
    }

    pub async fn send_seen_event(
        &self,
        messages: &[SendSeenEventMessageParams],
        thread_type: ThreadType,
    ) -> Result<SendSeenEventResponse> {
        send_seen_event::send_seen_event(self.api_client(), messages, thread_type).await
    }

    pub async fn set_hidden_conversations(
        &self,
        hidden: bool,
        thread_ids: &[String],
        thread_type: ThreadType,
    ) -> Result<SetHiddenConversationsResponse> {
        set_hidden_conversations::set_hidden_conversations(self.api_client(), hidden, thread_ids, thread_type).await
    }

    pub async fn set_mute(
        &self,
        duration: MuteDuration,
        action: MuteAction,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<SetMuteResponse> {
        set_mute::set_mute(self.api_client(), duration, action, thread_id, thread_type).await
    }

    pub async fn set_pinned_conversations(
        &self,
        pinned: bool,
        thread_ids: &[String],
        thread_type: ThreadType,
    ) -> Result<SetPinnedConversationsResponse> {
        set_pinned_conversations::set_pinned_conversations(self.api_client(), pinned, thread_ids, thread_type).await
    }

    pub async fn share_poll(&self, poll_id: i64) -> Result<SharePollResponse> {
        share_poll::share_poll(self.api_client(), poll_id).await
    }

    pub async fn unblock_user(&self, user_id: &str) -> Result<UnblockUserResponse> {
        unblock_user::unblock_user(self.api_client(), user_id).await
    }

    pub async fn undo(
        &self,
        payload: UndoPayload,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<UndoResponse> {
        undo::undo(self.api_client(), payload, thread_id, thread_type).await
    }

    pub async fn undo_friend_request(&self, friend_id: &str) -> Result<UndoFriendRequestResponse> {
        undo_friend_request::undo_friend_request(self.api_client(), friend_id).await
    }

    pub async fn update_active_status(&self, active: bool) -> Result<UpdateActiveStatusResponse> {
        update_active_status::update_active_status(self.api_client(), active).await
    }

    pub async fn update_archived_chat_list(
        &self,
        is_archived: bool,
        conversations: &[UpdateArchivedChatListTarget],
    ) -> Result<UpdateArchivedChatListResponse> {
        update_archived_chat_list::update_archived_chat_list(self.api_client(), is_archived, conversations).await
    }

    pub async fn update_auto_delete_chat(
        &self,
        ttl: ChatTTL,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<UpdateAutoDeleteChatResponse> {
        update_auto_delete_chat::update_auto_delete_chat(self.api_client(), ttl, thread_id, thread_type).await
    }

    pub async fn update_auto_reply(&self, payload: UpdateAutoReplyPayload) -> Result<UpdateAutoReplyResponse> {
        update_auto_reply::update_auto_reply(self.api_client(), payload).await
    }

    pub async fn update_bank_account(&self, payload: UpdateBankAccountPayload) -> Result<UpdateBankAccountResponse> {
        update_bank_account::update_bank_account(self.api_client(), payload).await
    }

    pub async fn update_catalog(&self, payload: UpdateCatalogPayload) -> Result<UpdateCatalogResponse> {
        update_catalog::update_catalog(self.api_client(), payload).await
    }

    pub async fn update_group_settings(
        &self,
        options: UpdateGroupSettingsOptions,
        group_id: &str,
    ) -> Result<UpdateGroupSettingsResponse> {
        update_group_settings::update_group_settings(self.api_client(), options, group_id).await
    }

    // --- batch 5 ---
    pub fn create_listener(&self) -> Listener {
        listen::create_listener(self.api_client(), self.ws_urls.clone())
    }

    pub async fn listen(&self, options: Option<ListenOptions>) -> Result<ListenResponse> {
        listen::listen(self.api_client(), options).await
    }

    pub async fn send_sticker(
        &self,
        sticker: SendStickerPayload,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<SendStickerResponse> {
        send_sticker::send_sticker(self.api_client(), sticker, thread_id, thread_type).await
    }

    pub async fn send_typing_event(
        &self,
        thread_id: &str,
        thread_type: ThreadType,
        dest_type: Option<DestType>,
    ) -> Result<SendTypingEventResponse> {
        send_typing_event::send_typing_event(self.api_client(), thread_id, thread_type, dest_type).await
    }

    pub async fn send_video(
        &self,
        options: SendVideoOptions,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<SendVideoResponse> {
        send_video::send_video(self.api_client(), options, thread_id, thread_type).await
    }

    pub async fn send_voice(
        &self,
        options: SendVoiceOptions,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<SendVoiceResponse> {
        send_voice::send_voice(self.api_client(), options, thread_id, thread_type).await
    }

    pub async fn update_hidden_convers_pin(&self, pin: &str) -> Result<UpdateHiddenConversPinResponse> {
        update_hidden_convers_pin::update_hidden_convers_pin(self.api_client(), pin).await
    }

    pub async fn update_labels(&self, payload: UpdateLabelsPayload) -> Result<UpdateLabelsResponse> {
        update_labels::update_labels(self.api_client(), payload).await
    }

    pub async fn update_lang(
        &self,
        language: Option<UpdateLangAvailableLanguages>,
    ) -> Result<UpdateLangResponse> {
        update_lang::update_lang(self.api_client(), language).await
    }

    pub async fn update_product_catalog(
        &self,
        payload: UpdateProductCatalogPayload,
    ) -> Result<UpdateProductCatalogResponse> {
        update_product_catalog::update_product_catalog(self.api_client(), payload).await
    }

    pub async fn update_profile(&self, payload: UpdateProfilePayload) -> Result<UpdateProfileResponse> {
        update_profile::update_profile(self.api_client(), payload).await
    }

    pub async fn update_profile_bio(&self, status: &str) -> Result<UpdateProfileBioResponse> {
        update_profile_bio::update_profile_bio(self.api_client(), status).await
    }

    pub async fn update_quick_message(
        &self,
        payload: UpdateQuickMessagePayload,
        item_id: i64,
    ) -> Result<UpdateQuickMessageResponse> {
        update_quick_message::update_quick_message(self.api_client(), payload, item_id).await
    }

    pub async fn update_settings(
        &self,
        setting_type: UpdateSettingsType,
        value: i32,
    ) -> Result<UpdateSettingsResponse> {
        update_settings::update_settings(self.api_client(), setting_type, value).await
    }

    pub async fn upgrade_group_to_community(
        &self,
        group_id: &str,
    ) -> Result<UpgradeGroupToCommunityResponse> {
        upgrade_group_to_community::upgrade_group_to_community(self.api_client(), group_id).await
    }

    pub async fn upload_attachment(
        &self,
        sources: Vec<AttachmentSource>,
        thread_id: &str,
        thread_type: ThreadType,
    ) -> Result<UploadAttachmentResponse> {
        upload_attachment::upload_attachment(self.api_client(), sources, thread_id, thread_type).await
    }

    pub async fn upload_product_photo(
        &self,
        payload: UploadProductPhotoPayload,
    ) -> Result<UploadProductPhotoResponse> {
        upload_product_photo::upload_product_photo(self.api_client(), payload).await
    }

    pub async fn vote_poll(&self, poll_id: i64, option_ids: Vec<i64>) -> Result<VotePollResponse> {
        vote_poll::vote_poll(self.api_client(), poll_id, option_ids).await
    }
}
