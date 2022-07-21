use near_sdk::env;
use near_sdk::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(tag = "standard")]
#[must_use = "don't forget to `.emit()` this event"]
#[serde(rename_all = "snake_case")]
pub(crate) enum NearEvent<'a> {
    Nep141(crate::events::Nep141Event<'a>),
}

impl<'a> NearEvent<'a> {
    fn to_json_string(&self) -> String {
        #[allow(clippy::redundant_closure)]
        serde_json::to_string(self).ok().unwrap_or_else(|| env::abort())
    }

    fn to_json_event_string(&self) -> String {
        format!("EVENT_JSON:{}", self.to_json_string())
    }

    pub(crate) fn emit(self) {
        near_sdk::env::log_str(&self.to_json_event_string());
    }
}
