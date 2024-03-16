mod user;
mod channel;
mod message;

trait Model {
    type Patch;
    type Insert;
    type Vector;

    fn to_patch(&self) -> Self::Patch;
    fn to_insert(&self) -> Self::Insert;
}

