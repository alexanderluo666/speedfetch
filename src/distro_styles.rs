#[derive(Clone, Copy)]
pub struct DistroStyle {
    pub start: (u8, u8, u8),
    pub mid: (u8, u8, u8),
    pub end: (u8, u8, u8),
    pub speed: f32,
    pub blur: f32,
}

impl DistroStyle {
    pub const fn new(
        start: (u8, u8, u8),
        mid: (u8, u8, u8),
        end: (u8, u8, u8),
        speed: f32,
        blur: f32,
    ) -> Self {
        Self {
            start,
            mid,
            end,
            speed,
            blur,
        }
    }
}

const FEDORA: DistroStyle = DistroStyle::new(
    (0, 90, 220),
    (0, 170, 255),
    (0, 255, 240),
    0.012,
    0.07,
);
const UBUNTU: DistroStyle = DistroStyle::new(
    (255, 130, 0),
    (255, 200, 40),
    (255, 40, 90),
    0.015,
    0.08,
);
const DEBIAN: DistroStyle = DistroStyle::new(
    (200, 0, 50),
    (255, 60, 120),
    (255, 0, 200),
    0.008,
    0.06,
);
const ARCH: DistroStyle = DistroStyle::new(
    (0, 220, 200),
    (80, 200, 255),
    (20, 90, 255),
    0.02,
    0.09,
);
const OPENSUSE: DistroStyle = DistroStyle::new(
    (100, 200, 50),
    (50, 220, 160),
    (0, 160, 220),
    0.011,
    0.07,
);
const ALPINE: DistroStyle = DistroStyle::new(
    (0, 120, 220),
    (140, 200, 255),
    (240, 250, 255),
    0.01,
    0.05,
);
const NIXOS: DistroStyle = DistroStyle::new(
    (80, 180, 255),
    (100, 220, 255),
    (0, 255, 220),
    0.013,
    0.075,
);
const GENTOO: DistroStyle = DistroStyle::new(
    (140, 60, 200),
    (200, 100, 255),
    (255, 140, 220),
    0.009,
    0.065,
);
const VOID: DistroStyle = DistroStyle::new(
    (80, 200, 120),
    (140, 220, 160),
    (200, 210, 220),
    0.007,
    0.055,
);
const SOLUS: DistroStyle = DistroStyle::new(
    (0, 180, 160),
    (80, 220, 140),
    (180, 255, 100),
    0.014,
    0.07,
);
const MINT: DistroStyle = DistroStyle::new(
    (40, 180, 80),
    (100, 220, 120),
    (200, 255, 160),
    0.012,
    0.07,
);
const POP: DistroStyle = DistroStyle::new(
    (180, 90, 255),
    (220, 140, 255),
    (255, 160, 80),
    0.016,
    0.085,
);
const ELEMENTARY: DistroStyle = DistroStyle::new(
    (100, 160, 255),
    (160, 200, 255),
    (240, 245, 255),
    0.01,
    0.06,
);
const DEEPIN: DistroStyle = DistroStyle::new(
    (0, 140, 255),
    (80, 200, 255),
    (200, 240, 255),
    0.011,
    0.065,
);
const KALI: DistroStyle = DistroStyle::new(
    (0, 160, 255),
    (120, 100, 255),
    (200, 0, 255),
    0.018,
    0.09,
);
const STEAMOS: DistroStyle = DistroStyle::new(
    (40, 120, 200),
    (100, 160, 220),
    (180, 200, 230),
    0.008,
    0.06,
);
const CENTOS: DistroStyle = DistroStyle::new(
    (160, 0, 40),
    (220, 60, 80),
    (255, 180, 100),
    0.009,
    0.06,
);
const AMAZON: DistroStyle = DistroStyle::new(
    (255, 140, 0),
    (255, 200, 80),
    (40, 40, 50),
    0.01,
    0.055,
);
const ORACLE: DistroStyle = DistroStyle::new(
    (200, 0, 0),
    (255, 100, 60),
    (80, 40, 40),
    0.009,
    0.055,
);
const SLACKWARE: DistroStyle = DistroStyle::new(
    (0, 80, 180),
    (100, 140, 200),
    (200, 210, 220),
    0.006,
    0.05,
);
const MAGEIA: DistroStyle = DistroStyle::new(
    (0, 160, 100),
    (80, 200, 140),
    (160, 240, 200),
    0.01,
    0.06,
);
const CLEAR: DistroStyle = DistroStyle::new(
    (0, 200, 255),
    (120, 230, 255),
    (255, 255, 255),
    0.014,
    0.075,
);
const TAILS: DistroStyle = DistroStyle::new(
    (120, 60, 200),
    (180, 100, 255),
    (100, 180, 255),
    0.007,
    0.05,
);
const QUBES: DistroStyle = DistroStyle::new(
    (0, 140, 220),
    (100, 180, 255),
    (220, 240, 255),
    0.01,
    0.06,
);
const MX: DistroStyle = DistroStyle::new(
    (0, 140, 200),
    (80, 180, 220),
    (200, 220, 240),
    0.01,
    0.06,
);
const FREEBSD: DistroStyle = DistroStyle::new(
    (180, 0, 50),
    (220, 80, 100),
    (255, 200, 80),
    0.01,
    0.06,
);
const UNKNOWN: DistroStyle = DistroStyle::new(
    (90, 90, 100),
    (150, 150, 165),
    (220, 220, 230),
    0.005,
    0.04,
);

fn norm(id: &str) -> String {
    id.to_ascii_lowercase().replace('_', "-")
}

/// Config key for ASCII logo when no exact distro entry exists.
pub fn logo_family(distro: &str) -> &'static str {
    match norm(distro).as_str() {
        "arch" | "manjaro" | "manjaro-arm" | "endeavouros" | "garuda" | "cachyos" | "archcraft"
        | "artix" | "arcolinux" | "archarm" | "archlinuxarm" | "archlabs" | "rebornos"
        | "bluestar" | "namib" | "parch" | "parchlinux" | "archbang" | "archmerge"
        | "arco" | "xeroarch" | "instantos" => "arch",

        "fedora" | "nobara" | "ultramarine" | "berry" | "fedora-asahi" | "asahi"
        | "rocky" | "rockylinux" | "alma" | "almalinux" | "centos" | "centos-stream"
        | "rhel" | "redhat" | "oracle" | "ol" | "amzn" | "amazon" | "azurelinux"
        | "cloudlinux" | "scientific" | "scientificlinux" | "eurolinux" | "virtuozzo"
        | "openmandriva" | "openmandrivalinux" | "mageia" | "rosa" | "pclinuxos"
        | "alt" | "altlinux" | "astra" | "astralinux" | "clearos" | "springdale"
        | "miraclelinux" | "kinoite" | "silverblue" | "sericea"
        | "bluefin" | "bazzite" | "aurora" | "coreos" | "fcos" | "rhcos" => "fedora",

        "debian" | "devuan" | "pureos" | "parrot" | "parrotsec"
        | "antix" | "mx" | "mx-linux" | "mxlinux" | "sparky" | "sparkylinux"
        | "peppermint" | "peppermintos" | "knoppix" | "deepin" | "deepinos" | "uos"
        | "uniontech" | "tanglu" | "kanotix" | "bunsenlabs" | "crunchbang++"
        | "crunchbangplusplus" | "elive" | "siduction" | "solydxk" | "makulu"
        | "excalibur" | "pear" | "pearos" | "droidian" | "graphene" | "tails" => "debian",

        "ubuntu" | "linuxmint" | "mint" | "pop" | "pop-os" | "popos" | "elementary"
        | "elementaryos" | "zorin" | "zorinos" | "neon" | "kde-neon" | "kdeneon"
        | "kubuntu" | "lubuntu" | "xubuntu" | "ubuntu-mate" | "ubuntumate"
        | "ubuntu-budgie" | "ubuntubudgie" | "ubuntu-studio" | "ubuntustudio"
        | "ubuntu-kylin" | "ubuntu-unity" | "ubuntuunity" | "edubuntu" | "trisquel"
        | "bodhi" | "bodhi-linux" | "lite" | "linuxlite" | "linux-lite"
        | "feren" | "ferenos" | "backbox" | "backboxlinux" | "ubuntucinnamon"
        | "cinnamon" | "vanilla" | "vanillaos" | "blendos" | "blend-os" | "regolith"
        | "ubports" | "linspire" => "ubuntu",

        "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" | "opensuse-tumbleweed-kde"
        | "opensuse-leap-kde" | "suse" | "sles" | "sled" | "geckolinux" | "agama" => "opensuse",

        "alpine" | "postmarketos" | "pmos" | "alpine-chroot" => "alpine",
        "nixos" | "nix" | "snowflake" | "snowflakeos" => "nixos",
        "gentoo" | "funtoo" | "calculate" | "calculate-linux" | "redcore" | "redcorelinux"
        | "chimera" | "chimeraos" => "gentoo",
        "void" | "voidlinux" => "void",
        "solus" | "soluslinux" => "solus",
        "steamos" | "holo" | "steamdeck" | "bazzite-deck" => "steamos",
        "kali" | "kali-linux" => "kali",
        "slackware" | "slackwarearm" => "slackware",
        "clear" | "clear-linux" | "clearlinux" => "clear",
        "qubes" | "qubesos" => "qubes",
        "freebsd" | "ghostbsd" | "midnightbsd" | "nomadbsd" | "dragonfly"
        | "dragonflybsd" | "netbsd" | "openbsd" => "freebsd",
        "raspbian" | "raspberrypi" | "raspios" | "raspberrypios" => "raspbian",
        "android" | "lineage" | "lineageos" => "android",
        "chromeos" | "chromiumos" | "chrome-os" => "chromeos",
        "bedrock" => "bedrock",
        "guix" | "guixsd" => "guix",
        "hyperbola" => "hyperbola",
        "kiss" => "kiss",

        _ => "unknown",
    }
}

fn style_for_family(family: &str) -> DistroStyle {
    match family {
        "fedora" => FEDORA,
        "ubuntu" => UBUNTU,
        "debian" => DEBIAN,
        "arch" => ARCH,
        "opensuse" => OPENSUSE,
        "alpine" => ALPINE,
        "nixos" => NIXOS,
        "gentoo" => GENTOO,
        "void" => VOID,
        "solus" => SOLUS,
        "kali" => KALI,
        "steamos" => STEAMOS,
        "slackware" => SLACKWARE,
        "mageia" => MAGEIA,
        "clear" => CLEAR,
        "qubes" => QUBES,
        "freebsd" => FREEBSD,
        "raspbian" => DistroStyle::new(
            (200, 0, 60),
            (255, 80, 120),
            (255, 180, 200),
            0.01,
            0.06,
        ),
        "android" => DistroStyle::new(
            (60, 200, 100),
            (140, 230, 140),
            (200, 255, 200),
            0.012,
            0.07,
        ),
        "chromeos" => DistroStyle::new(
            (60, 140, 255),
            (140, 180, 255),
            (255, 200, 80),
            0.01,
            0.06,
        ),
        "bedrock" => DistroStyle::new(
            (180, 140, 80),
            (220, 180, 120),
            (255, 220, 160),
            0.008,
            0.055,
        ),
        "guix" => DistroStyle::new(
            (80, 180, 255),
            (140, 220, 255),
            (255, 200, 80),
            0.009,
            0.06,
        ),
        "hyperbola" => DistroStyle::new(
            (40, 40, 50),
            (100, 100, 110),
            (200, 200, 210),
            0.005,
            0.04,
        ),
        "kiss" => DistroStyle::new(
            (255, 200, 0),
            (255, 230, 100),
            (255, 255, 200),
            0.007,
            0.05,
        ),
        _ => UNKNOWN,
    }
}

/// Animated gradient preset; per-distro overrides, then family, then unknown.
pub fn distro_style(distro: &str) -> DistroStyle {
    let d = norm(distro);
    match d.as_str() {
        "linuxmint" | "mint" => MINT,
        "pop" | "pop-os" | "popos" => POP,
        "elementary" | "elementaryos" => ELEMENTARY,
        "deepin" | "deepinos" | "uos" | "uniontech" => DEEPIN,
        "zorin" | "zorinos" => DistroStyle::new(
            (0, 160, 120),
            (80, 200, 180),
            (160, 240, 220),
            0.011,
            0.065,
        ),
        "neon" | "kde-neon" | "kdeneon" => DistroStyle::new(
            (100, 180, 255),
            (160, 200, 255),
            (200, 120, 255),
            0.014,
            0.075,
        ),
        "rocky" | "rockylinux" | "almalinux" | "alma" => DistroStyle::new(
            (0, 100, 200),
            (0, 180, 240),
            (100, 220, 255),
            0.011,
            0.065,
        ),
        "centos" | "centos-stream" | "rhel" | "redhat" => CENTOS,
        "oracle" | "ol" => ORACLE,
        "amzn" | "amazon" | "azurelinux" => AMAZON,
        "parrot" | "parrotsec" => DistroStyle::new(
            (0, 200, 120),
            (80, 220, 160),
            (160, 80, 255),
            0.016,
            0.085,
        ),
        "endeavouros" => DistroStyle::new(
            (120, 80, 200),
            (160, 120, 240),
            (80, 200, 255),
            0.018,
            0.085,
        ),
        "garuda" => DistroStyle::new(
            (200, 60, 120),
            (255, 120, 180),
            (255, 180, 80),
            0.017,
            0.09,
        ),
        "nobara" | "ultramarine" => DistroStyle::new(
            (255, 100, 40),
            (255, 160, 80),
            (0, 180, 255),
            0.013,
            0.075,
        ),
        "cachyos" => DistroStyle::new(
            (0, 220, 200),
            (100, 200, 255),
            (255, 160, 80),
            0.019,
            0.09,
        ),
        "artix" | "arcolinux" => DistroStyle::new(
            (0, 200, 180),
            (80, 180, 220),
            (40, 90, 200),
            0.015,
            0.08,
        ),
        "mx" | "mx-linux" | "mxlinux" => MX,
        "tails" => TAILS,
        "antix" => DistroStyle::new(
            (80, 80, 90),
            (140, 140, 160),
            (220, 220, 230),
            0.006,
            0.045,
        ),
        "bazzite" | "bluefin" | "kinoite" | "silverblue" | "sericea" => DistroStyle::new(
            (0, 120, 220),
            (80, 200, 255),
            (0, 255, 200),
            0.012,
            0.07,
        ),
        "manjaro" | "manjaro-arm" => DistroStyle::new(
            (40, 200, 120),
            (80, 220, 160),
            (20, 160, 255),
            0.014,
            0.08,
        ),
        _ => style_for_family(logo_family(&d)),
    }
}
