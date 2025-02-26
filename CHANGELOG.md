# v0.11.0 [UNRELEASED]

## Notice

* The `--webserver` flag is now enabled by default in `tabby serve`. To turn off the webserver and only use OSS features, use the `--no-webserver` flag.
* `/v1beta/chat/completions` is now moved to `/v1/chat/completions`, while the old endpoint is still available for backward compatibility.

## Features

## Fixes and Improvements
* Changed the default model filename from `q8_0.v2.gguf` to `model.gguf` in MODEL_SPEC.md

# v0.10.0 [UNRELEASED]

## Features
* Introduced the `--chat-device` flag to specify the device used to run the chat model.
* Added a "Reports" tab in the web interface, which provides team-wise statistics for Tabby IDE and Extensions usage (e.g., completions, acceptances).
* Enabled the use of segmented models with the `tabby download` command.
* Implemented the "Go to file" functionality in the Code Browser.

## Fixes and Improvements
* Fix worker unregisteration misfunctioning caused by unmatched address.
* Accurate repository context filtering using fuzzy matching on `git_url` field.
* Support the use of client-side context, including function/class declarations from LSP, and relevant snippets from local changed files.

# v0.9.1

## Fixes and Improvements
* Fix worker registration check against enterprise licenses.
* Fix default value of `disable_client_side_telemetry` when `--webserver` is not used.

# v0.9.0

## Features

* Support for SMTP configuration in the user management system.
* Support for SSO and team management as features in the Enterprise tier.
* Fully managed repository indexing using `--webserver`, with job history logging available in the web interface.

# v0.8.3

## Fixes and Improvements

* Ensure `~/.tabby/repositories` exists for tabby scheduler jobs: https://github.com/TabbyML/tabby/pull/1375
* Add cpu only binary `tabby-cpu` to docker distribution.

# v0.8.0

## Notice

* Due to format changes, re-executing `tabby scheduler --now` is required to ensure that `Code Browser` functions properly.

## Features

* Introducing a preview release of the `Source Code Browser`, featuring visualization of code snippets utilized for code completion in RAG.
* Added a Windows CPU binary distribution.
* Added a Linux ROCm (AMD GPU) binary distribution.

## Fixes and Improvements

* Fixed an issue with cached permanent redirection in certain browsers (e.g., Chrome) when the `--webserver` flag is disabled.
* Introduced the `TABBY_MODEL_CACHE_ROOT` environment variable to individually override the model cache directory.
* The `/v1beta/chat/completions` API endpoint is now compatible with OpenAI's chat completion API.
* Models from our official registry can now be referred to without the TabbyML prefix. Therefore, for the model TabbyML/CodeLlama-7B, you can simply refer to it as CodeLlama-7B everywhere.

# v0.7.0 (12/15/2023)

## Features

* Tabby now includes built-in user management and secure access, ensuring that it is only accessible to your team.
* The `--webserver` flag is a new addition to `tabby serve` that enables secure access to the tabby server. When this flag is on, IDE extensions will need to provide an authorization token to access the instance.
  - Some functionalities that are bound to the webserver (e.g. playground) will also require the `--webserver` flag.


## Fixes and Improvements

*  Fix https://github.com/TabbyML/tabby/issues/1036, events log should be written to dated json files.

# v0.6.0 (11/27/2023)

## Features

* Add distribution support (running completion / chat model on different process / machine).
* Add conversation history in chat playground.
* Add `/metrics` endpoint for prometheus metrics collection. 

## Fixes and Improvements

* Fix the slow repository indexing due to constraint memory arena in tantivy index writer.
* Make `--model` optional, so users can create a chat only instance.
* Add `--parallelism` to control the throughput and VRAM usage: https://github.com/TabbyML/tabby/pull/727

# v0.5.5 (11/09/2023)

## Fixes and Improvements

## Notice

* llama.cpp backend (CPU, Metal) now requires a redownload of gguf model due to upstream format changes: https://github.com/TabbyML/tabby/pull/645 https://github.com/ggerganov/llama.cpp/pull/3252
* Due to indexing format changes, the `~/.tabby/index` needs to be manually removed before any further runs of `tabby scheduler`.
* `TABBY_REGISTRY` is replaced with `TABBY_DOWNLOAD_HOST` for the github based registry implementation.

## Features

* Improved dashboard UI.

## Fixes and Improvements

* Cpu backend is switched to llama.cpp: https://github.com/TabbyML/tabby/pull/638
* add `server.completion_timeout` to control the code completion interface timeout: https://github.com/TabbyML/tabby/pull/637
* Cuda backend is switched to llama.cpp: https://github.com/TabbyML/tabby/pull/656
* Tokenizer implementation is switched to llama.cpp, so tabby no longer need to download additional tokenizer file: https://github.com/TabbyML/tabby/pull/683
* Fix deadlock issue reported in https://github.com/TabbyML/tabby/issues/718

# v0.4.0 (10/24/2023)

## Features

* Supports golang: https://github.com/TabbyML/tabby/issues/553
* Supports ruby: https://github.com/TabbyML/tabby/pull/597
* Supports using local directory for `Repository.git_url`: use `file:///path/to/repo` to specify a local directory.
* A new UI design for webserver.

## Fixes and Improvements

* Improve snippets retrieval by dedup candidates to existing content + snippets: https://github.com/TabbyML/tabby/pull/582

# v0.3.1 (10/21/2023)
## Fixes and improvements

* Fix GPU OOM issue caused the parallelism: https://github.com/TabbyML/tabby/issues/541, https://github.com/TabbyML/tabby/issues/587
* Fix git safe directory check in docker: https://github.com/TabbyML/tabby/issues/569

# v0.3.0 (10/13/2023)

## Features
### Retrieval-Augmented Code Completion Enabled by Default

The currently supported languages are:

* Rust
* Python
* JavaScript / JSX
* TypeScript / TSX

A blog series detailing the technical aspects of Retrieval-Augmented Code Completion will be published soon. Stay tuned!

## Fixes and Improvements

* Fix [Issue #511](https://github.com/TabbyML/tabby/issues/511) by marking ggml models as optional.
* Improve stop words handling by combining RegexSet into Regex for efficiency.

# v0.2.2 (10/09/2023)
## Fixes and improvements

* Fix a critical issue that might cause request dead locking in ctranslate2 backend (when loading is heavy)

# v0.2.1 (10/03/2023)
## Features
### Chat Model & Web Interface

We have introduced a new argument, `--chat-model`, which allows you to specify the model for the chat playground located at http://localhost:8080/playground

To utilize this feature, use the following command in the terminal:

```bash
tabby serve --device metal --model TabbyML/StarCoder-1B --chat-model TabbyML/Mistral-7B
```

### ModelScope Model Registry

Mainland Chinese users have been facing challenges accessing Hugging Face due to various reasons. The Tabby team is actively working to address this issue by mirroring models to a hosting provider in mainland China called modelscope.cn.

```bash
# Download from the Modelscope registry
TABBY_REGISTRY=modelscope tabby download --model TabbyML/WizardCoder-1B
```

## Fixes and improvements

* Implemented more accurate UTF-8 incremental decoding in the [GitHub pull request](https://github.com/TabbyML/tabby/pull/491).
* Fixed the stop words implementation by utilizing RegexSet to isolate the stop word group.
* Improved model downloading logic; now Tabby will attempt to fetch the latest model version if there's a remote change, and the local cache key becomes stale.
* set default num_replicas_per_device for ctranslate2 backend to increase parallelism.
