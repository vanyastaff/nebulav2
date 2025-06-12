# nebula-registry

[![Crates.io](https://img.shields.io/crates/v/nebula-registry.svg)](https://crates.io/crates/nebula-registry)
[![Documentation](https://docs.rs/nebula-registry/badge.svg)](https://docs.rs/nebula-registry)
[![License](https://img.shields.io/crates/l/nebula-registry.svg)](LICENSE)

Plugin registry and dynamic loading system for Nebula workflow engine.

## Overview

This crate provides the plugin infrastructure for Nebula:

- **Dynamic Loading** - Load Action implementations from shared libraries
- **Registry Management** - Central registry of all available Actions
- **Plugin Discovery** - Automatic discovery of plugins in directories
- **Type Safety** - Safe loading with version checking
- **Hot Reloading** - Support for plugin updates without restart (optional)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nebula-registry = "0.1"
```

### Creating a Registry

```rust
use nebula_registry::{Registry, PluginLoader};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create registry
    let mut registry = Registry::new();
    
    // Register built-in actions
    registry.register(Box::new(HttpTriggerAction))?;
    registry.register(Box::new(FileReadAction))?;
    
    // Load plugins from directory
    let loader = PluginLoader::new();
    loader.load_plugins_from_dir(Path::new("./plugins"), &mut registry).await?;
    
    // Get action by ID
    if let Some(action) = registry.get("HTTP_TRIGGER") {
        println!("Found action: {}", action.name());
    }
    
    Ok(())
}
```

### Writing a Plugin

Create a new crate with `crate-type = ["cdylib"]`:

```toml
# my-plugin/Cargo.toml
[package]
name = "my-plugin"
version = "0.1.0"

[lib]
crate-type = ["cdylib"]

[dependencies]
nebula-core = "0.1"
nebula-derive = "0.1"
```

Implement your Actions:

```rust
// my-plugin/src/lib.rs
use nebula_core::action::{Action, ActionResult};
use nebula_derive::Parameters;

pub struct MyCustomAction;

impl Action for MyCustomAction {
    type Input = MyInput;
    type Output = MyOutput;
    
    fn id(&self) -> &'static str { "MY_CUSTOM_ACTION" }
    fn name(&self) -> &'static str { "My Custom Action" }
    
    async fn execute(
        &self,
        input: Self::Input,
        context: &mut ProcessContext,
    ) -> Result<ActionResult<Self::Output>, EngineError> {
        // Implementation
        Ok(ActionResult::Value(MyOutput { result: "Done".into() }))
    }
}

// Required plugin entry point
#[no_mangle]
pub extern "C" fn nebula_plugin_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[no_mangle]
pub extern "C" fn register_actions() -> Vec<Box<dyn Action>> {
    vec![
        Box::new(MyCustomAction),
        // Add more actions here
    ]
}
```

Build the plugin:

```bash
cargo build --release
cp target/release/libmy_plugin.so ~/.nebula/plugins/
```

## Plugin Discovery

The registry can automatically discover plugins:

```rust
use nebula_registry::{Registry, DiscoveryConfig};

let config = DiscoveryConfig {
    // Directories to scan
    plugin_dirs: vec![
        PathBuf::from("./plugins"),
        PathBuf::from("/usr/local/lib/nebula/plugins"),
        dirs::data_local_dir().unwrap().join("nebula/plugins"),
    ],
    
    // File patterns to match
    patterns: vec!["*.so", "*.dll", "*.dylib"],
    
    // Enable watching for changes
    watch_for_changes: true,
};

let registry = Registry::with_discovery(config).await?;
```

## Safety and Versioning

The registry performs safety checks:

```rust
// Version compatibility check
if !loader.is_compatible(plugin_version, engine_version) {
    return Err(PluginError::IncompatibleVersion);
}

// Symbol validation
loader.validate_symbols(&library)?;

// Safe loading with error handling
match loader.load_plugin(path) {
    Ok(actions) => registry.register_all(actions),
    Err(PluginError::SymbolNotFound(sym)) => {
        eprintln!("Plugin missing symbol: {}", sym);
    }
    Err(e) => eprintln!("Failed to load plugin: {}", e),
}
```

## Registry API

### Querying Actions

```rust
// Get by ID
let action = registry.get("HTTP_TRIGGER")?;

// Get all actions
for (id, action) in registry.iter() {
    println!("{}: {}", id, action.name());
}

// Filter by trait
let triggers: Vec<_> = registry
    .iter()
    .filter(|(_, action)| action.as_any().is::<dyn TriggerAction>())
    .collect();

// Search by name
let results = registry.search("http");
```

### Metadata

```rust
// Get action metadata
let metadata = registry.metadata("HTTP_TRIGGER")?;
println!("Plugin: {}", metadata.plugin_name);
println!("Version: {}", metadata.plugin_version);
println!("Path: {}", metadata.plugin_path.display());

// List all plugins
for plugin in registry.plugins() {
    println!("{} v{} - {} actions", 
        plugin.name, 
        plugin.version, 
        plugin.action_count
    );
}
```

## Hot Reloading

Enable hot reloading for development:

```rust
use nebula_registry::{Registry, HotReloadConfig};

let config = HotReloadConfig {
    enabled: true,
    poll_interval: Duration::from_secs(5),
    
    // Callback for reload events
    on_reload: |event| {
        match event {
            ReloadEvent::PluginAdded(path) => {
                println!("New plugin: {}", path.display());
            }
            ReloadEvent::PluginUpdated(path) => {
                println!("Plugin updated: {}", path.display());
            }
            ReloadEvent::PluginRemoved(path) => {
                println!("Plugin removed: {}", path.display());
            }
        }
    },
};

let registry = Registry::with_hot_reload(config)?;
```

## Error Handling

```rust
use nebula_registry::{RegistryError, PluginError};

match registry.load_plugin(path).await {
    Ok(count) => println!("Loaded {} actions", count),
    Err(RegistryError::DuplicateAction(id)) => {
        eprintln!("Action {} already registered", id);
    }
    Err(RegistryError::PluginLoad(PluginError::InvalidLibrary(e))) => {
        eprintln!("Invalid library format: {}", e);
    }
    Err(e) => eprintln!("Failed to load plugin: {}", e),
}
```

## Performance Considerations

- Plugins are loaded once and cached
- Action lookups are O(1) using HashMap
- Lazy loading available for large plugin directories
- Parallel plugin loading for faster startup

## Security

- Plugins run in the same process (no sandboxing)
- Validate plugin sources before loading
- Use checksums for plugin verification
- Consider signing plugins for production

## Features

- `tracing` - Enable plugin loading traces

## Platform Support

- **Linux**: `.so` files
- **macOS**: `.dylib` files
- **Windows**: `.dll` files

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.