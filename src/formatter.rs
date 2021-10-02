use std::fmt::{Display, Formatter};

pub trait LinkFormatter<Text: Display, Url: Display> : Sized + Display {
    fn new(text: Text, url: Url) -> Self;
}

pub struct SlackLinkFormatter<Text: Display, Url: Display>(Text, Url);
pub struct MarkdownLinkFormatter<Text: Display, Url: Display>(Text, Url);
impl<Text: Display, Url: Display> LinkFormatter<Text, Url> for SlackLinkFormatter<Text, Url> {
    fn new(text: Text, url: Url) -> Self { Self(text, url) }
}
impl<Text: Display, Url: Display> LinkFormatter<Text, Url> for MarkdownLinkFormatter<Text, Url> {
    fn new(text: Text, url: Url) -> Self { Self(text, url) }
}
impl<Text: Display, Url: Display> Display for SlackLinkFormatter<Text, Url> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "<{}|{}>", self.1, self.0)
    }
}
impl<Text: Display, Url: Display> Display for MarkdownLinkFormatter<Text, Url> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "[{}]({})", self.0, self.1)
    }
}

pub struct NumericLinkTextFormatter<'s> { pub number: u32, pub repo_name: &'s str }
impl Display for NumericLinkTextFormatter<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "{}#{}", self.repo_name, self.number)
    }
}

pub struct SlackPREmoji;
impl Display for SlackPREmoji {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result { fmt.write_str(":pr:") }
}
impl Default for SlackPREmoji { fn default() -> Self { SlackPREmoji } }
pub struct DiscordPREmoji;
impl Display for DiscordPREmoji {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result { fmt.write_str("<:pr:777087690868654080>") }
}
impl Default for DiscordPREmoji { fn default() -> Self { DiscordPREmoji } }

pub struct DurationFormatter(pub u64);
impl Display for DurationFormatter {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let (min, sec) = (self.0 / 60, self.0 % 60);
        write!(fmt, "{}分{}秒", min, sec)
    }
}

pub enum CommitInfoFormatter<'s> {
    DiffPR {
        repository: &'s str, pr_number: u32, pr_name: &'s str, compare_url: &'s str, short_sha: &'s str
    },
    Branch {
        repository: &'s str, sha: &'s str, branch_name: &'s str, short_sha: &'s str
    }
}
pub struct CommitInfoWithLinkFormatter<'s, LF: LinkFormatter<String, String>, PREmoji: Display + Default>(
    &'s CommitInfoFormatter<'s>, std::marker::PhantomData<(LF, PREmoji)>
);
impl<'s, LF: LinkFormatter<String, String>, PREmoji: Display + Default> From<&'s CommitInfoFormatter<'s>> for CommitInfoWithLinkFormatter<'s, LF, PREmoji> {
    fn from(v: &'s CommitInfoFormatter<'s>) -> Self { Self(v, std::marker::PhantomData) }
}
impl<LF: LinkFormatter<String, String>, PREmoji: Display + Default> Display for CommitInfoWithLinkFormatter<'_, LF, PREmoji> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self.0 {
            CommitInfoFormatter::DiffPR { repository, pr_number, pr_name, compare_url, short_sha, .. } => write!(
                fmt, "{}の{}",
                LF::new(format!("{}#{}「{}」", PREmoji::default(), pr_number, pr_name), format!("https://github.com/{}/pull/{}", repository, pr_number)),
                LF::new(format!("コミット {}", short_sha), String::from(*compare_url))
            ),
            CommitInfoFormatter::Branch { repository, sha, branch_name, short_sha, .. } => LF::new(
                format!("ブランチ {} のコミット {}", branch_name, short_sha),
                format!("https://github.com/{}/tree/{}", repository, sha)
            ).fmt(fmt)
        }
    }
}
