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

	pub fn is_in_exception(&self) -> bool {
		use ExceptionLevel::*;

		match self {
			Self::Kernel(Level1) | Self::Kernel(Level2) => true,
			_ => false,
		}
	}
}

pub enum ExceptionLevel {
	NoException,
	Level1,
	Level2,
}