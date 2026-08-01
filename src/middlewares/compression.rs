use tower_http::compression::{CompressionLayer, predicate::SizeAbove};

pub fn get_compression() -> CompressionLayer<SizeAbove> {
    CompressionLayer::new().compress_when(SizeAbove::new(0))
}
