use std::collections::HashMap;
use crate::renderer::cell::{DirtyCells};
use crate::renderer::region_builder::{Region, RegionBuilder};

const DATA_ROWS: usize = 3;
const ENGINE_DEBUG_ROWS: usize = 1;

pub struct FrameBuilderParams {
    terminal_width: usize,
    terminal_height: usize,
}

impl FrameBuilderParams {
    pub fn new(terminal_width: usize, terminal_height: usize) -> FrameBuilderParams {
        FrameBuilderParams {
            terminal_width,
            terminal_height,
        }
    }
}

pub struct Layout {
    pub world: Region,
    pub data: Region,
    pub debug: Region,
}
pub struct FrameBuilder {
    dirty_cells: DirtyCells,
    layout: Layout,
}

impl FrameBuilder {
    pub fn init(params: FrameBuilderParams) -> Self {
        let world_height = params.terminal_height - DATA_ROWS - ENGINE_DEBUG_ROWS;

        let layout = Layout {
            world: Region {
                x: 0,
                y: 0,
                width: params.terminal_width,
                height: world_height,
            },
            data: Region {
                x: 0,
                y: world_height,
                width: params.terminal_width,
                height: DATA_ROWS,
            },
            debug: Region {
                x: 0,
                y: world_height + DATA_ROWS,
                width: params.terminal_width,
                height: ENGINE_DEBUG_ROWS,
            }
        };

        Self {
            dirty_cells: HashMap::new(),
            layout,
        }
    }

    pub fn world(&mut self) -> RegionBuilder<'_> {
        RegionBuilder::new(
            self.layout.world,
            &mut self.dirty_cells,
        )
    }

    pub fn data(&mut self) -> RegionBuilder<'_> {
        RegionBuilder::new(
            self.layout.data,
            &mut self.dirty_cells,
        )
    }

    pub fn debug(&mut self) -> RegionBuilder<'_> {
        RegionBuilder::new(
            self.layout.debug,
            &mut self.dirty_cells,
        )
    }

    #[inline]
    pub fn take_dirty(&mut self) -> DirtyCells {
        std::mem::take(&mut self.dirty_cells)
    }
}