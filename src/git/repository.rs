use std::fs::DirBuilder;
use std::fs::File;
use std::path::PathBuf;

use std;
use std::io::Write;

pub struct Repository {
    worktree: PathBuf,
    git_dir: PathBuf,
}

impl Repository {
    fn new(worktree: &str) -> Repository {
        let wp = PathBuf::from(worktree);
        let gp: PathBuf = [worktree, ".git"].iter().collect();

        Repository {
            worktree: wp,
            git_dir: gp,
        }
    }

    fn write(&self) -> std::io::Result<()> {
        let builder = DirBuilder::new();
        let git_dir_path = self.git_dir.as_path();

        builder.create(git_dir_path);
        builder.create(git_dir_path.join("refs"));
        builder.create(git_dir_path.join("objects"));

        let mut head_file = File::create(git_dir_path.join("HEAD"))?;
        head_file.write_all(b"ref: refs/heads/master")?;
        Ok(())
    }

    pub fn create(path: &str) {
        Repository::new(path).write();
    }
}
