

#[derive(Debug, Clone)]
pub struct AddUserError {
	error: String,
}
impl fmt::Display for AddUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.error)
    }
}
impl fmt::Debug for AddUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AddUserError {{ code: {}, message: {} }}\n {}",
            self.code, self.message, self.error,
        )
    }
}


#[derive(Debug, Clone)]
pub struct RemoveUserError;
impl fmt::Display for RemoveUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.error)
    }
}
impl fmt::Debug for RemoveUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RemoveUserError {{ code: {}, message: {} }}\n {}",
            self.code, self.message, self.error,
        )
    }
}
#[derive(Debug, Clone)]
pub struct FindUserError;
impl fmt::Display for FindUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.error)
    }
}
impl fmt::Debug for FindUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FindUserError {{ code: {}, message: {} }}\n {}",
            self.code, self.message, self.error,
        )
    }
}