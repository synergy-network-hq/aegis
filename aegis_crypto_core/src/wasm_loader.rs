//! WASM Loader Utility
//!
//! This module provides utilities for dynamically loading external WASM files
//! and managing their lifecycle in the browser environment.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_sys::{ Object, Reflect, Uint8Array, WebAssembly };
use wasm_bindgen::JsCast;
use web_sys::{ Request, RequestInit, RequestMode, Response };

use std::{ vec::Vec, string::{ String, ToString } };

/// WASM module cache for storing loaded modules
pub struct WasmModuleCache {
    modules: std::collections::HashMap<String, WebAssembly::Instance>,
}

impl WasmModuleCache {
    /// Create a new WASM module cache
    pub fn new() -> Self {
        Self {
            modules: std::collections::HashMap::new(),
        }
    }

    /// Check if a module is cached
    pub fn has_module(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    /// Get a cached module
    pub fn get_module(&self, name: &str) -> Option<&WebAssembly::Instance> {
        self.modules.get(name)
    }

    /// Store a module in the cache
    pub fn store_module(&mut self, name: String, instance: WebAssembly::Instance) {
        self.modules.insert(name, instance);
    }

    /// Remove a module from the cache
    pub fn remove_module(&mut self, name: &str) -> Option<WebAssembly::Instance> {
        self.modules.remove(name)
    }

    /// Clear all cached modules
    pub fn clear(&mut self) {
        self.modules.clear();
    }

    /// Get the number of cached modules
    pub fn len(&self) -> usize {
        self.modules.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }
}

/// WASM loader for external modules
pub struct ExternalWasmLoader {
    cache: WasmModuleCache,
    base_path: String,
}

impl ExternalWasmLoader {
    /// Create a new external WASM loader
    pub fn new(base_path: String) -> Self {
        Self {
            cache: WasmModuleCache::new(),
            base_path,
        }
    }

    /// Load a WASM module from a file path
    pub async fn load_module(&mut self, filename: &str) -> Result<WebAssembly::Instance, JsValue> {
        // Check if already cached
        if let Some(instance) = self.cache.get_module(filename) {
            return Ok(instance.clone());
        }

        // Construct full path
        let full_path = if self.base_path.ends_with('/') {
            format!("{}{}", self.base_path, filename)
        } else {
            format!("{}/{}", self.base_path, filename)
        };

        // Load the WASM file
        let instance = self.fetch_and_instantiate(&full_path).await?;

        // Cache the instance
        self.cache.store_module(filename.to_string(), instance.clone());

        Ok(instance)
    }

    /// Fetch and instantiate a WASM module
    async fn fetch_and_instantiate(&self, url: &str) -> Result<WebAssembly::Instance, JsValue> {
        // Create a fetch request
        let mut opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(url, &opts)?;

        // Fetch the WASM file
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
        let fetch_promise = window.fetch_with_request(&request);

        // Convert to async/await
        let response: Response = wasm_bindgen_futures::JsFuture
            ::from(fetch_promise).await?
            .dyn_into()?;

        if !response.ok() {
            return Err(
                JsValue::from_str(&format!("Failed to fetch WASM file: {}", response.status()))
            );
        }

        // Get the response as an array buffer
        let array_buffer_promise = response.array_buffer()?;
        let array_buffer: js_sys::ArrayBuffer = wasm_bindgen_futures::JsFuture
            ::from(array_buffer_promise).await?
            .dyn_into()?;

        // Convert to Uint8Array and then to &[u8]
        let wasm_bytes = Uint8Array::new(&array_buffer);
        let wasm_slice = wasm_bytes.to_vec();

        // Instantiate the WASM module
        let instantiate_promise = WebAssembly::instantiate_buffer(
            &wasm_slice,
            &js_sys::Object::new()
        );
        let result = wasm_bindgen_futures::JsFuture::from(instantiate_promise).await?;

        // Extract the instance
        let instance = Reflect::get(&result, &"instance".into())?;
        let instance: WebAssembly::Instance = instance.dyn_into()?;

        Ok(instance)
    }

    /// Get a cached module or load it if not cached
    pub async fn get_or_load_module(
        &mut self,
        filename: &str
    ) -> Result<WebAssembly::Instance, JsValue> {
        if self.cache.has_module(filename) {
            self.cache
                .get_module(filename)
                .cloned()
                .ok_or_else(|| JsValue::from_str("Failed to get cached module"))
        } else {
            self.load_module(filename).await
        }
    }

    /// Preload multiple modules
    pub async fn preload_modules(&mut self, filenames: &[&str]) -> Result<(), JsValue> {
        for filename in filenames {
            self.load_module(filename).await?;
        }
        Ok(())
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> JsValue {
        let stats = Object::new();
        Reflect::set(&stats, &"cached_modules".into(), &(self.cache.len() as u32).into()).unwrap();
        Reflect::set(&stats, &"is_empty".into(), &self.cache.is_empty().into()).unwrap();
        stats.into()
    }
}

/// JavaScript bindings for the WASM loader
#[wasm_bindgen]
pub struct WasmLoaderJs {
    loader: ExternalWasmLoader,
}

#[wasm_bindgen]
impl WasmLoaderJs {
    /// Create a new WASM loader
    #[wasm_bindgen(constructor)]
    pub fn new(base_path: &str) -> Self {
        Self {
            loader: ExternalWasmLoader::new(base_path.to_string()),
        }
    }

    /// Load a WASM module
    #[wasm_bindgen]
    pub async fn load_module(&mut self, filename: &str) -> Result<JsValue, JsValue> {
        let instance = self.loader.load_module(filename).await?;
        Ok(instance.into())
    }

    /// Get or load a module
    #[wasm_bindgen]
    pub async fn get_or_load_module(&mut self, filename: &str) -> Result<JsValue, JsValue> {
        let instance = self.loader.get_or_load_module(filename).await?;
        Ok(instance.into())
    }

    /// Preload multiple modules
    #[wasm_bindgen]
    pub async fn preload_modules(&mut self, filenames: &JsValue) -> Result<(), JsValue> {
        let filenames_array: js_sys::Array = filenames.clone().dyn_into()?;
        let mut filenames_vec = Vec::new();

        for i in 0..filenames_array.length() {
            let filename = filenames_array.get(i);
            if let Some(filename_str) = filename.as_string() {
                filenames_vec.push(filename_str);
            }
        }

        // Convert Vec<String> to Vec<&str> for the loader
        let filenames_refs: Vec<&str> = filenames_vec
            .iter()
            .map(|s| s.as_str())
            .collect();
        self.loader.preload_modules(&filenames_refs).await
    }

    /// Get cache statistics
    #[wasm_bindgen]
    pub fn cache_stats(&self) -> JsValue {
        self.loader.cache_stats()
    }

    /// Clear the cache
    #[wasm_bindgen]
    pub fn clear_cache(&mut self) {
        self.loader.cache.clear();
    }
}

/// Initialize the WASM loader with default paths
#[wasm_bindgen]
pub fn init_wasm_loader() -> WasmLoaderJs {
    // Default to loading from the current directory
    WasmLoaderJs::new("./")
}

/// Load ML-KEM WASM modules
#[wasm_bindgen]
pub async fn load_mlkem_modules() -> Result<JsValue, JsValue> {
    let mut loader = ExternalWasmLoader::new("./rustpqc/ml-kem/wasm-kat/".to_string());

    let modules = ["mlkem512.wasm", "mlkem768.wasm", "mlkem1024.wasm"];

    loader.preload_modules(&modules).await?;

    Ok(loader.cache_stats())
}

/// Load ML-DSA WASM modules
#[wasm_bindgen]
pub async fn load_mldsa_modules() -> Result<JsValue, JsValue> {
    let mut loader = ExternalWasmLoader::new("./rustpqc/ml-dsa/wasm-kat/".to_string());

    let modules = [
        "ml-dsa2.wasm",
        "ml-dsa3.wasm",
        "ml-dsa5.wasm",
        "ml-dsa2-aes.wasm",
        "ml-dsa3-aes.wasm",
        "ml-dsa5-aes.wasm",
    ];

    loader.preload_modules(&modules).await?;

    Ok(loader.cache_stats())
}

/// Check if WASM is supported in the current environment
#[wasm_bindgen]
pub fn is_wasm_supported() -> bool {
    web_sys
        ::window()
        .and_then(|w| w.get("WebAssembly"))
        .is_some()
}

/// Get WASM environment information
#[wasm_bindgen]
pub fn get_wasm_info() -> JsValue {
    let info = Object::new();

    // Check for WebAssembly support
    let wasm_supported = is_wasm_supported();
    Reflect::set(&info, &"wasm_supported".into(), &wasm_supported.into()).unwrap();

    // Check for fetch API support
    let fetch_supported = web_sys
        ::window()
        .and_then(|w| w.get("fetch"))
        .is_some();
    Reflect::set(&info, &"fetch_supported".into(), &fetch_supported.into()).unwrap();

    // Check for Promise support
    let promise_supported = web_sys
        ::window()
        .and_then(|w| w.get("Promise"))
        .is_some();
    Reflect::set(&info, &"promise_supported".into(), &promise_supported.into()).unwrap();

    info.into()
}
