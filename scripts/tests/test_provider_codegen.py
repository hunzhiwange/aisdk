import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path


sys.dont_write_bytecode = True
MODULE_PATH = Path(__file__).resolve().parents[1] / "provider-codegen.py"
SPEC = importlib.util.spec_from_file_location("provider_codegen", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
provider_codegen = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = provider_codegen
SPEC.loader.exec_module(provider_codegen)


def compatible_provider_data() -> dict:
    return {
        "api": "https://example.com/v1",
        "env": ["EXAMPLE_API_KEY"],
        "npm": "@ai-sdk/openai-compatible",
        "models": {
            "example-model": {
                "name": "Example Model",
                "tool_call": True,
                "modalities": {"input": ["text"], "output": ["text"]},
            }
        },
    }


class OpenAICompatibleGenerationTests(unittest.TestCase):
    def test_new_provider_prepares_module_and_capabilities(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)

            writes = provider_codegen.prepare_openai_compatible_provider(
                "fresh-provider",
                compatible_provider_data(),
                with_capabilities=True,
                root=root,
            )

            paths = {write.path.relative_to(root).as_posix() for write in writes}
            self.assertEqual(
                paths,
                {
                    "src/providers/fresh-provider/mod.rs",
                    "src/providers/fresh-provider/capabilities.rs",
                },
            )

    def test_existing_normalized_directory_is_reused_for_capabilities(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            provider_dir = root / "src/providers/amazon_bedrock"
            provider_dir.mkdir(parents=True)
            (provider_dir / "mod.rs").touch()

            writes = provider_codegen.prepare_openai_compatible_provider(
                "amazon-bedrock",
                compatible_provider_data(),
                with_capabilities=True,
                root=root,
            )

            capability_write = next(
                write for write in writes if write.file == "capabilities.rs"
            )
            self.assertEqual(
                capability_write.path.relative_to(root).as_posix(),
                "src/providers/amazon_bedrock/capabilities.rs",
            )

    def test_model_type_is_disambiguated_from_provider_type(self):
        content = provider_codegen.generate_capabilities_rs(
            "claudinio",
            compatible_provider_data()["models"]
            | {
                "claudinio": {
                    "name": "Claudinio",
                    "modalities": {"input": ["text"], "output": ["text"]},
                }
            },
        )

        self.assertIn("provider: Claudinio,", content)
        self.assertIn("ClaudinioModel {", content)
        self.assertIn("constructor_name: claudinio_model,", content)


class ProviderRegistryGenerationTests(unittest.TestCase):
    def test_registry_contains_only_openai_compatible_providers(self):
        providers = {
            "compatible": {
                "api": "https://compatible.example/v1",
                "env": ["COMPATIBLE_API_KEY"],
                "npm": "@ai-sdk/openai-compatible",
            },
            "native-openai": {
                "api": "https://native.example/v1",
                "env": ["NATIVE_API_KEY"],
                "npm": "@ai-sdk/openai",
            },
        }

        content = provider_codegen.generate_provider_registry_rs(providers)

        self.assertIn('#[cfg(feature = "compatible")]', content)
        self.assertIn('id: "compatible"', content)
        self.assertIn(
            "protocol: super::registry::ProviderProtocol::OpenAiCompatible",
            content,
        )
        self.assertNotIn("native-openai", content)
        self.assertNotIn("NATIVE_API_KEY", content)

    def test_registry_preserves_all_env_names_and_sort_order(self):
        providers = {
            "z-provider": {
                "api": "https://z.example/v1",
                "env": ["Z_PRIMARY_API_KEY", "Z_SECONDARY_API_KEY"],
                "npm": "@ai-sdk/openai-compatible",
            },
            "a-provider": {
                "api": "https://a.example/v1",
                "env": ["A_API_KEY"],
                "npm": "@ai-sdk/openai-compatible",
            },
        }

        content = provider_codegen.generate_provider_registry_rs(providers)

        self.assertLess(
            content.index('id: "a-provider"'),
            content.index('id: "z-provider"'),
        )
        self.assertIn('#[cfg(feature = "a-provider")]', content)
        self.assertIn('#[cfg(feature = "z-provider")]', content)
        self.assertIn(
            'env_names: &["Z_PRIMARY_API_KEY", "Z_SECONDARY_API_KEY"]',
            content,
        )


if __name__ == "__main__":
    unittest.main()
