use crate::request::Request;

mod user_exists;
mod role_check;

pub use user_exists::UserExists;
pub use role_check::RoleCheck;

pub trait Middleware {
    fn execute(&mut self, request: &mut Request) -> bool {
        let success = self.handle(request);

        if let Some(next)  = &mut self.next() {
            if success {
                return next.execute(request);
            }
        }

        success
    }

    fn handle(&mut self, patient: &mut Request) -> bool;
    fn next(&mut self) -> &mut Option<Box<dyn Middleware>>;
}

pub(self) fn into_next(
    middleware: impl Middleware + Sized + 'static,
) -> Option<Box<dyn Middleware>> {
    Some(Box::new(middleware))
}
