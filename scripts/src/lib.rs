use std::error::Error;

use octocrab::models::repos::Ref;

pub fn ref_sha(reference: Ref) -> Result<String, Box<dyn Error>> {
    match reference.object {
        octocrab::models::repos::Object::Commit { sha, .. } => Ok(sha),
        octocrab::models::repos::Object::Tag { sha, .. } => Ok(sha),
        _ => Err("Unknown reference object.".into()),
    }
}
