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

#[cfg_attr(test, automock)]
pub trait Callback {
    fn complete_callback(&self);
}

pub fn run_callback<F>(callback: F) where F: Callback {
    callback.complete_callback();
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

    #[test]
    fn check_callback_complete_run() {
        // Create mock object
        let mut mock_callback = MockCallback::new();
        mock_callback.expect_complete_callback()
            .times(1)
            .returning(|| ());
            run_callback(mock_callback);
    }
}

// https://stackoverflow.com/questions/32050478/how-to-test-a-function-with-a-callback