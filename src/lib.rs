#[derive(Clone)]
pub struct U2f {
    app_id: String,
}

impl U2f {
    // The app ID is a string used to uniquely identify an U2F app
    pub fn new(app_id: String) -> Self {
        U2f {
            app_id: app_id,
        }
    }

    pub fn register(&self) -> String {
       "".into()
    }
}