pub enum Region {
    US,
    EU,
    JP,
    SH
}

impl ToString for Region {
    fn to_string(&self) -> String {
        match self {
            Region::EU => "eu".to_string(),
            Region::US => "us".to_string(),
            Region::JP => "jp".to_string(),
            Region::SH => "sh".to_string(),
        }
    }
}

pub struct Rom {
    pub path: String,
    pub region: Region,
}

impl Rom {
    pub fn default() -> Rom {
        Rom {
            path: String::new(),
            region: Region::US,
        }
    }
}

pub struct Repo {
    pub name: String,
    pub url: String,
    pub branch: String,
    pub supports_packs: bool,
    pub supports_textures: bool
}

impl Repo {
    pub fn default() -> Repo {
        Repo {
            name: "dummy repo".to_string(),
            url: "https://github.com/ezntek/smbuilder".to_string(),
            branch: "main".to_string(),
            supports_packs: false,
            supports_textures: false
        }
    }
}

pub struct DynOSPack {
    pub path: String,
    pub label: String,
    pub enabled: bool,
}

// The Build Specification Structure. Contains all the metadata required to run the Smbuilder class and the SmbuilderBuilder class, etc.
//
// Supports:
//
// * Jobs (jobs = [make] -jX)
// * Name (A custom name can be used, else it is repo.name)
// * Additional Make Options (eg. FOO=1 BAR=baz QUUX=0, make  FOO=1 BAR=baz QUUX=0 -jX)
// * A custom texture pack (think Render96)
// * DynOS data packs (also think Render96, but other ports like sm64ex-coop supports them too)
//
pub struct BuildSpec {
    // The number of jobs to be put together with the MAKEOPTS during the compile stage.
    pub jobs: u8,
    // The name of the build, it will default to the name of the repo if left empty.
    pub name: String,
    // Any additional makeopts to be added to the make call. Will include the jobs.
    pub additional_makeopts: Vec<String>,
    // The executable path. Not playable if tempty, playable if not empty.
    pub executable_path: Option<String>,
    // A custom texture pack (There can only be one!)
    pub texture_pack_path: Option<String>,
    // Any DynOS packs the user wishes to add
    pub dynos_packs: Vec<DynOSPack>,
    // The repo struct
    pub repo: Repo,
    // The rom struct
    pub rom: Rom,
}