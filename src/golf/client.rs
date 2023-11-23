#![allow(dead_code)]

use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use sqlx::SqlitePool;
use std::sync::Arc;

use super::BookingEvent;

const DATE_FROM: &str = "11-11-2023";
const DATE_TO: &str = "20-12-2023";

#[derive(Debug)]
pub struct GolfClient {
    http_client: Client,
    base_url: String,
    cookie_store: Arc<CookieStoreMutex>,
    pool: Arc<SqlitePool>,
}

impl GolfClient {
    /// Creates a new [`GolfClient`].
    pub(crate) fn new(base_url: &str, pool: &SqlitePool) -> Self {
        let cookie_store = Arc::new(CookieStoreMutex::default());
        let http_client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .build()
            .unwrap();

        let pool = Arc::new(pool.clone());

        Self {
            http_client,
            base_url: base_url.to_string(),
            cookie_store,
            pool,
        }
    }

    pub(crate) async fn get_event(&self, event_id: u32) -> anyhow::Result<BookingEvent> {
        let url = format!("{}/spring/bookings/events/{}", self.base_url, event_id);
        let res = self.http_client.get(&url).send().await?.text().await?;
        let event: BookingEvent = quick_xml::de::from_str(&res)?;
        Ok(event)
    }
}

mod login {
    use super::*;

    const LOGIN_POST_PATH: &str = "security/login.msp";
    const LOGIN_REFERRER_PATH: &str = "/security/login.msp";
    const LOGIN_TEST_PATH: &str = "cms/members/members-home/";

    #[derive(Debug, serde::Serialize)]
    struct LoginForm {
        user: String,
        password: String,
        action: String,
        #[serde(rename = "Submit")]
        submit: String,
    }

    impl LoginForm {
        fn new() -> Self {
            let user = std::env::var("GC_USERNAME").expect("GC_USERNAME not set");
            let password = std::env::var("GC_PASSWORD").expect("GC_PASSWORD not set");

            Self {
                user: user.to_string(),
                password: password.to_string(),
                action: "login".to_string(),
                submit: "Login".to_string(),
            }
        }
    }

    impl GolfClient {
        pub async fn login(&self) -> anyhow::Result<()> {
            let form = LoginForm::new();
            let url = format!("{}/{}", self.base_url, LOGIN_POST_PATH);
            self.http_client.post(&url).form(&form).send().await?;
            Ok(())
        }
    }
}

mod booking {
    use super::*;
    const AJAX_PATH: &str = "/members/Ajax";

    #[derive(Debug, serde::Serialize)]
    struct BookingParams {
        #[serde(rename = "doAction")]
        action: String,
        #[serde(rename = "rowId")]
        row_id: u32,
        #[serde(rename = "xIndex")]
        index: String,
        #[serde(rename = "memberId")]
        member_id: String,
        #[serde(rename = "myGroup")]
        my_group: bool,
        #[serde(rename = "findAlternative")]
        find_alternative: bool,
    }

    impl BookingParams {
        fn new(row_id: u32) -> Self {
            let member_id = std::env::var("NSWGC_MEMBER_ID").expect("NSWGC_MEMBER_ID not set");

            Self {
                action: "makeBooking".to_string(),
                row_id,
                index: "1".to_string(),
                member_id,
                my_group: true,
                find_alternative: false,
            }
        }
    }

    impl GolfClient {
        pub(crate) async fn book(&self, booking_group_id: u32) -> anyhow::Result<()> {
            let params = BookingParams::new(booking_group_id);
            let url = format!("{}{}", self.base_url, AJAX_PATH);
            self.http_client.post(&url).form(&params).send().await?;
            todo!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
