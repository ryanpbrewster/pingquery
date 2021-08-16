pub mod proto {
    #[path = "pingquery.api.rs"]
    pub mod api;
}

pub mod actor;
pub mod config;
pub mod diagnostics;
pub mod persistence;
pub mod requests;
pub mod server;
pub mod value;

#[cfg(test)]
mod test {
    #[test]
    fn hello() {
        assert_eq!(2 + 2, 4);
    }
}
