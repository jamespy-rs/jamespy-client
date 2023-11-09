use serde_derive::{Deserialize, Serialize};
use serenity::all::{
    Channel, ChannelId, Guild, GuildChannel, GuildId, GuildMemberUpdateEvent, Member, Message,
    MessageId, MessageUpdateEvent, PartialGuildChannel, Reaction, User, VoiceState,
};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug)]
pub enum WebSocketEvent {
    NewMessage {
        message: Message,
        guild_name: String,
        channel_name: String,
    },
    MessageEdit {
        old_if_available: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent,
        guild_name: Option<String>,
        channel_name: Option<String>,
    },
    MessageDelete {
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
        message: Option<Message>,
        guild_name: String,
        channel_name: String,
    },
    ChannelCreate {
        channel: GuildChannel,
        guild_name: String,
    },
    ChannelUpdate {
        old: Option<Channel>,
        new: Channel,
        guild_name: String,
    },
    ChannelDelete {
        channel: GuildChannel,
        guild_name: String,
    },
    ThreadCreate {
        thread: GuildChannel,
        guild_name: String,
    },
    ThreadUpdate {
        old: Option<GuildChannel>,
        new: GuildChannel,
        parent_channel: Option<Channel>,
        guild_name: String,
    },
    ThreadDelete {
        thread: PartialGuildChannel,
        full_thread_data: Option<GuildChannel>,
        guild_name: String,
    },
    GuildCreate {
        guild: Guild,
        is_new: Option<bool>,
    },
    GuildMemberAddition {
        new_member: Member,
        guild_name: String,
    },
    GuildMemberRemoval {
        guild_id: GuildId,
        user: User,
        guild_name: String,
    },
    ReactionAdd {
        add_reaction: Reaction,
        user_name: String,
        guild_name: String,
        channel_name: String,
    },
    ReactionRemove {
        removed_reaction: Reaction,
        user_name: String,
        guild_name: String,
        channel_name: String,
    },
    GuildMemberUpdate {
        old_if_available: Option<Member>,
        new: Option<Member>,
        event: GuildMemberUpdateEvent,
        guild_name: String,
    },
    VoiceStateUpdate {
        old: Option<VoiceState>,
        new: VoiceState,
        old_guild_name: Option<String>,
        old_channel_name: Option<String>,
        new_guild_name: Option<String>,
        new_channel_name: Option<String>,
        user_name: Option<String>,
    },
}
