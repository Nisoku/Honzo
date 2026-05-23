use honzo_core::HonzoError;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

pub const ANNO_NAMESPACE: &str = "org.nisoku.anno";
pub const DRM_NAMESPACE: &str = "org.nisoku.drm";
pub const SYNC_NAMESPACE: &str = "org.nisoku.sync";

pub mod anno;
pub mod drm;
pub mod sync;

pub type RegisteredExtraValue = Box<dyn Any + Send + Sync>;
pub type RegisteredExtraParser =
    dyn Fn(&[u8]) -> Result<RegisteredExtraValue, HonzoError> + Send + Sync + 'static;

static EXTRA_REGISTRY: OnceLock<RwLock<HashMap<String, Arc<RegisteredExtraParser>>>> =
    OnceLock::new();

fn registry() -> &'static RwLock<HashMap<String, Arc<RegisteredExtraParser>>> {
    EXTRA_REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn boxed_extra<T: Any + Send + Sync>(value: T) -> RegisteredExtraValue {
    Box::new(value)
}

pub struct RegisteredExtra {
    namespace: String,
    value: RegisteredExtraValue,
}

impl core::fmt::Debug for RegisteredExtra {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RegisteredExtra")
            .field("namespace", &self.namespace)
            .field("value_type", &std::any::type_name_of_val(&*self.value))
            .finish()
    }
}

impl RegisteredExtra {
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.value.downcast_ref()
    }

    pub fn into_downcast<T: Any>(self) -> Result<Box<T>, Self> {
        match self.value.downcast::<T>() {
            Ok(value) => Ok(value),
            Err(value) => Err(Self {
                namespace: self.namespace,
                value,
            }),
        }
    }
}

#[derive(Debug)]
pub enum ParsedExtra {
    Known(KnownExtra),
    Registered(RegisteredExtra),
}

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
    if matches!(namespace, ANNO_NAMESPACE | DRM_NAMESPACE | SYNC_NAMESPACE) {
        return true;
    }
    registry()
        .read()
        .expect("extra registry poisoned")
        .contains_key(namespace)
}

pub fn parse_known(namespace: &str, body: &[u8]) -> Option<Result<KnownExtra, HonzoError>> {
    match namespace {
        ANNO_NAMESPACE => Some(anno::parse_anno(body).map(KnownExtra::Anno)),
        DRM_NAMESPACE => Some(drm::parse_drm(body).map(KnownExtra::Drm)),
        SYNC_NAMESPACE => Some(sync::parse_sync(body).map(KnownExtra::Sync)),
        _ => None,
    }
}

pub fn register_extra_parser(
    namespace: impl Into<String>,
    parser: impl Fn(&[u8]) -> Result<RegisteredExtraValue, HonzoError> + Send + Sync + 'static,
) -> Option<Arc<RegisteredExtraParser>> {
    registry()
        .write()
        .expect("extra registry poisoned")
        .insert(namespace.into(), Arc::new(parser))
}

pub fn parse_registered(
    namespace: &str,
    body: &[u8],
) -> Option<Result<RegisteredExtra, HonzoError>> {
    let parser = registry()
        .read()
        .expect("extra registry poisoned")
        .get(namespace)
        .cloned()?;

    Some(parser(body).map(|value| RegisteredExtra {
        namespace: namespace.to_string(),
        value,
    }))
}

pub fn parse_extra(namespace: &str, body: &[u8]) -> Option<Result<ParsedExtra, HonzoError>> {
    if let Some(parsed) = parse_known(namespace, body) {
        return Some(parsed.map(ParsedExtra::Known));
    }

    parse_registered(namespace, body).map(|parsed| parsed.map(ParsedExtra::Registered))
}
