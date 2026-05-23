use honzo_core::HonzoError;

pub const ANNO_NAMESPACE: &str = "org.nisoku.anno";
pub const DRM_NAMESPACE: &str = "org.nisoku.drm";
pub const SYNC_NAMESPACE: &str = "org.nisoku.sync";

pub mod anno;
pub mod drm;
pub mod sync;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnownExtra {
    Anno(Vec<anno::Annotation>),
    Drm(drm::DrmEnvelope),
    Sync(Vec<sync::SyncCue>),
}

impl KnownExtra {
    pub fn namespace(&self) -> &'static str {
        match self {
            Self::Anno(_) => ANNO_NAMESPACE,
            Self::Drm(_) => DRM_NAMESPACE,
            Self::Sync(_) => SYNC_NAMESPACE,
        }
    }
}

pub fn is_known_namespace(namespace: &str) -> bool {
    matches!(namespace, ANNO_NAMESPACE | DRM_NAMESPACE | SYNC_NAMESPACE)
}

pub fn parse_known(namespace: &str, body: &[u8]) -> Option<Result<KnownExtra, HonzoError>> {
    match namespace {
        ANNO_NAMESPACE => Some(anno::parse_anno(body).map(KnownExtra::Anno)),
        DRM_NAMESPACE => Some(drm::parse_drm(body).map(KnownExtra::Drm)),
        SYNC_NAMESPACE => Some(sync::parse_sync(body).map(KnownExtra::Sync)),
        _ => None,
    }
}
