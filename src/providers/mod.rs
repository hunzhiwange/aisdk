//! This module provides the `Provider` trait, which defines the interface for
//! interacting with different AI providers.

#[cfg(feature = "openai")]
pub mod openai;
#[cfg(feature = "openai")]
pub use openai::OpenAI;

// Public OpenAI-compatible provider
#[cfg(feature = "openaicompatible")]
pub mod openai_compatible;
#[cfg(feature = "openaicompatible")]
pub use openai_compatible::OpenAICompatible;

#[cfg(feature = "anthropic")]
pub mod anthropic;
#[cfg(feature = "anthropic")]
pub use anthropic::Anthropic;

#[cfg(feature = "groq")]
pub mod groq;
#[cfg(feature = "groq")]
pub use groq::Groq;

#[cfg(feature = "google")]
pub mod google;
#[cfg(feature = "google")]
pub use google::Google;

#[cfg(feature = "vercel")]
pub mod vercel;
#[cfg(feature = "vercel")]
pub use vercel::Vercel;

#[cfg(feature = "openrouter")]
pub mod openrouter;
#[cfg(feature = "openrouter")]
pub use openrouter::Openrouter;

#[cfg(feature = "mistral")]
pub mod mistral;
#[cfg(feature = "mistral")]
pub use mistral::Mistral;

#[cfg(feature = "amazon-bedrock")]
pub mod amazon_bedrock;
#[cfg(feature = "amazon-bedrock")]
pub use amazon_bedrock::AmazonBedrock;

#[cfg(feature = "togetherai")]
pub mod togetherai;
#[cfg(feature = "togetherai")]
pub use togetherai::TogetherAI;

#[cfg(feature = "xai")]
pub mod xai;
#[cfg(feature = "xai")]
pub use xai::XAI;

// Internal module for OpenAI Chat Completions API compatible providers
#[cfg(feature = "openaichatcompletions")]
pub(crate) mod openai_chat_completions;

// [codegen]
#[cfg(feature = "302ai")]
#[path = "302ai/mod.rs"]
pub mod ai_302;
#[cfg(feature = "302ai")]
pub use ai_302::Ai302;

#[cfg(feature = "abacus")]
pub mod abacus;
#[cfg(feature = "abacus")]
pub use abacus::Abacus;

#[cfg(feature = "abliteration-ai")]
#[path = "abliteration-ai/mod.rs"]
pub mod abliteration_ai;
#[cfg(feature = "abliteration-ai")]
pub use abliteration_ai::AbliterationAi;

#[cfg(feature = "ai-router")]
#[path = "ai-router/mod.rs"]
pub mod ai_router;
#[cfg(feature = "ai-router")]
pub use ai_router::AiRouter;

#[cfg(feature = "aiand")]
pub mod aiand;
#[cfg(feature = "aiand")]
pub use aiand::Aiand;

#[cfg(feature = "aihubmix")]
pub mod aihubmix;
#[cfg(feature = "aihubmix")]
pub use aihubmix::Aihubmix;

#[cfg(feature = "aki-io")]
#[path = "aki-io/mod.rs"]
pub mod aki_io;
#[cfg(feature = "aki-io")]
pub use aki_io::AkiIo;

#[cfg(feature = "alibaba")]
pub mod alibaba;
#[cfg(feature = "alibaba")]
pub use alibaba::Alibaba;

#[cfg(feature = "alibaba-cn")]
#[path = "alibaba-cn/mod.rs"]
pub mod alibaba_cn;
#[cfg(feature = "alibaba-cn")]
pub use alibaba_cn::AlibabaCn;

#[cfg(feature = "alibaba-coding-plan")]
#[path = "alibaba-coding-plan/mod.rs"]
pub mod alibaba_coding_plan;
#[cfg(feature = "alibaba-coding-plan")]
pub use alibaba_coding_plan::AlibabaCodingPlan;

#[cfg(feature = "alibaba-coding-plan-cn")]
#[path = "alibaba-coding-plan-cn/mod.rs"]
pub mod alibaba_coding_plan_cn;
#[cfg(feature = "alibaba-coding-plan-cn")]
pub use alibaba_coding_plan_cn::AlibabaCodingPlanCn;

#[cfg(feature = "alibaba-token-plan")]
#[path = "alibaba-token-plan/mod.rs"]
pub mod alibaba_token_plan;
#[cfg(feature = "alibaba-token-plan")]
pub use alibaba_token_plan::AlibabaTokenPlan;

#[cfg(feature = "alibaba-token-plan-cn")]
#[path = "alibaba-token-plan-cn/mod.rs"]
pub mod alibaba_token_plan_cn;
#[cfg(feature = "alibaba-token-plan-cn")]
pub use alibaba_token_plan_cn::AlibabaTokenPlanCn;

#[cfg(feature = "ambient")]
pub mod ambient;
#[cfg(feature = "ambient")]
pub use ambient::Ambient;

#[cfg(feature = "anyapi")]
pub mod anyapi;
#[cfg(feature = "anyapi")]
pub use anyapi::Anyapi;

#[cfg(feature = "atomic-chat")]
#[path = "atomic-chat/mod.rs"]
pub mod atomic_chat;
#[cfg(feature = "atomic-chat")]
pub use atomic_chat::AtomicChat;

#[cfg(feature = "auriko")]
pub mod auriko;
#[cfg(feature = "auriko")]
pub use auriko::Auriko;

#[cfg(feature = "bailing")]
pub mod bailing;
#[cfg(feature = "bailing")]
pub use bailing::Bailing;

#[cfg(feature = "baseten")]
pub mod baseten;
#[cfg(feature = "baseten")]
pub use baseten::Baseten;

#[cfg(feature = "berget")]
pub mod berget;
#[cfg(feature = "berget")]
pub use berget::Berget;

#[cfg(feature = "blueclaw")]
pub mod blueclaw;
#[cfg(feature = "blueclaw")]
pub use blueclaw::Blueclaw;

#[cfg(feature = "chutes")]
pub mod chutes;
#[cfg(feature = "chutes")]
pub use chutes::Chutes;

#[cfg(feature = "clarifai")]
pub mod clarifai;
#[cfg(feature = "clarifai")]
pub use clarifai::Clarifai;

#[cfg(feature = "claudinio")]
pub mod claudinio;
#[cfg(feature = "claudinio")]
pub use claudinio::Claudinio;

#[cfg(feature = "cline-pass")]
#[path = "cline-pass/mod.rs"]
pub mod cline_pass;
#[cfg(feature = "cline-pass")]
pub use cline_pass::ClinePass;

#[cfg(feature = "cloudferro-sherlock")]
#[path = "cloudferro-sherlock/mod.rs"]
pub mod cloudferro_sherlock;
#[cfg(feature = "cloudferro-sherlock")]
pub use cloudferro_sherlock::CloudferroSherlock;

#[cfg(feature = "cloudflare-ai-gateway")]
#[path = "cloudflare-ai-gateway/mod.rs"]
pub mod cloudflare_ai_gateway;
#[cfg(feature = "cloudflare-ai-gateway")]
pub use cloudflare_ai_gateway::CloudflareAiGateway;

#[cfg(feature = "cloudflare-workers-ai")]
#[path = "cloudflare-workers-ai/mod.rs"]
pub mod cloudflare_workers_ai;
#[cfg(feature = "cloudflare-workers-ai")]
pub use cloudflare_workers_ai::CloudflareWorkersAi;

#[cfg(feature = "cortecs")]
pub mod cortecs;
#[cfg(feature = "cortecs")]
pub use cortecs::Cortecs;

#[cfg(feature = "crof")]
pub mod crof;
#[cfg(feature = "crof")]
pub use crof::Crof;

#[cfg(feature = "crossmodel")]
pub mod crossmodel;
#[cfg(feature = "crossmodel")]
pub use crossmodel::Crossmodel;

#[cfg(feature = "daoxe")]
pub mod daoxe;
#[cfg(feature = "daoxe")]
pub use daoxe::Daoxe;

#[cfg(feature = "databricks")]
pub mod databricks;
#[cfg(feature = "databricks")]
pub use databricks::Databricks;

#[cfg(feature = "deepseek")]
pub mod deepseek;
#[cfg(feature = "deepseek")]
pub use deepseek::Deepseek;

#[cfg(feature = "digitalocean")]
pub mod digitalocean;
#[cfg(feature = "digitalocean")]
pub use digitalocean::Digitalocean;

#[cfg(feature = "dinference")]
pub mod dinference;
#[cfg(feature = "dinference")]
pub use dinference::Dinference;

#[cfg(feature = "drun")]
pub mod drun;
#[cfg(feature = "drun")]
pub use drun::Drun;

#[cfg(feature = "ebcloud")]
pub mod ebcloud;
#[cfg(feature = "ebcloud")]
pub use ebcloud::Ebcloud;

#[cfg(feature = "empiriolabs")]
pub mod empiriolabs;
#[cfg(feature = "empiriolabs")]
pub use empiriolabs::Empiriolabs;

#[cfg(feature = "evroc")]
pub mod evroc;
#[cfg(feature = "evroc")]
pub use evroc::Evroc;

#[cfg(feature = "fastrouter")]
pub mod fastrouter;
#[cfg(feature = "fastrouter")]
pub use fastrouter::Fastrouter;

#[cfg(feature = "fireworks-ai")]
#[path = "fireworks-ai/mod.rs"]
pub mod fireworks_ai;
#[cfg(feature = "fireworks-ai")]
pub use fireworks_ai::FireworksAi;

#[cfg(feature = "firmware")]
pub mod firmware;
#[cfg(feature = "firmware")]
pub use firmware::Firmware;

#[cfg(feature = "friendli")]
pub mod friendli;
#[cfg(feature = "friendli")]
pub use friendli::Friendli;

#[cfg(feature = "frogbot")]
pub mod frogbot;
#[cfg(feature = "frogbot")]
pub use frogbot::Frogbot;

#[cfg(feature = "github-copilot")]
#[path = "github-copilot/mod.rs"]
pub mod github_copilot;
#[cfg(feature = "github-copilot")]
pub use github_copilot::GithubCopilot;

#[cfg(feature = "github-models")]
#[path = "github-models/mod.rs"]
pub mod github_models;
#[cfg(feature = "github-models")]
pub use github_models::GithubModels;

#[cfg(feature = "gmicloud")]
pub mod gmicloud;
#[cfg(feature = "gmicloud")]
pub use gmicloud::Gmicloud;

#[cfg(feature = "greenpt")]
pub mod greenpt;
#[cfg(feature = "greenpt")]
pub use greenpt::Greenpt;

#[cfg(feature = "helicone")]
pub mod helicone;
#[cfg(feature = "helicone")]
pub use helicone::Helicone;

#[cfg(feature = "hetzner")]
pub mod hetzner;
#[cfg(feature = "hetzner")]
pub use hetzner::Hetzner;

#[cfg(feature = "hpc-ai")]
#[path = "hpc-ai/mod.rs"]
pub mod hpc_ai;
#[cfg(feature = "hpc-ai")]
pub use hpc_ai::HpcAi;

#[cfg(feature = "huggingface")]
pub mod huggingface;
#[cfg(feature = "huggingface")]
pub use huggingface::Huggingface;

#[cfg(feature = "hyper")]
pub mod hyper;
#[cfg(feature = "hyper")]
pub use hyper::Hyper;

#[cfg(feature = "iflowcn")]
pub mod iflowcn;
#[cfg(feature = "iflowcn")]
pub use iflowcn::Iflowcn;

#[cfg(feature = "impossibl")]
pub mod impossibl;
#[cfg(feature = "impossibl")]
pub use impossibl::Impossibl;

#[cfg(feature = "inception")]
pub mod inception;
#[cfg(feature = "inception")]
pub use inception::Inception;

#[cfg(feature = "inceptron")]
pub mod inceptron;
#[cfg(feature = "inceptron")]
pub use inceptron::Inceptron;

#[cfg(feature = "inference")]
pub mod inference;
#[cfg(feature = "inference")]
pub use inference::Inference;

#[cfg(feature = "inferx")]
pub mod inferx;
#[cfg(feature = "inferx")]
pub use inferx::Inferx;

#[cfg(feature = "io-net")]
#[path = "io-net/mod.rs"]
pub mod io_net;
#[cfg(feature = "io-net")]
pub use io_net::IoNet;

#[cfg(feature = "jiekou")]
pub mod jiekou;
#[cfg(feature = "jiekou")]
pub use jiekou::Jiekou;

#[cfg(feature = "kenari")]
pub mod kenari;
#[cfg(feature = "kenari")]
pub use kenari::Kenari;

#[cfg(feature = "kilo")]
pub mod kilo;
#[cfg(feature = "kilo")]
pub use kilo::Kilo;

#[cfg(feature = "kuae-cloud-coding-plan")]
#[path = "kuae-cloud-coding-plan/mod.rs"]
pub mod kuae_cloud_coding_plan;
#[cfg(feature = "kuae-cloud-coding-plan")]
pub use kuae_cloud_coding_plan::KuaeCloudCodingPlan;

#[cfg(feature = "lilac")]
pub mod lilac;
#[cfg(feature = "lilac")]
pub use lilac::Lilac;

#[cfg(feature = "llama")]
pub mod llama;
#[cfg(feature = "llama")]
pub use llama::Llama;

#[cfg(feature = "llmgateway")]
pub mod llmgateway;
#[cfg(feature = "llmgateway")]
pub use llmgateway::Llmgateway;

#[cfg(feature = "llmtr")]
pub mod llmtr;
#[cfg(feature = "llmtr")]
pub use llmtr::Llmtr;

#[cfg(feature = "lmstudio")]
pub mod lmstudio;
#[cfg(feature = "lmstudio")]
pub use lmstudio::Lmstudio;

#[cfg(feature = "longcat")]
pub mod longcat;
#[cfg(feature = "longcat")]
pub use longcat::Longcat;

#[cfg(feature = "lucidquery")]
pub mod lucidquery;
#[cfg(feature = "lucidquery")]
pub use lucidquery::Lucidquery;

#[cfg(feature = "lynkr")]
pub mod lynkr;
#[cfg(feature = "lynkr")]
pub use lynkr::Lynkr;

#[cfg(feature = "meganova")]
pub mod meganova;
#[cfg(feature = "meganova")]
pub use meganova::Meganova;

#[cfg(feature = "mixlayer")]
pub mod mixlayer;
#[cfg(feature = "mixlayer")]
pub use mixlayer::Mixlayer;

#[cfg(feature = "moark")]
pub mod moark;
#[cfg(feature = "moark")]
pub use moark::Moark;

#[cfg(feature = "modal")]
pub mod modal;
#[cfg(feature = "modal")]
pub use modal::Modal;

#[cfg(feature = "model-oracle-ai")]
#[path = "model-oracle-ai/mod.rs"]
pub mod model_oracle_ai;
#[cfg(feature = "model-oracle-ai")]
pub use model_oracle_ai::ModelOracleAi;

#[cfg(feature = "modelis")]
pub mod modelis;
#[cfg(feature = "modelis")]
pub use modelis::Modelis;

#[cfg(feature = "modelscope")]
pub mod modelscope;
#[cfg(feature = "modelscope")]
pub use modelscope::Modelscope;

#[cfg(feature = "moonshotai")]
pub mod moonshotai;
#[cfg(feature = "moonshotai")]
pub use moonshotai::Moonshotai;

#[cfg(feature = "moonshotai-cn")]
#[path = "moonshotai-cn/mod.rs"]
pub mod moonshotai_cn;
#[cfg(feature = "moonshotai-cn")]
pub use moonshotai_cn::MoonshotaiCn;

#[cfg(feature = "morph")]
pub mod morph;
#[cfg(feature = "morph")]
pub use morph::Morph;

#[cfg(feature = "nano-gpt")]
#[path = "nano-gpt/mod.rs"]
pub mod nano_gpt;
#[cfg(feature = "nano-gpt")]
pub use nano_gpt::NanoGpt;

#[cfg(feature = "nearai")]
pub mod nearai;
#[cfg(feature = "nearai")]
pub use nearai::Nearai;

#[cfg(feature = "nebius")]
pub mod nebius;
#[cfg(feature = "nebius")]
pub use nebius::Nebius;

#[cfg(feature = "neon")]
pub mod neon;
#[cfg(feature = "neon")]
pub use neon::Neon;

#[cfg(feature = "neuralwatt")]
pub mod neuralwatt;
#[cfg(feature = "neuralwatt")]
pub use neuralwatt::Neuralwatt;

#[cfg(feature = "nova")]
pub mod nova;
#[cfg(feature = "nova")]
pub use nova::Nova;

#[cfg(feature = "novita-ai")]
#[path = "novita-ai/mod.rs"]
pub mod novita_ai;
#[cfg(feature = "novita-ai")]
pub use novita_ai::NovitaAi;

#[cfg(feature = "nvidia")]
pub mod nvidia;
#[cfg(feature = "nvidia")]
pub use nvidia::Nvidia;

#[cfg(feature = "ofox")]
pub mod ofox;
#[cfg(feature = "ofox")]
pub use ofox::Ofox;

#[cfg(feature = "ollama-cloud")]
#[path = "ollama-cloud/mod.rs"]
pub mod ollama_cloud;
#[cfg(feature = "ollama-cloud")]
pub use ollama_cloud::OllamaCloud;

#[cfg(feature = "opencode")]
pub mod opencode;
#[cfg(feature = "opencode")]
pub use opencode::Opencode;

#[cfg(feature = "opencode-go")]
#[path = "opencode-go/mod.rs"]
pub mod opencode_go;
#[cfg(feature = "opencode-go")]
pub use opencode_go::OpencodeGo;

#[cfg(feature = "orcarouter")]
pub mod orcarouter;
#[cfg(feature = "orcarouter")]
pub use orcarouter::Orcarouter;

#[cfg(feature = "ovhcloud")]
pub mod ovhcloud;
#[cfg(feature = "ovhcloud")]
pub use ovhcloud::Ovhcloud;

#[cfg(feature = "pioneer")]
pub mod pioneer;
#[cfg(feature = "pioneer")]
pub use pioneer::Pioneer;

#[cfg(feature = "poe")]
pub mod poe;
#[cfg(feature = "poe")]
pub use poe::Poe;

#[cfg(feature = "poolside")]
pub mod poolside;
#[cfg(feature = "poolside")]
pub use poolside::Poolside;

#[cfg(feature = "qihang-ai")]
#[path = "qihang-ai/mod.rs"]
pub mod qihang_ai;
#[cfg(feature = "qihang-ai")]
pub use qihang_ai::QihangAi;

#[cfg(feature = "qiniu-ai")]
#[path = "qiniu-ai/mod.rs"]
pub mod qiniu_ai;
#[cfg(feature = "qiniu-ai")]
pub use qiniu_ai::QiniuAi;

#[cfg(feature = "regolo-ai")]
#[path = "regolo-ai/mod.rs"]
pub mod regolo_ai;
#[cfg(feature = "regolo-ai")]
pub use regolo_ai::RegoloAi;

#[cfg(feature = "requesty")]
pub mod requesty;
#[cfg(feature = "requesty")]
pub use requesty::Requesty;

#[cfg(feature = "routing-run")]
#[path = "routing-run/mod.rs"]
pub mod routing_run;
#[cfg(feature = "routing-run")]
pub use routing_run::RoutingRun;

#[cfg(feature = "sakana")]
pub mod sakana;
#[cfg(feature = "sakana")]
pub use sakana::Sakana;

#[cfg(feature = "sarvam")]
pub mod sarvam;
#[cfg(feature = "sarvam")]
pub use sarvam::Sarvam;

#[cfg(feature = "scaleway")]
pub mod scaleway;
#[cfg(feature = "scaleway")]
pub use scaleway::Scaleway;

#[cfg(feature = "siliconflow")]
pub mod siliconflow;
#[cfg(feature = "siliconflow")]
pub use siliconflow::Siliconflow;

#[cfg(feature = "siliconflow-cn")]
#[path = "siliconflow-cn/mod.rs"]
pub mod siliconflow_cn;
#[cfg(feature = "siliconflow-cn")]
pub use siliconflow_cn::SiliconflowCn;

#[cfg(feature = "snowflake-cortex")]
#[path = "snowflake-cortex/mod.rs"]
pub mod snowflake_cortex;
#[cfg(feature = "snowflake-cortex")]
pub use snowflake_cortex::SnowflakeCortex;

#[cfg(feature = "stackit")]
pub mod stackit;
#[cfg(feature = "stackit")]
pub use stackit::Stackit;

#[cfg(feature = "stepfun")]
pub mod stepfun;
#[cfg(feature = "stepfun")]
pub use stepfun::Stepfun;

#[cfg(feature = "stepfun-ai")]
#[path = "stepfun-ai/mod.rs"]
pub mod stepfun_ai;
#[cfg(feature = "stepfun-ai")]
pub use stepfun_ai::StepfunAi;

#[cfg(feature = "stepfun-ai-step-plan")]
#[path = "stepfun-ai-step-plan/mod.rs"]
pub mod stepfun_ai_step_plan;
#[cfg(feature = "stepfun-ai-step-plan")]
pub use stepfun_ai_step_plan::StepfunAiStepPlan;

#[cfg(feature = "stepfun-step-plan")]
#[path = "stepfun-step-plan/mod.rs"]
pub mod stepfun_step_plan;
#[cfg(feature = "stepfun-step-plan")]
pub use stepfun_step_plan::StepfunStepPlan;

#[cfg(feature = "submodel")]
pub mod submodel;
#[cfg(feature = "submodel")]
pub use submodel::Submodel;

#[cfg(feature = "synthetic")]
pub mod synthetic;
#[cfg(feature = "synthetic")]
pub use synthetic::Synthetic;

#[cfg(feature = "tencent-coding-plan")]
#[path = "tencent-coding-plan/mod.rs"]
pub mod tencent_coding_plan;
#[cfg(feature = "tencent-coding-plan")]
pub use tencent_coding_plan::TencentCodingPlan;

#[cfg(feature = "tencent-token-plan")]
#[path = "tencent-token-plan/mod.rs"]
pub mod tencent_token_plan;
#[cfg(feature = "tencent-token-plan")]
pub use tencent_token_plan::TencentTokenPlan;

#[cfg(feature = "tencent-tokenhub")]
#[path = "tencent-tokenhub/mod.rs"]
pub mod tencent_tokenhub;
#[cfg(feature = "tencent-tokenhub")]
pub use tencent_tokenhub::TencentTokenhub;

#[cfg(feature = "tensorx")]
pub mod tensorx;
#[cfg(feature = "tensorx")]
pub use tensorx::Tensorx;

#[cfg(feature = "the-grid-ai")]
#[path = "the-grid-ai/mod.rs"]
pub mod the_grid_ai;
#[cfg(feature = "the-grid-ai")]
pub use the_grid_ai::TheGridAi;

#[cfg(feature = "tinfoil")]
pub mod tinfoil;
#[cfg(feature = "tinfoil")]
pub use tinfoil::Tinfoil;

#[cfg(feature = "trustedrouter")]
pub mod trustedrouter;
#[cfg(feature = "trustedrouter")]
pub use trustedrouter::Trustedrouter;

#[cfg(feature = "umans-ai")]
#[path = "umans-ai/mod.rs"]
pub mod umans_ai;
#[cfg(feature = "umans-ai")]
pub use umans_ai::UmansAi;

#[cfg(feature = "umans-ai-coding-plan")]
#[path = "umans-ai-coding-plan/mod.rs"]
pub mod umans_ai_coding_plan;
#[cfg(feature = "umans-ai-coding-plan")]
pub use umans_ai_coding_plan::UmansAiCodingPlan;

#[cfg(feature = "unorouter")]
pub mod unorouter;
#[cfg(feature = "unorouter")]
pub use unorouter::Unorouter;

#[cfg(feature = "upstage")]
pub mod upstage;
#[cfg(feature = "upstage")]
pub use upstage::Upstage;

#[cfg(feature = "vultr")]
pub mod vultr;
#[cfg(feature = "vultr")]
pub use vultr::Vultr;

#[cfg(feature = "wafer.ai")]
#[path = "wafer.ai/mod.rs"]
pub mod wafer_ai;
#[cfg(feature = "wafer.ai")]
pub use wafer_ai::WaferAi;

#[cfg(feature = "wandb")]
pub mod wandb;
#[cfg(feature = "wandb")]
pub use wandb::Wandb;

#[cfg(feature = "xiaomi")]
pub mod xiaomi;
#[cfg(feature = "xiaomi")]
pub use xiaomi::Xiaomi;

#[cfg(feature = "xiaomi-token-plan-ams")]
#[path = "xiaomi-token-plan-ams/mod.rs"]
pub mod xiaomi_token_plan_ams;
#[cfg(feature = "xiaomi-token-plan-ams")]
pub use xiaomi_token_plan_ams::XiaomiTokenPlanAms;

#[cfg(feature = "xiaomi-token-plan-cn")]
#[path = "xiaomi-token-plan-cn/mod.rs"]
pub mod xiaomi_token_plan_cn;
#[cfg(feature = "xiaomi-token-plan-cn")]
pub use xiaomi_token_plan_cn::XiaomiTokenPlanCn;

#[cfg(feature = "xiaomi-token-plan-sgp")]
#[path = "xiaomi-token-plan-sgp/mod.rs"]
pub mod xiaomi_token_plan_sgp;
#[cfg(feature = "xiaomi-token-plan-sgp")]
pub use xiaomi_token_plan_sgp::XiaomiTokenPlanSgp;

#[cfg(feature = "xpersona")]
pub mod xpersona;
#[cfg(feature = "xpersona")]
pub use xpersona::Xpersona;

#[cfg(feature = "zai")]
pub mod zai;
#[cfg(feature = "zai")]
pub use zai::Zai;

#[cfg(feature = "zai-coding-plan")]
#[path = "zai-coding-plan/mod.rs"]
pub mod zai_coding_plan;
#[cfg(feature = "zai-coding-plan")]
pub use zai_coding_plan::ZaiCodingPlan;

#[cfg(feature = "zeldoc")]
pub mod zeldoc;
#[cfg(feature = "zeldoc")]
pub use zeldoc::Zeldoc;

#[cfg(feature = "zenifra")]
pub mod zenifra;
#[cfg(feature = "zenifra")]
pub use zenifra::Zenifra;

#[cfg(feature = "zenmux")]
pub mod zenmux;
#[cfg(feature = "zenmux")]
pub use zenmux::Zenmux;

#[cfg(feature = "zhipuai")]
pub mod zhipuai;
#[cfg(feature = "zhipuai")]
pub use zhipuai::Zhipuai;

#[cfg(feature = "zhipuai-coding-plan")]
#[path = "zhipuai-coding-plan/mod.rs"]
pub mod zhipuai_coding_plan;
#[cfg(feature = "zhipuai-coding-plan")]
pub use zhipuai_coding_plan::ZhipuaiCodingPlan;
// [end-codegen]
