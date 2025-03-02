use std::fmt;

use super::*;

#[derive(Clone)]
pub struct EnvVarsMapping {
    /// Size limit of the entity LFU cache.
    ///
    /// Set by the environment variable `GRAPH_ENTITY_CACHE_SIZE` (expressed in
    /// kilobytes). The default value is 10 megabytes.
    pub entity_cache_size: usize,
    /// Set by the environment variable `GRAPH_MAX_API_VERSION`. The default
    /// value is `0.0.6`.
    pub max_api_version: Version,
    /// Set by the environment variable `GRAPH_MAPPING_HANDLER_TIMEOUT`
    /// (expressed in seconds). No default is provided.
    pub timeout: Option<Duration>,
    /// Maximum stack size for the WASM runtime.
    ///
    /// Set by the environment variable `GRAPH_RUNTIME_MAX_STACK_SIZE`
    /// (expressed in bytes). The default value is 512KiB.
    pub max_stack_size: usize,
    /// How many blocks per network should be kept in the query cache. When the
    /// limit is reached, older blocks are evicted. This should be kept small
    /// since a lookup to the cache is O(n) on this value, and the cache memory
    /// usage also increases with larger number. Set to 0 to disable
    /// the cache.
    ///
    /// Set by the environment variable `GRAPH_QUERY_CACHE_BLOCKS`. The default
    /// value is 2.
    pub query_cache_blocks: usize,
    /// Maximum total memory to be used by the cache. Each block has a max size of
    /// `QUERY_CACHE_MAX_MEM` / (`QUERY_CACHE_BLOCKS` *
    /// `GRAPH_QUERY_BLOCK_CACHE_SHARDS`).
    ///
    /// Set by the environment variable `GRAPH_QUERY_CACHE_MAX_MEM` (expressed
    /// in MB). The default value is 1GB.
    pub query_cache_max_mem: usize,
    /// Set by the environment variable `GRAPH_QUERY_CACHE_STALE_PERIOD`. The
    /// default value is 100.
    pub query_cache_stale_period: u64,

    /// Set by the environment variable `GRAPH_MAX_IPFS_CACHE_FILE_SIZE`
    /// (expressed in bytes). The default value is 1MiB.
    pub max_ipfs_cache_file_size: usize,
    /// Set by the environment variable `GRAPH_MAX_IPFS_CACHE_SIZE`. The default
    /// value is 50 items.
    pub max_ipfs_cache_size: u64,
    /// The timeout for all IPFS requests.
    ///
    /// Set by the environment variable `GRAPH_IPFS_TIMEOUT` (expressed in
    /// seconds). The default value is 30s.
    pub ipfs_timeout: Duration,
    /// Sets the `ipfs.map` file size limit.
    ///
    /// Set by the environment variable `GRAPH_MAX_IPFS_MAP_FILE_SIZE_LIMIT`
    /// (expressed in bytes). The default value is 256MiB.
    pub max_ipfs_map_file_size: usize,
    /// Sets the `ipfs.cat` file size limit.
    ///
    /// Set by the environment variable `GRAPH_MAX_IPFS_FILE_BYTES` (expressed in
    /// bytes). No default value is provided.
    ///
    /// FIXME: Having an env variable here is a problem for consensus.
    /// Index Nodes should not disagree on whether the file should be read.
    pub max_ipfs_file_bytes: Option<usize>,
    /// Set by the flag `GRAPH_ALLOW_NON_DETERMINISTIC_IPFS`. Off by
    /// default.
    pub allow_non_deterministic_ipfs: bool,
}

// This does not print any values avoid accidentally leaking any sensitive env vars
impl fmt::Debug for EnvVarsMapping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "env vars")
    }
}

impl From<InnerMappingHandlers> for EnvVarsMapping {
    fn from(x: InnerMappingHandlers) -> Self {
        Self {
            entity_cache_size: x.entity_cache_size_in_kb * 1000,
            max_api_version: x.max_api_version,
            timeout: x.mapping_handler_timeout_in_secs.map(Duration::from_secs),
            max_stack_size: x.runtime_max_stack_size.0 .0,
            query_cache_blocks: x.query_cache_blocks,
            query_cache_max_mem: x.query_cache_max_mem_in_mb.0 * 1000 * 1000,
            query_cache_stale_period: x.query_cache_stale_period,

            max_ipfs_cache_file_size: x.max_ipfs_cache_file_size.0,
            max_ipfs_cache_size: x.max_ipfs_cache_size,
            ipfs_timeout: Duration::from_secs(x.ipfs_timeout_in_secs),
            max_ipfs_map_file_size: x.max_ipfs_map_file_size.0,
            max_ipfs_file_bytes: x.max_ipfs_file_bytes,
            allow_non_deterministic_ipfs: x.allow_non_deterministic_ipfs.0,
        }
    }
}

#[derive(Clone, Debug, Envconfig)]
pub struct InnerMappingHandlers {
    #[envconfig(from = "GRAPH_ENTITY_CACHE_SIZE", default = "10000")]
    entity_cache_size_in_kb: usize,
    #[envconfig(from = "GRAPH_MAX_API_VERSION", default = "0.0.7")]
    max_api_version: Version,
    #[envconfig(from = "GRAPH_MAPPING_HANDLER_TIMEOUT")]
    mapping_handler_timeout_in_secs: Option<u64>,
    #[envconfig(from = "GRAPH_RUNTIME_MAX_STACK_SIZE", default = "")]
    runtime_max_stack_size: WithDefaultUsize<NoUnderscores<usize>, { 512 * 1024 }>,
    #[envconfig(from = "GRAPH_QUERY_CACHE_BLOCKS", default = "2")]
    query_cache_blocks: usize,
    #[envconfig(from = "GRAPH_QUERY_CACHE_MAX_MEM", default = "1000")]
    query_cache_max_mem_in_mb: NoUnderscores<usize>,
    #[envconfig(from = "GRAPH_QUERY_CACHE_STALE_PERIOD", default = "100")]
    query_cache_stale_period: u64,

    // IPFS.
    #[envconfig(from = "GRAPH_MAX_IPFS_CACHE_FILE_SIZE", default = "")]
    max_ipfs_cache_file_size: WithDefaultUsize<usize, { 1024 * 1024 }>,
    #[envconfig(from = "GRAPH_MAX_IPFS_CACHE_SIZE", default = "50")]
    max_ipfs_cache_size: u64,
    #[envconfig(from = "GRAPH_IPFS_TIMEOUT", default = "30")]
    ipfs_timeout_in_secs: u64,
    #[envconfig(from = "GRAPH_MAX_IPFS_MAP_FILE_SIZE", default = "")]
    max_ipfs_map_file_size: WithDefaultUsize<usize, { 256 * 1024 * 1024 }>,
    #[envconfig(from = "GRAPH_MAX_IPFS_FILE_BYTES")]
    max_ipfs_file_bytes: Option<usize>,
    #[envconfig(from = "GRAPH_ALLOW_NON_DETERMINISTIC_IPFS", default = "false")]
    allow_non_deterministic_ipfs: EnvVarBoolean,
}
