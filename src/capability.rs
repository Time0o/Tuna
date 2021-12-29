use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid capability '{}'", name))]
    Invalid {
        name: &'static str,
        source: capng::Error
    },
    #[snafu(display("Missing capability '{}'", name))]
    Missing {
        name: &'static str,
    },
}

type Result<T> = std::result::Result<T, Error>;

pub fn check(name: &'static str) -> Result<()> {
    let cap = capng::name_to_capability(name).context(Invalid { name })?;

    if capng::have_capability(capng::Type::EFFECTIVE, cap) {
        Ok(())
    } else {
        Err(Error::Missing { name })
    }
}
