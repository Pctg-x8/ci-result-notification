use lambda_runtime::{handler_fn, Context, Error};

mod formatter;
use self::formatter::*;
mod character;
use self::character::*;

#[derive(serde::Deserialize)]
pub struct CIEngineCommitInfo {
    pub committer: String,
    pub message: String,
    pub sha: String,
}
#[derive(serde::Deserialize)]
pub struct CIEngineInput {
    #[serde(default)]
    pub weekly: bool,
    pub status: String,
    pub failure_step: Option<String>,
    pub build_url: String,
    pub compare_url: Option<String>,
    pub commit_hash: Option<String>,
    pub number: u32,
    pub duration: u64,
    pub repository: String,
    pub branch_name: Option<String>,
    pub commit: CIEngineCommitInfo,
    #[serde(rename = "ref")]
    pub head_ref: Option<String>,
    pub pr_number: Option<u32>,
    pub pr_name: Option<String>,
    pub report_name: String,
    pub support_info: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ExecutionResult {
    code: u32,
    body: Option<String>,
}
impl ExecutionResult {
    pub const OK: Self = Self {
        code: 200,
        body: None,
    };
}

#[derive(serde::Serialize)]
pub struct SlackAttachmentField<'s> {
    title: &'s str,
    value: &'s str,
    short: Option<bool>,
}
impl<'s> SlackAttachmentField<'s> {
    pub fn new(name: &'s str, value: &'s str) -> Self {
        SlackAttachmentField {
            title: name,
            value,
            short: None,
        }
    }
    pub fn short(mut self, short: bool) -> Self {
        self.short = Some(short);
        self
    }
}
#[derive(serde::Serialize)]
pub struct SlackAttachment<'s> {
    color: Option<&'s str>,
    text: Option<std::borrow::Cow<'s, str>>,
    title: Option<&'s str>,
    fields: Vec<SlackAttachmentField<'s>>,
}
impl Default for SlackAttachment<'_> {
    fn default() -> Self {
        Self {
            color: None,
            text: None,
            title: None,
            fields: vec![],
        }
    }
}
#[derive(serde::Serialize)]
pub struct SlackPostData<'s> {
    pub channel: &'static str,
    pub text: &'s str,
    pub as_user: bool,
    pub icon_emoji: &'static str,
    pub username: &'s str,
    pub attachments: &'s [SlackAttachment<'s>],
}
#[derive(serde::Deserialize)]
pub struct SlackPostResult {
    pub ok: bool,
    pub error: Option<String>,
}

#[derive(serde::Serialize)]
pub struct DiscordEmbedFooterObject<'s> {
    pub text: &'s str,
    pub icon_url: Option<&'s str>,
    pub proxy_icon_url: Option<&'s str>,
}
impl<'s> DiscordEmbedFooterObject<'s> {
    pub fn new(text: &'s str) -> Self {
        DiscordEmbedFooterObject {
            text,
            icon_url: None,
            proxy_icon_url: None,
        }
    }
    pub fn icon_url(mut self, url: &'s str) -> Self {
        self.icon_url = Some(url);
        self
    }
    pub fn proxy_icon_url(mut self, url: &'s str) -> Self {
        self.proxy_icon_url = Some(url);
        self
    }
}
#[derive(serde::Serialize)]
pub struct DiscordEmbedAuthorObject<'s> {
    pub name: Option<&'s str>,
    pub url: Option<&'s str>,
    pub icon_url: Option<&'s str>,
    pub proxy_icon_url: Option<&'s str>,
}
impl Default for DiscordEmbedAuthorObject<'_> {
    fn default() -> Self {
        DiscordEmbedAuthorObject {
            name: None,
            url: None,
            icon_url: None,
            proxy_icon_url: None,
        }
    }
}
#[derive(serde::Serialize)]
pub struct DiscordEmbedFieldObject<'s> {
    pub name: &'s str,
    pub value: &'s str,
    pub inline: Option<bool>,
}
impl<'s> DiscordEmbedFieldObject<'s> {
    pub fn new(name: &'s str, value: &'s str) -> Self {
        DiscordEmbedFieldObject {
            name,
            value,
            inline: None,
        }
    }
    pub fn inline(mut self, can_inline: bool) -> Self {
        self.inline = Some(can_inline);
        self
    }
}
#[derive(serde::Serialize)]
pub struct DiscordEmbedObject<'s> {
    pub title: Option<&'s str>,
    #[serde(rename = "type")]
    pub type_: Option<&'s str>,
    pub description: Option<&'s str>,
    pub url: Option<&'s str>,
    pub color: Option<u32>,
    pub footer: Option<DiscordEmbedFooterObject<'s>>,
    pub author: Option<DiscordEmbedAuthorObject<'s>>,
    pub fields: Vec<DiscordEmbedFieldObject<'s>>,
}
impl Default for DiscordEmbedObject<'_> {
    fn default() -> Self {
        DiscordEmbedObject {
            title: None,
            type_: None,
            description: None,
            url: None,
            color: None,
            footer: None,
            author: None,
            fields: vec![],
        }
    }
}
#[derive(serde::Serialize)]
pub struct DiscordExecuteWebhookPayload<'s> {
    pub content: &'s str,
    pub username: Option<&'s str>,
    pub avatar_url: Option<&'s str>,
    pub embeds: Vec<DiscordEmbedObject<'s>>,
}
impl<'s> DiscordExecuteWebhookPayload<'s> {
    pub fn with_content(content: &'s str) -> Self {
        DiscordExecuteWebhookPayload {
            content,
            username: None,
            avatar_url: None,
            embeds: vec![],
        }
    }
    pub fn override_user(mut self, name: &'s str, avatar_url: &'s str) -> Self {
        self.username = Some(name);
        self.avatar_url = Some(avatar_url);
        self
    }
    pub fn embeds(mut self, embeds: Vec<DiscordEmbedObject<'s>>) -> Self {
        self.embeds = embeds;
        self
    }
}
#[derive(serde::Deserialize)]
pub struct DiscordResponse {
    pub message: Option<String>,
}

const DISCORD_EMBED_COLOR_RED: u32 = 15158332;
const DISCORD_EMBED_COLOR_GREEN: u32 = 3066993;

const SLACK_AUTH_HEADER: &'static str = concat!("Bearer ", include_str!("../.secrets/slack_token"));
const DISCORD_WEBHOOK_URL: &'static str = include_str!("../.secrets/discord_webhook_url");
include!("../.secrets/avatar_url_map.rs");
include!("../.secrets/character_name.rs");

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    lambda_runtime::run(handler_fn(handler)).await
}
async fn handler(e: CIEngineInput, _context: Context) -> Result<ExecutionResult, Error> {
    Ok(report(e, Character::new()).await)
}

async fn report<C: ReportCharacter>(e: CIEngineInput, character: C) -> ExecutionResult {
    let check_title = &e.report_name;
    let succeeded = e.status == "success";

    let repo_name = e
        .repository
        .split('/')
        .nth(1)
        .expect("invalid repository format");
    let build_url_text = NumericLinkTextFormatter {
        number: e.number,
        repo_name,
    };
    let (message1, message1_d, color, face, face_ident, state);
    if succeeded {
        message1 = character
            .construct_success_message(&SlackLinkFormatter::new(&build_url_text, &e.build_url));
        message1_d = character
            .construct_success_message(&MarkdownLinkFormatter::new(&build_url_text, &e.build_url));
        color = &ColorCode::GOOD;
        let (f, fi) = character.success_face_icon();
        face = f;
        face_ident = fi;
        state = "Passed";
    } else {
        message1 = character
            .construct_failure_message(&SlackLinkFormatter::new(&build_url_text, &e.build_url));
        message1_d = character
            .construct_failure_message(&MarkdownLinkFormatter::new(&build_url_text, &e.build_url));
        color = &ColorCode::DANGER;
        let (f, fi) = character.failure_face_icon();
        face = f;
        face_ident = fi;
        state = "Failed";
    }

    // Build commit info line
    let commitinfo = if let Some(pr_number) = e.pr_number {
        // diff/pr mode
        CommitInfoFormatter::DiffPR {
            repository: &e.repository,
            pr_number,
            pr_name: e.pr_name.as_ref().expect("missing pr_name"),
            compare_url: e.compare_url.as_ref().expect("missing compare_url"),
            short_sha: &e.commit.sha[..8],
        }
    } else {
        // branch mode
        CommitInfoFormatter::Branch {
            repository: &e.repository,
            sha: &e.commit.sha,
            branch_name: e.branch_name.as_ref().expect("missing branch_name"),
            short_sha: &e.commit.sha[..8],
        }
    };
    let commitinfo_full = format!(
        "{}（コミッターさん: {}）「{}」",
        CommitInfoWithLinkFormatter::<SlackLinkFormatter<String, String>, SlackPREmoji>::from(
            &commitinfo
        ),
        e.commit.committer,
        e.commit.message
    );
    let commitinfo_full_d = format!(
        "{}（コミッターさん: {}）「{}」",
        CommitInfoWithLinkFormatter::<MarkdownLinkFormatter<String, String>, DiscordPREmoji>::from(
            &commitinfo
        ),
        e.commit.committer,
        e.commit.message
    );

    let text = format!(
        "{}（かかった時間: {}）",
        message1,
        DurationFormatter(e.duration)
    );
    let text_d = format!(
        "{}（かかった時間: {}）",
        message1_d,
        DurationFormatter(e.duration)
    );
    let username = format!("{}{}（{}: {}）", C::NAME, face_ident, check_title, state);
    let discord_avatar_url = avatar_url_map(face);
    let attachments = vec![SlackAttachment {
        color: Some(color.slack),
        text: e.support_info.as_deref().map(std::borrow::Cow::Borrowed),
        fields: match e.failure_step {
            Some(ref fs) => vec![
                SlackAttachmentField::new("失敗したジョブ", fs),
                SlackAttachmentField::new("コミット情報", &commitinfo_full),
            ],
            None => vec![SlackAttachmentField::new("コミット情報", &commitinfo_full)],
        },
        ..Default::default()
    }];
    let embeds = vec![DiscordEmbedObject {
        color: Some(color.discord),
        description: e.support_info.as_deref(),
        fields: match e.failure_step {
            Some(ref fs) => vec![
                DiscordEmbedFieldObject::new("失敗したジョブ", fs),
                DiscordEmbedFieldObject::new("コミット情報", &commitinfo_full_d),
            ],
            None => vec![DiscordEmbedFieldObject::new(
                "コミット情報",
                &commitinfo_full_d,
            )],
        },
        ..Default::default()
    }];

    let postdata = DiscordExecuteWebhookPayload::with_content(&text_d)
        .override_user(&username, discord_avatar_url)
        .embeds(embeds);
    let discord_post_task = execute_discord_webhook(postdata);

    let postdata = SlackPostData {
        channel: "#ci-notifications",
        text: &text,
        as_user: false,
        icon_emoji: face,
        username: &username,
        attachments: &attachments,
    };
    let slack_post_task = post_slack_message(postdata);

    let res = tokio::try_join!(discord_post_task, slack_post_task);
    match res {
        Ok(_) => ExecutionResult::OK,
        Err(e) => {
            log::error!("Post failed: {}", e);
            e.into()
        }
    }
}

pub struct ColorCode {
    slack: &'static str,
    discord: u32,
}
impl ColorCode {
    pub const GOOD: Self = Self {
        slack: "good",
        discord: DISCORD_EMBED_COLOR_GREEN,
    };
    pub const DANGER: Self = Self {
        slack: "danger",
        discord: DISCORD_EMBED_COLOR_RED,
    };
}

enum PostError {
    SlackRequesting(reqwest::Error),
    FromSlack(String),
    DiscordRequesting(reqwest::Error),
}
impl std::fmt::Display for PostError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SlackRequesting(e) => write!(fmt, "Slack requesting failed: {}", e),
            Self::DiscordRequesting(e) => write!(fmt, "Discord requesting failed: {}", e),
            Self::FromSlack(e) => write!(fmt, "Slack responded an error: {}", e),
        }
    }
}
impl From<PostError> for ExecutionResult {
    fn from(v: PostError) -> Self {
        match v {
            PostError::SlackRequesting(e) => ExecutionResult {
                code: e.status().expect("no error?").as_u16() as _,
                body: Some(e.to_string()),
            },
            PostError::DiscordRequesting(e) => ExecutionResult {
                code: e.status().expect("no error?").as_u16() as _,
                body: Some(e.to_string()),
            },
            PostError::FromSlack(e) => ExecutionResult {
                code: 500,
                body: Some(e),
            },
        }
    }
}

async fn post_slack_message<'a>(msg: SlackPostData<'a>) -> Result<(), PostError> {
    let mut headers = reqwest::header::HeaderMap::with_capacity(2);
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_static(SLACK_AUTH_HEADER),
    );

    let r: SlackPostResult = reqwest::Client::new()
        .post("https://api.slack.com/api/chat.postMessage")
        .headers(headers)
        .body(serde_json::to_string(&msg).expect("Failed to serialize slack post request"))
        .send()
        .await
        .map_err(PostError::SlackRequesting)?
        .json()
        .await
        .map_err(PostError::SlackRequesting)?;

    if !r.ok {
        Err(PostError::FromSlack(r.error.unwrap_or_else(String::new)))
    } else {
        Ok(())
    }
}

async fn execute_discord_webhook<'a>(
    payload: DiscordExecuteWebhookPayload<'a>,
) -> Result<(), PostError> {
    let r = reqwest::Client::new()
        .post(DISCORD_WEBHOOK_URL)
        .header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        )
        .body(serde_json::to_string(&payload).expect("Failed to serialize discord webhook payload"))
        .send()
        .await
        .map_err(PostError::DiscordRequesting)?
        .text()
        .await
        .map_err(PostError::DiscordRequesting)?;

    if !r.is_empty() {
        log::info!("Discord ExecuteWebhook Response: {}", r);
    }

    Ok(())
}
