#[cfg(any(feature = "mock", test))]
extern crate pseudo;
#[cfg(feature = "temp")]
extern crate rand;
#[cfg(feature = "temp")]
extern crate tempdir;

use std::io::Result;
use std::path::{Path, PathBuf};

#[cfg(any(feature = "mock", test))]
pub use mock::{FakeError, MockFileSystem};
#[cfg(feature = "fake")]
pub use fake::{FakeFileSystem, FakeTempDir};
pub use os::OsFileSystem;
#[cfg(feature = "temp")]
pub use os::OsTempDir;

#[cfg(feature = "fake")]
mod fake;
#[cfg(any(feature = "mock", test))]
mod mock;
mod os;

/// Provides standard file system operations.
pub trait FileSystem {
    /// Returns the current working directory.
    /// This is based on [`std::env::current_dir`].
    ///
    /// [`std::env::current_dir`]: https://doc.rust-lang.org/std/env/fn.current_dir.html
    fn current_dir(&self) -> Result<PathBuf>;
    /// Updates the current working directory.
    /// This is based on [`std::env::set_current_dir`].
    ///
    /// [`std::env::set_current_dir`]: https://doc.rust-lang.org/std/env/fn.set_current_dir.html
    fn set_current_dir<P: AsRef<Path>>(&self, path: P) -> Result<()>;

    /// Determines whether the path exists and points to a directory.
    fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool;
    /// Determines whether the path exists and points to a file.
    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool;

    /// Creates a new directory.
    /// This is based on [`std::fs::create_dir`].
    ///
    /// [`std::fs::create_dir`]: https://doc.rust-lang.org/std/fs/fn.create_dir.html
    fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    /// Recursively creates a directory and any missing parents.
    /// This is based on [`std::fs::create_dir`].
    ///
    /// [`std::fs::create_dir_all`]: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
    fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    /// Removes an empty directory.
    /// This is based on [`std::fs::remove_dir`].
    ///
    /// [`std::fs::remove_dir`]: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
    fn remove_dir<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    /// Removes a directory and any child files or directories.
    /// This is based on [`std::fs::remove_dir_all`].
    ///
    /// [`std::fs::remove_dir_all`]: https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
    fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()>;

    /// Writes `buf` to a new file at `path`.
    ///
    /// # Errors
    ///
    /// * A file or directory already exists at `path`.
    /// * The parent directory of `path` does not exist.
    /// * Current user has insufficient permissions.
    fn create_file<P, B>(&self, path: P, buf: B) -> Result<()>
        where P: AsRef<Path>,
              B: AsRef<[u8]>;
    /// Writes `buf` to a new or existing file at `buf`.
    /// This will overwrite any contents that already exist.
    ///
    /// # Errors
    ///
    /// * The parent directory of `path` does not exist.
    /// * Current user has insufficient permissions.
    fn write_file<P, B>(&self, path: P, buf: B) -> Result<()>
        where P: AsRef<Path>,
              B: AsRef<[u8]>;
    /// Returns the contents of `path`.
    ///
    /// * Errors
    ///
    /// * `path` does not exist.
    /// * `path` is a directory.
    /// * Current user has insufficient permissions.
    fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>>;

    /// Returns `true` if `path` is a readonly file.
    ///
    /// * Errors
    ///
    /// * `path` does not exist.
    /// * Current user has insufficient permissions.
    fn readonly<P: AsRef<Path>>(&self, path: P) -> Result<bool>;
    /// Sets or unsets the readonly flag of `path`.
    ///
    /// * Errors
    ///
    /// * `path` does not exist.
    /// * Current user has insufficient permissions.
    fn set_readonly<P: AsRef<Path>>(&self, path: P, readonly: bool) -> Result<()>;
}

#[cfg(unix)]
pub trait UnixFileSystem {
    /// Returns the current mode bits of `path`.
    ///
    /// * Errors
    ///
    /// * `path` does not exist.
    /// * Current user has insufficient permissions.
    fn mode<P: AsRef<Path>>(&self, path: P) -> Result<u32>;
    /// Sets the mode bits of `path`.
    ///
    /// * Errors
    ///
    /// * `path` does not exist.
    /// * Current user has insufficient permissions.
    fn set_mode<P: AsRef<Path>>(&self, path: P, mode: u32) -> Result<()>;
}

#[cfg(feature = "temp")]
/// Tracks a temporary directory that will be deleted once the struct goes out of scope.
pub trait TempDir {
    /// Returns the [`Path`] of the temporary directory.
    ///
    /// [`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
    fn path(&self) -> &Path;
}

#[cfg(feature = "temp")]
pub trait TempFileSystem {
    type TempDir: TempDir;

    /// Creates a new temporary directory.
    fn temp_dir<S: AsRef<str>>(&self, prefix: S) -> Result<Self::TempDir>;
}
