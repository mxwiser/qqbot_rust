use serde::Deserialize;
#[derive(Deserialize,Clone, Debug)]
#[allow(dead_code)]
pub struct MessageEvent {
    #[serde(default)]
    pub id: Option<String>,
    pub op: Option<i32>,
    #[serde(default)]
    pub d: Option<Data>,
    #[serde(default)]
    pub t: Option<String>,
}
#[derive(Deserialize,Clone, Debug)]
#[allow(dead_code)]
pub struct Data {
    #[serde(default)]
    pub author: Option<Author>,
    #[serde(default)]
    pub plain_token: Option<String>,
    #[serde(default)]
    pub event_ts: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub group_openid: Option<String>,
}

#[derive(Deserialize, Debug,Clone)]
#[allow(dead_code)]
pub struct Author {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub union_openid: Option<String>,
    #[serde(default)]
    pub user_openid: Option<String>,
    #[serde(default)]
    pub group_openid: Option<String>,
    #[serde(default)]
    pub member_openid: Option<String>,
}
