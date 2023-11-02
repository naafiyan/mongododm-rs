pub trait SafeDeleteable {
    fn cascade_delete(&self); // TODO: add error handling/checking e.g. Result return type to
                              // ensure db operations work
}

pub trait Schemable {
    fn collection_name(&self) -> &'static str;
}

pub fn safe_delete<T: SafeDeleteable>(to_delete : &T) -> Result<(), Box<dyn std::error::Error>> {
    to_delete.cascade_delete();
    std::result::Result::Ok(()) // TODO: remove placeholder return
}
