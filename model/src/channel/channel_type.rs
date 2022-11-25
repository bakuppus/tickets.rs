use serde_repr::{Deserialize_repr, Serialize_repr};
use ChannelType::*;

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
    GuildNewsThread = 10,
    GuildPublicThread = 11,
    GuildPrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
}

impl ChannelType {
    pub fn is_thread(&self) -> bool {
        match self {
            GuildNewsThread | GuildPublicThread | GuildPrivateThread => true,
            _ => false,
        }
    }
}
