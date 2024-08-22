use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct PaginationParameters {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pagination: PaginatedResponseMeta,
    data: Vec<T>,
}

#[derive(Serialize)]
struct PaginatedResponseMeta {
    total: i64,
    count: usize,
}

impl<T> PaginatedResponse<T> {
    pub fn new(total: i64, data: Vec<T>) -> Self {
        let count = data.len();
        let pagination = PaginatedResponseMeta {
            total,
            count,
        };

        Self {
            pagination,
            data
        }
    }
}
