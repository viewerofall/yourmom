use std::collections::HashMap;
use std::fs;
use crate::errors::Result;

pub struct ShortcutMap {
    map: HashMap<String, String>,
}

impl ShortcutMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    
    /// Load shortcuts from a .momjoke file
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut map = HashMap::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            
            // Parse: "shorthand -> full_form"
            if let Some((short, full)) = line.split_once("->") {
                let short = short.trim().to_string();
                let full = full.split('#').next().unwrap().trim().to_string();
                map.insert(short, full);
            }
        }
        
        Ok(Self { map })
    }
    
    /// Load default jksmpl.momjoke
    pub fn default_stdlib() -> Self {
        let mut map = HashMap::new();
        
        // Core shortcuts
        map.insert("ymf".to_string(), "yo_mama_so_fat".to_string());
        map.insert("ymd".to_string(), "yo_mama_so_dumb".to_string());
        map.insert("ymsu".to_string(), "yo_mama_so_ugly".to_string());
        map.insert("dym".to_string(), "doing_your_mom".to_string());
        map.insert("ret".to_string(), "did_your_mom".to_string());
        
        Self { map }
    }
    
    /// Expand a shorthand to its full form
    pub fn expand(&self, word: &str) -> String {
        self.map.get(word)
            .cloned()
            .unwrap_or_else(|| word.to_string())
    }
    
    /// Merge another shortcut map into this one
    pub fn merge(&mut self, other: ShortcutMap) {
        self.map.extend(other.map);
    }
}

impl Default for ShortcutMap {
    fn default() -> Self {
        Self::default_stdlib()
    }
}
