#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait Database {
    fn execute_query(&self, query: String);
}

pub fn get_user(db: Box<dyn Database>, id: i32) {
    let query = format!("select * from users where id = {}", id);
    db.execute_query(query);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_user_executes_correct_query() {
        // Create mock object
        let mut mock_database = Box::new(MockDatabase::new());
        // Make all expectation
        mock_database.expect_execute_query()
            .with(eq("select * from users where id = 22".to_owned()))
            .once()
            .returning(|_x| ());
        // Run code to check
        get_user(mock_database, 22);
    }
}
