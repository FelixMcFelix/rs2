pub enum PrivilegeLevel {
	Kernel(ExceptionLevel),
	Supervisor,
	User,
}

impl PrivilegeLevel {
	pub fn is_kernel(&self) -> bool {
		if let Self::Kernel(_) = self {
			true
		} else {
			false
		}
	}
}

pub enum ExceptionLevel {
	NoException,
	Level1,
	Level2,
}