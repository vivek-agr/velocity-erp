// This code initializes the database and creates a "Company" record. In Tally, this is equivalent to "Creating a Company."
use surrealdb::engine::local::File;
use surrealdb::Surreal;
use serde::{Deserialize, Serialize};

// Define your Data Models
#[derive(Debug, Serialize, Deserialize)]
struct Company {
    name: String,
    gst_number: String,
    currency: String,
}

pub struct DbInstance {
    pub client: Surreal<surrealdb::engine::local::Db>,
}

impl DbInstance {
    pub async fn init() -> surrealdb::Result<Self> {
        // 1. Connect to a local file (This creates 'velocity.db' in your app folder)
        let db = Surreal::new::<File>("velocity.db").await?;

        // 2. Select a Namespace and Database
        db.use_ns("velocity_erp").use_db("accounting").await?;

        Ok(DbInstance { client: db })
    }

    pub async fn create_company(&self, name: &str, gst: &str) -> surrealdb::Result<()> {
        let _: Option<Company> = self.client
            .create(("company", "main")) // Unique ID: 'company:main'
            .content(Company {
                name: name.into(),
                gst_number: gst.into(),
                currency: "INR".into(),
            })
            .await?;
        Ok(())
    }
}