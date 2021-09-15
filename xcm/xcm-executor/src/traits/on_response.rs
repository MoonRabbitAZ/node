

use xcm::v0::{Response, MultiLocation};
use frame_support::weights::Weight;

pub trait OnResponse {
	fn expecting_response(origin: &MultiLocation, query_id: u64) -> bool;
	fn on_response(origin: MultiLocation, query_id: u64, response: Response) -> Weight;
}
impl OnResponse for () {
	fn expecting_response(_origin: &MultiLocation, _query_id: u64) -> bool { false }
	fn on_response(_origin: MultiLocation, _query_id: u64, _response: Response) -> Weight { 0 }
}
