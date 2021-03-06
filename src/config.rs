use git2::{Repository, Signature};

pub struct Config {
    pub user: String,
    pub repository_name: String,

    pub branch: String,

    pub repository_path: String,

    pub write_mode: bool,
    pub release_mode: bool,

    pub repository: Repository,
    pub signature: Signature<'static>,

    pub gh_token: Option<String>,
    pub cargo_token: Option<String>,
}

pub struct ConfigBuilder {
    user: Option<String>,
    repository_name: Option<String>,

    branch: Option<String>,

    repository_path: Option<String>,

    write_mode: bool,
    release_mode: bool,

    repository: Option<Repository>,
    signature: Option<Signature<'static>>,

    gh_token: Option<String>,
    cargo_token: Option<String>,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            user: None,
            repository_name: None,
            branch: None,
            repository_path: None,
            write_mode: false,
            release_mode: false,
            repository: None,
            signature: None,
            gh_token: None,
            cargo_token: None,
        }
    }

    pub fn user(&mut self, user: String) -> &mut Self {
        self.user = Some(user);
        self
    }

    pub fn repository_name(&mut self, name: String) -> &mut Self {
        self.repository_name = Some(name);
        self
    }

    pub fn branch(&mut self, branch: String) -> &mut Self {
        self.branch = Some(branch);
        self
    }

    pub fn repository_path(&mut self, path: String) -> &mut Self {
        self.repository_path = Some(path);
        self
    }

    pub fn repository(&mut self, repository: Repository) -> &mut Self {
        self.repository = Some(repository);
        self
    }

    pub fn write(&mut self, mode: bool) -> &mut Self {
        self.write_mode = mode;
        self
    }

    pub fn release(&mut self, mode: bool) -> &mut Self {
        self.release_mode = mode;
        self
    }

    pub fn signature(&mut self, sig: Signature<'static>) -> &mut Self {
        self.signature = Some(sig);
        self
    }

    pub fn gh_token(&mut self, token: String) -> &mut Self {
        self.gh_token = Some(token);
        self
    }

    pub fn cargo_token(&mut self, token: String) -> &mut Self {
        self.cargo_token = Some(token);
        self
    }

    pub fn build(self) -> Config {
        Config {
            user: self.user.unwrap_or("".into()),
            repository_name: self.repository_name.unwrap_or("".into()),
            branch: self.branch.unwrap_or("master".into()),
            repository_path: self.repository_path.unwrap(),
            write_mode: self.write_mode,
            release_mode: self.release_mode,
            repository: self.repository.unwrap(),
            signature: self.signature.unwrap(),
            gh_token: self.gh_token,
            cargo_token: self.cargo_token,
        }
    }
}
