mod internal;
use rand::prelude::SliceRandom;

pub use self::internal::*;

pub trait ReportCharacter {
    const NAME: &'static str;

    fn construct_success_message<LF: std::fmt::Display>(&self, build_url: &LF) -> String;
    fn construct_failure_message<LF: std::fmt::Display>(&self, build_url: &LF) -> String;
    /// icon emoji, identification string
    fn success_face_icon(&self) -> (&'static str, &'static str);
    /// icon emoji, identification string
    fn failure_face_icon(&self) -> (&'static str, &'static str);
}

pub struct ReportCharacterTestFace;
impl ReportCharacter for ReportCharacterTestFace {
    const NAME: &'static str = "テストちゃん";

    fn construct_success_message<LF: std::fmt::Display>(&self, build_url: &LF) -> String {
        format!("{} のビルドに成功したよ！", build_url)
    }
    fn construct_failure_message<LF: std::fmt::Display>(&self, build_url: &LF) -> String {
        format!("{} のビルドに失敗しちゃった＞＜", build_url)
    }
    fn success_face_icon(&self) -> (&'static str, &'static str) {
        [(":testface_normal:", ""), (":testface_smile:", "\u{200b}")]
            .choose(&mut rand::thread_rng())
            .copied()
            .unwrap()
    }
    fn failure_face_icon(&self) -> (&'static str, &'static str) {
        (":testface_fail:", "")
    }
}
