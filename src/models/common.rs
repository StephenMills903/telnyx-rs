use serde::{Deserialize, Serialize};

/// API response envelope for non-list endpoints
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiResponse<T> {
    /// Endpoint specific response object
    pub data: T,
}

/// API response envelope for list endpoints
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiListResponse<T> {
    /// Vector of endpoint specific response objects
    pub data: Vec<T>,
    /// Metadata for paginated responses
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

/// Metadata for paginated responses from list endpoints
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationMeta {
    /// The total number of pages available
    pub total_pages: i32,
    /// The totla number of results for the given filters
    pub total_results: i32,
    /// The current page number
    pub page_number: i32,
    /// The total size of the page
    pub page_size: i32,
}
