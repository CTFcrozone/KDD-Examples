use derive_more::{Display, From};
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum Error {
	#[from(String, &String, &str)]
	Custom(String),
	FailInit,
	DbDumpFail,
	NoSQLFiles,
	FileNameIncorrectFormat,
	#[display("File path '{_0}' not found.")]
	FilePathNotFound(String),
	#[display("Invalid path '{_0}'")]
	InvalidPath(String),
	MissingConfigMustHaveEndpointOrRegion,
	#[display("AWS SDK Error [{code}]: {message}")]
	AwsSdkErrorWrapper {
		code: String,
		message: String,
	},
	// -- Externals
	#[from]
	Io(std::io::Error),
	#[from]
	Sqlx(sqlx::Error),
	#[from]
	Regex(lazy_regex::regex::Error),
}

// region:    --- Custom

impl Error {
	pub fn custom_from_err(err: impl std::error::Error) -> Self {
		Self::Custom(err.to_string())
	}

	pub fn custom(val: impl Into<String>) -> Self {
		Self::Custom(val.into())
	}
}

// endregion: --- Custom

// region:    --- Error Boilerplate

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
