use std::process::ExitCode;

fn sub() -> Result<(), String> {
    let i = std::io::stdin();
    let il = i.lock();
    let v: Vec<f32> = num2exr::source::read2vec(il, Some(0.0));

    let filename: String = std::env::var("ENV_EXR_NAME")
        .map_err(|e| format!("output exr image name ENV_EXR_NAME missing: {e}"))?;
    let width: String =
        std::env::var("ENV_WIDTH").map_err(|e| format!("width ENV_WIDTH missing: {e}"))?;
    let w: usize =
        str::parse(width.as_str()).map_err(|e| format!("invalid width string({width}): {e}"))?;
    let chname: String = std::env::var("ENV_CHANNEL_NAME")
        .ok()
        .unwrap_or_else(|| "Y".into());
    let alt: f32 = std::env::var("ENV_ALT_VALUE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or_default();
    num2exr::img::slice2path32f(&v, w, filename, chname.as_str(), alt)
        .map_err(|e| format!("unable to save as an exr image: {e}"))?;
    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
