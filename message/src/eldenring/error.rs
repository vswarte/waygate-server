use thiserror::Error;

pub trait NetError: std::error::Error {
    fn to_wire_code(&self) -> u32;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ServerError {
    #[error("OK")]
    Ok = 0,
    #[error("Unknown")]
    Unknown = 1,
    #[error("Invalid")]
    Invalid = 2,
    #[error("Out of memory")]
    OutOfMemory = 3,
    #[error("Invalid argument")]
    Args = 4,
    #[error("External library error")]
    ExternalLibraryError = 5,
    #[error("Not implemented")]
    NotImplemented = 6,
    #[error("Database error")]
    Db = 7,
    #[error("DB connection not found")]
    DbConnectionNotFound = 8,
    #[error("DB data not found")]
    DbDataNotFound = 9,
    #[error("No update rows")]
    NoUpdateRows = 10,
    #[error("Already in use")]
    AlreadyInUse = 11,
    #[error("Invalid application version")]
    InvalidAppVersion = 12,
    #[error("Invalid protocol")]
    InvalidProtocol = 13,
    #[error("Invalid sandbox")]
    InvalidSandbox = 14,
    #[error("Service stopped")]
    ServiceStopped = 15,
    #[error("Account banned")]
    AccountBanned = 16,
    #[error("Not found")]
    NotFound = 17,
    #[error("Prior reservation")]
    PriorReservation = 18,
    #[error("Ticket authentication error")]
    TicketAuthError = 19,
    #[error("Anti-cheat banned")]
    AntiCheatBanned = 20,
    #[error("Not initialized")]
    NotInitialized = 21,
    #[error("Cannot connect to Redis")]
    RedisConnectionError = 22,

    // Platform / session error codes
    #[error("EOS SDK error")]
    EosSdkError = 91,
    #[error("Steam ticket error")]
    SteamTicketError = 92,
    #[error("Cannot start game with guest profile")]
    GuestProfile = 100,
    #[error("No profile selected")]
    ProfileNotSelected = 101,
    #[error("Communication error")]
    CommunicationError = 200,
    #[error("Signed out")]
    SignedOut = 300,
    #[error("Offline")]
    Offline = 301,
    #[error("Age restriction")]
    AgeRestricted = 400,
    #[error("System update required")]
    SystemUpdateRequired = 401,
    #[error("Game update required")]
    GameUpdateRequired = 402,
    #[error("Sign-in required")]
    SignInRequired = 403,
    #[error("Signed out during session")]
    SessionSignedOut = 600,
    #[error("Authorization code failed")]
    AuthCodeFailed = 601,
    #[error("Unknown platform error (710)")]
    Unk710 = 710,
    #[error("Unknown platform error (711)")]
    Unk711 = 711,
    #[error("Unknown platform error (750)")]
    Unk750 = 750,

    // Matching server errors
    #[error("Matching server timeout")]
    MatchingServerTimeout = 900,
    #[error("Matching server request error")]
    MatchingServerRequestError = 901,
    #[error("Cannot connect to matching server")]
    MatchingServerConnectionError = 902,
    #[error("Matching server response is empty")]
    MatchingServerEmptyResponse = 903,
    #[error("Matching server response is invalid")]
    MatchingServerInvalidResponse = 904,
    #[error("Matching server response mismatch")]
    MatchingServerResponseMismatch = 905,

    // Memcached errors
    #[error("Memcached error")]
    MemcachedError = 1000,
    #[error("Cannot connect to memcached")]
    MemcachedConnectionError = 1001,

    // Container errors
    #[error("Failed to add element to container")]
    ContainerAddFailed = 1100,

    // Multiplayer area errors
    #[error("Boss area ID not found")]
    BossAreaIdNotFound = 1200,
    #[error("Multiplay area container not found")]
    MultiplayAreaContainerNotFound = 1201,

    // Blood message errors
    #[error("Failed to register blood message search index")]
    BloodMessageIndexFailed = 1300,
    #[error("Blood message data not found")]
    BloodMessageNotFound = 1301,
    #[error("Blood message already deleted")]
    BloodMessageAlreadyDeleted = 1302,

    // Phantom errors
    #[error("Phantom ID already registered")]
    PhantomAlreadyRegistered = 1400,
    #[error("Dead phantom data not found")]
    DeadPhantomNotFound = 1500,

    // Client-side error codes
    #[error("Client timeout")]
    TimeoutC = 10000,
    #[error("Not signed in")]
    NotSignedIn = 10001,
    #[error("Request aborted")]
    RequestAborted = 10002,
    #[error("Flatbuffers parse error")]
    FlatbuffersParseError = 10003,
    #[error("Invalid response")]
    InvalidResponse = 10004,
    #[error("Invalid client library state")]
    InvalidClientState = 10005,
    #[error("Login cancelled")]
    LoginCancelled = 10006,
    #[error("Server connection failed")]
    ServerConnectionFailed = 10007,
    #[error("Failed to get platform ticket")]
    PlatformTicketFailed = 10008,
    #[error("Failed to send request")]
    SendRequestFailed = 10009,
    #[error("Reconnecting")]
    Reconnecting = 10010,

    // Server-side error codes
    #[error("Login required")]
    LoginRequired = 20000,
    #[error("Rate limit exceeded")]
    RateLimitExceeded = 20001,
    #[error("Server timeout")]
    TimeoutS = 20002,
    #[error("Memcached conflict")]
    MemcachedConflict = 20003,
    #[error("Matching server index replication failed")]
    MatchingServerReplicationFailed = 20004,
    #[error("Redirect required")]
    RedirectRequired = 20005,
    #[error("Session overwritten by other session")]
    SessionOverwrittenByOther = 20006,
    #[error("Session invalidated by other")]
    SessionInvalidatedByOther = 20007,
}

impl NetError for ServerError {
    fn to_wire_code(&self) -> u32 {
        *self as u32
    }
}
