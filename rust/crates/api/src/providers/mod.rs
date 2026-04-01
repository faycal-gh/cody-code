use std::future::Future;
use std::pin::Pin;

use crate::error::ApiError;
use crate::types::{MessageRequest, MessageResponse};

pub mod cody_provider;
pub mod openai_compat;

pub type ProviderFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, ApiError>> + Send + 'a>>;

pub trait Provider {
    type Stream;

    fn send_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, MessageResponse>;

    fn stream_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, Self::Stream>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    CodyApi,
    Xai,
    OpenAi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderMetadata {
    pub provider: ProviderKind,
    pub auth_env: &'static str,
    pub base_url_env: &'static str,
    pub default_base_url: &'static str,
}

const MODEL_REGISTRY: &[(&str, ProviderMetadata)] = &[
    (
        "opus",
        ProviderMetadata {
            provider: ProviderKind::CodyApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: cody_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "sonnet",
        ProviderMetadata {
            provider: ProviderKind::CodyApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: cody_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "haiku",
        ProviderMetadata {
            provider: ProviderKind::CodyApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: cody_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "claude-opus-4-6",
        ProviderMetadata {
            provider: ProviderKind::CodyApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: cody_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "claude-sonnet-4-6",
        ProviderMetadata {
            provider: ProviderKind::CodyApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: cody_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "claude-haiku-4-5-20251213",
        ProviderMetadata {
            provider: ProviderKind::CodyApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: cody_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "grok",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-3",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-mini",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-3-mini",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-2",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "gpt-oss-120b",
        ProviderMetadata {
            provider: ProviderKind::OpenAi,
            auth_env: "GROQ_API_KEY",
            base_url_env: "GROQ_BASE_URL",
            default_base_url: openai_compat::DEFAULT_GROQ_BASE_URL,
        },
    ),
    (
        "openai/gpt-oss-120b",
        ProviderMetadata {
            provider: ProviderKind::OpenAi,
            auth_env: "GROQ_API_KEY",
            base_url_env: "GROQ_BASE_URL",
            default_base_url: openai_compat::DEFAULT_GROQ_BASE_URL,
        },
    ),
    (
        "gpt-oss-20b",
        ProviderMetadata {
            provider: ProviderKind::OpenAi,
            auth_env: "GROQ_API_KEY",
            base_url_env: "GROQ_BASE_URL",
            default_base_url: openai_compat::DEFAULT_GROQ_BASE_URL,
        },
    ),
    (
        "openai/gpt-oss-20b",
        ProviderMetadata {
            provider: ProviderKind::OpenAi,
            auth_env: "GROQ_API_KEY",
            base_url_env: "GROQ_BASE_URL",
            default_base_url: openai_compat::DEFAULT_GROQ_BASE_URL,
        },
    ),
    (
        "gemini-3-flash-preview",
        ProviderMetadata {
            provider: ProviderKind::OpenAi,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_GEMINI_BASE_URL,
        },
    ),
];

#[must_use]
pub fn resolve_model_alias(model: &str) -> String {
    let trimmed = model.trim();
    let lower = trimmed.to_ascii_lowercase();
    MODEL_REGISTRY
        .iter()
        .find_map(|(alias, metadata)| {
            (*alias == lower).then_some(match metadata.provider {
                ProviderKind::CodyApi => match *alias {
                    "opus" => "claude-opus-4-6",
                    "sonnet" => "claude-sonnet-4-6",
                    "haiku" => "claude-haiku-4-5-20251213",
                    _ => trimmed,
                },
                ProviderKind::Xai => match *alias {
                    "grok" | "grok-3" => "grok-3",
                    "grok-mini" | "grok-3-mini" => "grok-3-mini",
                    "grok-2" => "grok-2",
                    _ => trimmed,
                },
                ProviderKind::OpenAi => trimmed,
            })
        })
        .map_or_else(|| trimmed.to_string(), ToOwned::to_owned)
}

#[must_use]
pub fn metadata_for_model(model: &str) -> Option<ProviderMetadata> {
    let canonical = resolve_model_alias(model);
    let lower = canonical.to_ascii_lowercase();
    if let Some((_, metadata)) = MODEL_REGISTRY.iter().find(|(alias, _)| *alias == lower) {
        return Some(*metadata);
    }
    if lower.starts_with("grok") {
        return Some(ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        });
    }
    if is_groq_model_name(&canonical) {
        return Some(ProviderMetadata {
            provider: ProviderKind::OpenAi,
            auth_env: "GROQ_API_KEY",
            base_url_env: "GROQ_BASE_URL",
            default_base_url: openai_compat::DEFAULT_GROQ_BASE_URL,
        });
    }
    if is_gemini_model_name(&canonical) {
        return Some(ProviderMetadata {
            provider: ProviderKind::OpenAi,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_GEMINI_BASE_URL,
        });
    }
    if is_hugging_face_model_name(&canonical) {
        return Some(ProviderMetadata {
            provider: ProviderKind::OpenAi,
            auth_env: "HF_TOKEN",
            base_url_env: "OPENAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_HUGGING_FACE_BASE_URL,
        });
    }
    if is_openai_model_name(&lower) {
        return Some(ProviderMetadata {
            provider: ProviderKind::OpenAi,
            auth_env: "OPENAI_API_KEY",
            base_url_env: "OPENAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_OPENAI_BASE_URL,
        });
    }
    None
}

fn is_openai_model_name(model: &str) -> bool {
    model.starts_with("gpt-")
        || model.starts_with("o1")
        || model.starts_with("o3")
        || model.starts_with("o4")
        || model.starts_with("codex")
}

#[must_use]
pub fn is_groq_model_name(model: &str) -> bool {
    let trimmed = model.trim().to_ascii_lowercase();
    matches!(
        trimmed.as_str(),
        "gpt-oss-120b" | "openai/gpt-oss-120b" | "gpt-oss-20b" | "openai/gpt-oss-20b"
    )
}

#[must_use]
pub fn is_gemini_model_name(model: &str) -> bool {
    model.trim().to_ascii_lowercase().starts_with("gemini-")
}

#[must_use]
pub fn is_hugging_face_model_name(model: &str) -> bool {
    let trimmed = model.trim();
    if trimmed.is_empty() || trimmed.contains(' ') {
        return false;
    }
    let Some((namespace, name)) = trimmed.split_once('/') else {
        return false;
    };
    !namespace.is_empty() && !name.is_empty() && !name.contains('/')
}

#[must_use]
pub fn detect_provider_kind(model: &str) -> ProviderKind {
    if let Some(metadata) = metadata_for_model(model) {
        return metadata.provider;
    }
    if cody_provider::has_auth_from_env_or_saved().unwrap_or(false) {
        return ProviderKind::CodyApi;
    }
    if openai_compat::has_api_key("OPENAI_API_KEY") {
        return ProviderKind::OpenAi;
    }
    if openai_compat::has_api_key("GROQ_API_KEY") {
        return ProviderKind::OpenAi;
    }
    if openai_compat::has_api_key("GEMINI_API_KEY") {
        return ProviderKind::OpenAi;
    }
    if openai_compat::has_api_key("XAI_API_KEY") {
        return ProviderKind::Xai;
    }
    ProviderKind::CodyApi
}

#[must_use]
pub fn max_tokens_for_model(model: &str) -> u32 {
    let canonical = resolve_model_alias(model);
    if canonical.contains("opus")
        || (is_hugging_face_model_name(&canonical) && !is_groq_model_name(&canonical))
    {
        32_000
    } else {
        64_000
    }
}

#[cfg(test)]
mod tests {
    use super::{
        detect_provider_kind, is_gemini_model_name, is_groq_model_name, max_tokens_for_model,
        resolve_model_alias, ProviderKind,
    };

    #[test]
    fn resolves_grok_aliases() {
        assert_eq!(resolve_model_alias("grok"), "grok-3");
        assert_eq!(resolve_model_alias("grok-mini"), "grok-3-mini");
        assert_eq!(resolve_model_alias("grok-2"), "grok-2");
        assert_eq!(resolve_model_alias("gpt-oss-120b"), "gpt-oss-120b");
        assert_eq!(
            resolve_model_alias("gemini-3-flash-preview"),
            "gemini-3-flash-preview"
        );
    }

    #[test]
    fn detects_provider_from_model_name_first() {
        assert_eq!(detect_provider_kind("grok"), ProviderKind::Xai);
        assert_eq!(
            detect_provider_kind("claude-sonnet-4-6"),
            ProviderKind::CodyApi
        );
        assert_eq!(detect_provider_kind("gpt-4o"), ProviderKind::OpenAi);
        assert_eq!(
            detect_provider_kind("codex-mini-latest"),
            ProviderKind::OpenAi
        );
        assert_eq!(
            detect_provider_kind("Qwen/Qwen2.5-Coder-32B-Instruct"),
            ProviderKind::OpenAi
        );
        assert_eq!(
            detect_provider_kind("openai/gpt-oss-120b"),
            ProviderKind::OpenAi
        );
        assert_eq!(
            detect_provider_kind("gemini-3-flash-preview"),
            ProviderKind::OpenAi
        );
    }

    #[test]
    fn recognizes_groq_oss_models_before_hf_repo_fallback() {
        assert!(is_groq_model_name("openai/gpt-oss-120b"));
        assert!(!is_groq_model_name("Qwen/Qwen2.5-Coder-32B-Instruct"));
    }

    #[test]
    fn recognizes_gemini_models() {
        assert!(is_gemini_model_name("gemini-3-flash-preview"));
        assert!(!is_gemini_model_name("gpt-4o"));
    }

    #[test]
    fn keeps_existing_max_token_heuristic() {
        assert_eq!(max_tokens_for_model("opus"), 32_000);
        assert_eq!(max_tokens_for_model("grok-3"), 64_000);
        assert_eq!(max_tokens_for_model("openai/gpt-oss-120b"), 64_000);
        assert_eq!(
            max_tokens_for_model("Qwen/Qwen2.5-Coder-32B-Instruct"),
            32_000
        );
    }
}
