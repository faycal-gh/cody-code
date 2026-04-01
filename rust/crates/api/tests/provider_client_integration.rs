use std::ffi::OsString;
use std::sync::{Mutex, OnceLock};

use api::{read_xai_base_url, ApiError, AuthSource, ProviderClient, ProviderKind};

#[test]
fn provider_client_routes_grok_aliases_through_xai() {
    let _lock = env_lock();
    let _xai_api_key = EnvVarGuard::set("XAI_API_KEY", Some("xai-test-key"));

    let client = ProviderClient::from_model("grok-mini").expect("grok alias should resolve");

    assert_eq!(client.provider_kind(), ProviderKind::Xai);
}

#[test]
fn provider_client_routes_gpt_models_through_openai() {
    let _lock = env_lock();
    let _openai_api_key = EnvVarGuard::set("OPENAI_API_KEY", Some("openai-test-key"));

    let client = ProviderClient::from_model("gpt-4o").expect("gpt model should resolve");

    assert_eq!(client.provider_kind(), ProviderKind::OpenAi);
}

#[test]
fn provider_client_routes_hugging_face_repo_models_through_openai_compat() {
    let _lock = env_lock();
    let _hf_token = EnvVarGuard::set("HF_TOKEN", Some("hf-test-key"));
    let _openai_api_key = EnvVarGuard::set("OPENAI_API_KEY", None);
    let _anthropic_api_key = EnvVarGuard::set("ANTHROPIC_API_KEY", Some("anthropic-test-key"));

    let client = ProviderClient::from_model("Qwen/Qwen2.5-Coder-32B-Instruct")
        .expect("hugging face repo model should resolve");

    assert_eq!(client.provider_kind(), ProviderKind::OpenAi);
}

#[test]
fn provider_client_routes_groq_oss_models_through_openai_compat() {
    let _lock = env_lock();
    let _groq_api_key = EnvVarGuard::set("GROQ_API_KEY", Some("gsk-test-key"));
    let _hf_token = EnvVarGuard::set("HF_TOKEN", None);
    let _openai_api_key = EnvVarGuard::set("OPENAI_API_KEY", None);

    let client =
        ProviderClient::from_model("openai/gpt-oss-120b").expect("groq oss model should resolve");

    assert_eq!(client.provider_kind(), ProviderKind::OpenAi);
}

#[test]
fn provider_client_routes_gemini_models_through_openai_compat() {
    let _lock = env_lock();
    let _gemini_api_key = EnvVarGuard::set("GEMINI_API_KEY", Some("gemini-test-key"));
    let _openai_api_key = EnvVarGuard::set("OPENAI_API_KEY", None);

    let client =
        ProviderClient::from_model("gemini-3-flash-preview").expect("gemini model should resolve");

    assert_eq!(client.provider_kind(), ProviderKind::OpenAi);
}

#[test]
fn provider_client_reports_missing_xai_credentials_for_grok_models() {
    let _lock = env_lock();
    let _xai_api_key = EnvVarGuard::set("XAI_API_KEY", None);

    let error = ProviderClient::from_model("grok-3")
        .expect_err("grok requests without XAI_API_KEY should fail fast");

    match error {
        ApiError::MissingCredentials { provider, env_vars } => {
            assert_eq!(provider, "xAI");
            assert_eq!(env_vars, &["XAI_API_KEY"]);
        }
        other => panic!("expected missing xAI credentials, got {other:?}"),
    }
}

#[test]
fn provider_client_reports_missing_gemini_credentials_for_gemini_models() {
    let _lock = env_lock();
    let _gemini_api_key = EnvVarGuard::set("GEMINI_API_KEY", None);

    let error = ProviderClient::from_model("gemini-3-flash-preview")
        .expect_err("gemini requests without GEMINI_API_KEY should fail fast");

    match error {
        ApiError::MissingCredentials { provider, env_vars } => {
            assert_eq!(provider, "Google Gemini");
            assert_eq!(env_vars, &["GEMINI_API_KEY"]);
        }
        other => panic!("expected missing Gemini credentials, got {other:?}"),
    }
}

#[test]
fn provider_client_reports_missing_groq_credentials_for_gpt_oss_models() {
    let _lock = env_lock();
    let _groq_api_key = EnvVarGuard::set("GROQ_API_KEY", None);

    let error = ProviderClient::from_model("openai/gpt-oss-120b")
        .expect_err("groq requests without GROQ_API_KEY should fail fast");

    match error {
        ApiError::MissingCredentials { provider, env_vars } => {
            assert_eq!(provider, "Groq");
            assert_eq!(env_vars, &["GROQ_API_KEY"]);
        }
        other => panic!("expected missing Groq credentials, got {other:?}"),
    }
}

#[test]
fn provider_client_reports_missing_openai_credentials_for_codex_models() {
    let _lock = env_lock();
    let _openai_api_key = EnvVarGuard::set("OPENAI_API_KEY", None);

    let error = ProviderClient::from_model("codex-mini-latest")
        .expect_err("codex requests without OPENAI_API_KEY should fail fast");

    match error {
        ApiError::MissingCredentials { provider, env_vars } => {
            assert_eq!(provider, "OpenAI");
            assert_eq!(env_vars, &["OPENAI_API_KEY"]);
        }
        other => panic!("expected missing OpenAI credentials, got {other:?}"),
    }
}

#[test]
fn provider_client_reports_missing_hugging_face_credentials_for_repo_models() {
    let _lock = env_lock();
    let _hf_token = EnvVarGuard::set("HF_TOKEN", None);
    let _hf_hub_token = EnvVarGuard::set("HUGGINGFACEHUB_API_TOKEN", None);
    let _openai_api_key = EnvVarGuard::set("OPENAI_API_KEY", None);

    let error = ProviderClient::from_model("Qwen/Qwen2.5-Coder-32B-Instruct")
        .expect_err("hf repo model without hf/openai credentials should fail fast");

    match error {
        ApiError::MissingCredentials { provider, env_vars } => {
            assert_eq!(provider, "Hugging Face");
            assert_eq!(
                env_vars,
                &["HF_TOKEN", "HUGGINGFACEHUB_API_TOKEN", "OPENAI_API_KEY"]
            );
        }
        other => panic!("expected missing Hugging Face credentials, got {other:?}"),
    }
}

#[test]
fn provider_client_uses_explicit_auth_without_env_lookup() {
    let _lock = env_lock();
    let _api_key = EnvVarGuard::set("ANTHROPIC_API_KEY", None);
    let _auth_token = EnvVarGuard::set("ANTHROPIC_AUTH_TOKEN", None);

    let client = ProviderClient::from_model_with_default_auth(
        "claude-sonnet-4-6",
        Some(AuthSource::ApiKey("cody-test-key".to_string())),
    )
    .expect("explicit auth should avoid env lookup");

    assert_eq!(client.provider_kind(), ProviderKind::CodyApi);
}

#[test]
fn read_xai_base_url_prefers_env_override() {
    let _lock = env_lock();
    let _xai_base_url = EnvVarGuard::set("XAI_BASE_URL", Some("https://example.xai.test/v1"));

    assert_eq!(read_xai_base_url(), "https://example.xai.test/v1");
}

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

struct EnvVarGuard {
    key: &'static str,
    original: Option<OsString>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: Option<&str>) -> Self {
        let original = std::env::var_os(key);
        match value {
            Some(value) => std::env::set_var(key, value),
            None => std::env::remove_var(key),
        }
        Self { key, original }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match &self.original {
            Some(value) => std::env::set_var(self.key, value),
            None => std::env::remove_var(self.key),
        }
    }
}
