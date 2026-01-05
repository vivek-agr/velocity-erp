// Bridge function
pub async fn find_local_company_server() -> Option<String> {
    crate::core::network::discover_server().ok()
}