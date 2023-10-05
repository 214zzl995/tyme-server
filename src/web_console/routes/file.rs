use axum::{
    body::Bytes,
    extract::{BodyStream, Path},
    response::IntoResponse,
    BoxError, Json,
};
use futures::{Stream, TryStreamExt};
use serde_json::json;
use std::io;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;

const UPLOADS_DIRECTORY: &str = "ssl";

pub async fn upload_crt(Path(file_name): Path<String>, body: BodyStream) -> impl IntoResponse {
    match stream_to_file(&file_name, body).await {
        Ok(_) => Json(json!({"result": "ok", "message": "Push success"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn stream_to_file<S, E>(path: &str, stream: S) -> anyhow::Result<()>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err(anyhow::anyhow!(format!("Invalid path:{}", path)));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(UPLOADS_DIRECTORY).join(path);
        if path.exists() {
            tokio::fs::remove_file(&path).await.unwrap();
        }
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, anyhow::Error>(())
    }
    .await
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}
