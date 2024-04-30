use crate::api::{HttpClient, HttpClientExt};

use std::sync::OnceLock;

use eyre::Result;
use log::trace;
use poise::serenity_prelude::Message;
use regex::Regex;

const MCLOGS: &str = "https://api.mclo.gs/1";
const RAW: &str = "/raw";

pub struct MCLogs;

impl super::LogProvider for MCLogs {
	async fn find_match(&self, message: &Message) -> Option<String> {
		static REGEX: OnceLock<Regex> = OnceLock::new();
		let regex = REGEX.get_or_init(|| Regex::new(r"https://mclo\.gs/(\w+)").unwrap());

		trace!("Checking if message {} is an mclo.gs paste", message.id);
		super::get_first_capture(regex, &message.content)
	}

	async fn fetch(&self, http: &HttpClient, content: &str) -> Result<String> {
		let url = format!("{MCLOGS}{RAW}/{content}");
		let log = http.get_request(&url).await?.text().await?;

		Ok(log)
	}
}
