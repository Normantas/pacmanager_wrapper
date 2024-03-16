/// An error which could be returned while doing an action
#[derive(thiserror::Error, Debug)]
pub enum PacManagerError {
    #[error("the action is unimplemented by the package manager")]
    UnimplementedAction,
    #[error("the supplied package manager is unsupported")]
    UnsupportedPacManager,
    #[error("the package manager returned an error while executing the command")]
    InternalPacManagerError(String),
}
