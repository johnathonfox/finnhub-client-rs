//! Calendar endpoints.

use std::sync::Arc;

use crate::client::ClientInner;
use crate::error::Result;
use crate::models::calendar::{EarningsCalendar, IpoCalendar};

/// Handle for the `calendar` endpoint group (`client.calendar()`).
#[derive(Clone)]
pub struct CalendarEndpoints {
    inner: Arc<ClientInner>,
}

impl CalendarEndpoints {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Historical and upcoming earnings releases between two dates,
    /// `YYYY-MM-DD` (`/calendar/earnings`).
    pub async fn earnings(&self, from: &str, to: &str) -> Result<EarningsCalendar> {
        self.inner
            .get("/calendar/earnings", &[("from", from), ("to", to)])
            .await
    }

    /// Recent and upcoming IPOs between two dates, `YYYY-MM-DD`
    /// (`/calendar/ipo`).
    pub async fn ipo(&self, from: &str, to: &str) -> Result<IpoCalendar> {
        self.inner
            .get("/calendar/ipo", &[("from", from), ("to", to)])
            .await
    }
}
