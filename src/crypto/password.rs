use crate::error::{ErrorKind, Result};

pub fn hash_password(password: &str) -> Result<String> {
  // NOTE: default cost nya 12 dari library dan makan waktu sekitar 500ms
  // berlaku untuk hashing sama verifikasi password
  // kalau cost 10 bisa sekitar 100-250ms
  const BCRYPT_COST: u32 = 10;

  let hashed = bcrypt::hash(password, BCRYPT_COST)?;

  Ok(hashed)
}

pub fn verify_password<P: AsRef<[u8]>>(password: P, hash: String) -> Result<()> {
  let is_valid = bcrypt::verify(password, hash.as_str())?;

  if !is_valid {
    return Err(("wrong password", ErrorKind::CredentialsInvalid).into());
  }

  Ok(())
}
