pub mod proto {
    #[path = "pingquery.api.rs"]
    pub mod api;
}

mod config;
pub mod persistence;
pub mod server;
pub mod value;

#[cfg(test)]
mod test {
    #[test]
    fn hello() {
        assert_eq!(2 + 2, 4);
    }
}
