use std::sync::Arc;

use axum::extract::FromRef;

use crate::application::auth::register::usecase::RegisterUseCase;

#[derive(Clone)]
pub struct AppState {
    pub register_usecase: Arc<RegisterUseCase>,
}

impl FromRef<AppState> for Arc<RegisterUseCase> {
    fn from_ref(state: &AppState) -> Self {
        state.register_usecase.clone()
    }
}
