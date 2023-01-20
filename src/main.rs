use cli::{TaskAdd, TaskGet};
use persistency::MongoDBClient;
use todo::{create_task, get_task};
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let args = cli::parse();
    let mongo_db_client = MongoDBClient::new();
    match args.task {
        cli::Task::Add(TaskAdd { title }) => {
            if let Some(id) = create_task(mongo_db_client, &title) {
                println!("id: {}", id);
            }
        }
        cli::Task::Get(TaskGet { id }) => {
            let task = get_task(mongo_db_client, id);
            if let Some(task) = task {
                println!("title: {}", task.title);
            } else {
                println!("No entry found.");
            }
        }
    }
}

mod cli {
    use clap::{Args, Parser, Subcommand};

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    pub struct Cli {
        #[command(subcommand)]
        pub task: Task,
    }

    #[derive(Subcommand)]
    pub enum Task {
        Add(TaskAdd),
        Get(TaskGet),
    }

    #[derive(Args)]
    pub struct TaskAdd {
        pub title: String,
    }

    #[derive(Args)]
    pub struct TaskGet {
        pub id: String,
    }

    pub fn parse() -> Cli {
        Cli::parse()
    }
}

mod persistency {
    use std::str::FromStr;

    use crate::todo::{PersistencyDriver, Task};
    use mongodb::{
        bson::{doc, oid::ObjectId},
        options::ClientOptions,
        sync::Client,
    };

    pub struct MongoDBClient {
        client: Client,
    }

    impl MongoDBClient {
        pub fn new() -> Self {
            let error_message = "Something went wrong when connecting to the database. Check if the MONGO_DB_URL env variable is correct and the server is running.";
            let mongo_db_url = std::env::var("MONGO_DB_URL").expect(error_message);
            let mut client_options =
                ClientOptions::parse(mongo_db_url).expect(error_message);
            client_options.app_name = Some("Task".to_string());
            let client = Client::with_options(client_options).expect(error_message);
            Self { client }
        }
    }

    impl PersistencyDriver for MongoDBClient {
        fn insert(&self, title: &str) -> Option<String> {
            let collection = self.client.database("task").collection("entry");
            let task = collection.insert_one(
                Task {
                    title: title.into(),
                },
                None,
            );
            let id = task.ok()?.inserted_id.as_object_id()?.to_string();
            Some(id)
        }

        fn get(&self, id: String) -> Option<Task> {
            let collection = self.client.database("task").collection("entry");
            let id = ObjectId::from_str(&id).ok()?;
            collection
                .find_one(doc! { "_id": id }, None)
                .unwrap_or(None)
        }
    }
}

mod todo {
    use serde::{Deserialize, Serialize};

    pub trait PersistencyDriver {
        fn insert(&self, title: &str) -> Option<String>;
        fn get(&self, id: String) -> Option<Task>;
    }

    pub fn create_task<P: PersistencyDriver>(persistency: P, title: &str) -> Option<String> {
        persistency.insert(title)
    }

    pub fn get_task<P: PersistencyDriver>(persistency: P, id: String) -> Option<Task> {
        persistency.get(id)
    }

    #[derive(Serialize, Deserialize)]
    pub struct Task {
        pub title: String,
    }
}
