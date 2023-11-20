use noodles_bgzf as bgzf;
use noodles_core::Position;

use super::Index;
use crate::index::reference_sequence::bin::Chunk;

/// A linear index.
pub type LinearIndex = Vec<bgzf::VirtualPosition>;

// _Sequence Alignment/Map Format Specification_ (2023-05-24) § 5.1.3 "Combining with linear
// index": "...each tiling 16384bp window..."
const WINDOW_SIZE: usize = 1 << 14;

impl Index for LinearIndex {
    fn min_offset(&self, _: u8, _: u8, start: Position) -> bgzf::VirtualPosition {
        let i = (usize::from(start) - 1) / WINDOW_SIZE;
        self.get(i).copied().unwrap_or_default()
    }

    fn last_first_start_position(&self) -> Option<bgzf::VirtualPosition> {
        self.last().copied()
    }

    fn update(&mut self, _: u8, _: u8, start: Position, end: Position, chunk: Chunk) {
        let linear_index_start_offset = (usize::from(start) - 1) / WINDOW_SIZE;
        let linear_index_end_offset = (usize::from(end) - 1) / WINDOW_SIZE;

        if linear_index_end_offset >= self.len() {
            self.resize(
                linear_index_end_offset + 1,
                bgzf::VirtualPosition::default(),
            );
        }

        #[allow(clippy::needless_range_loop)]
        for i in linear_index_start_offset..=linear_index_end_offset {
            if self[i] == bgzf::VirtualPosition::default() {
                self[i] = chunk.start();
            }
        }
    }
}
