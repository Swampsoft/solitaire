
use resources::Resources;
use table::Table;

pub struct Game<T> {
    pub resources: Resources,
    pub table: Table,
    pub state: T,
}
