// Integration test for SigmaOS-video-codec-libraries
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use video_codec_libraries as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
