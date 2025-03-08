use std::borrow::Cow;
use std::cell::Cell;

use deno_core::cppgc::GarbageCollected;
use deno_core::op2;
use deno_core::Resource;

#[derive(Debug)]
pub struct Rect {
    top: Cell<u32>,
    left: Cell<u32>,
    width: Cell<u32>,
    height: Cell<u32>,
}

impl GarbageCollected for Rect {}

#[op2]
impl Rect {
    #[constructor]
    #[cppgc]
    pub fn constructor(top: u32, left: u32, width: u32, height: u32) -> Rect {
        Rect {
            top: Cell::new(top),
            left: Cell::new(left),
            width: Cell::new(width),
            height: Cell::new(height),
        }
    }

    #[fast]
    #[getter]
    pub fn top(&self) -> u32 {
        self.top.get()
    }

    #[fast]
    #[getter]
    pub fn left(&self) -> u32 {
        self.left.get()
    }

    #[fast]
    #[getter]
    pub fn width(&self) -> u32 {
        self.width.get()
    }

    #[fast]
    #[getter]
    pub fn height(&self) -> u32 {
        self.height.get()
    }

    #[fast]
    #[setter]
    pub fn top(&self, top: u32) {
        self.top.set(top);
    }

    #[fast]
    #[setter]
    pub fn left(&self, left: u32) {
        self.left.set(left);
    }

    #[fast]
    #[setter]
    pub fn width(&self, width: u32) {
        self.width.set(width);
    }

    #[fast]
    #[setter]
    pub fn height(&self, height: u32) {
        self.height.set(height);
    }
}

impl Resource for Rect {
    fn name(&self) -> Cow<str> {
        Cow::Borrowed("rect")
    }
}

/* #[op2(fast)]
#[smi]
fn op_create_rect(
    state: &mut OpState,
    top: u32,
    left: u32,
    width: u32,
    height: u32,
) -> Result<ResourceId, JsErrorBox> {
    let rect = Rect::new(top, left, width, height);
    let rid = state.resource_table.add(rect);
    Ok(rid)
} */
