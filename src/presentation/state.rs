use axum::extract::FromRef;

use crate::application::auth::register::usecase::Register;

#[derive(Clone)]
pub struct AppState {
    pub register_usecase: Register,
}

impl FromRef<AppState> for Register {
    fn from_ref(state: &AppState) -> Self {
        state.register_usecase.clone()
    }
}
