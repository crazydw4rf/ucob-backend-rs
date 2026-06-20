pub fn normalize_url(url: &str) -> Result<String, url::ParseError> {
  let a = url::Url::parse(url)?;
  Ok(a.to_string())
}
