use futures::StreamExt;

/// Basic implementations.
pub mod impls;
mod queue;
pub(crate) use queue::*;
mod util;

/// Snapshot tag is a tag that bound to some snapshot resource.
/// If the resource is a file the tag is the path to the file, for example.
#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotTag {
    pub contents: bytes::Bytes,
}
impl AsRef<[u8]> for SnapshotTag {
    fn as_ref(&self) -> &[u8] {
        self.contents.as_ref()
    }
}
impl From<Vec<u8>> for SnapshotTag {
    fn from(x: Vec<u8>) -> SnapshotTag {
        SnapshotTag { contents: x.into() }
    }
}

use crate::proto_compiled::GetSnapshotRep;
use bytes::Bytes;
use futures::stream::Stream;
/// The stream type that is used internally. it is considered as just a stream of bytes.
/// The length of each bytes may vary.
pub type SnapshotStream =
    std::pin::Pin<Box<dyn futures::stream::Stream<Item = anyhow::Result<Bytes>> + Send>>;
pub(crate) type SnapshotStreamOut = std::pin::Pin<
    Box<dyn futures::stream::Stream<Item = Result<GetSnapshotRep, tonic::Status>> + Send + Sync>,
>;
pub(crate) fn into_out_stream(in_stream: SnapshotStream) -> SnapshotStreamOut {
    let out_stream = in_stream.map(|res| {
        res.map(|x| GetSnapshotRep { chunk: x.to_vec() })
            .map_err(|_| tonic::Status::unknown("streaming error"))
    });
    Box::pin(crate::SyncStream::new(out_stream))
}
pub(crate) fn into_in_stream(
    out_stream: impl Stream<Item = Result<GetSnapshotRep, tonic::Status>>,
) -> impl Stream<Item = anyhow::Result<Bytes>> {
    out_stream.map(|res| {
        res.map(|x| x.chunk.into())
            .map_err(|_| anyhow::Error::msg("streaming error"))
    })
}
