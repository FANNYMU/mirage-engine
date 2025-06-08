use egui::{Context, Ui, RichText, Color32, ScrollArea, Sense};
use std::collections::HashMap;
use crate::ui::editor::ui_components::{HierarchyItem};

/// Functions for managing and rendering the hierarchy panel
pub struct HierarchyPanel {
    /// The currently selected entity
    pub selected_entity: Option<u32>,
    /// The hierarchy expanded state
    pub hierarchy_expanded: HashMap<u32, bool>,
    /// Entity names
    pub entity_names: HashMap<u32, String>,
    /// Entity parent map
    pub entity_parent_map: HashMap<u32, u32>,
    /// Drag entity ID
    pub drag_entity_id: Option<u32>,
    /// Show hierarchy search
    pub show_hierarchy_search: bool,
    /// Hierarchy search text
    pub hierarchy_search_text: String,
    /// Show create entity menu
    pub show_create_entity_menu: bool,
    /// Entity types (for icons)
    pub entity_types: HashMap<u32, EntityType>,
}

/// Entity type for hierarchy display
#[derive(Clone, Copy, PartialEq)]
pub enum EntityType {
    /// Empty game object
    GameObject,
    /// Camera entity
    Camera,
    /// Light entity
    Light,
    /// UI element
    UI,
    /// Sprite renderer
    Sprite,
    /// Particle system
    ParticleSystem,
    /// Audio source
    AudioSource,
}

impl HierarchyPanel {
    /// Create a new hierarchy panel
    pub fn new() -> Self {
        let mut entity_names = HashMap::new();
        entity_names.insert(1, "Main Camera".to_string());
        entity_names.insert(2, "Directional Light".to_string());
        entity_names.insert(3, "Player".to_string());
        entity_names.insert(4, "Background".to_string());
        entity_names.insert(5, "UI Canvas".to_string());
        
        // Entity types for icons
        let mut entity_types = HashMap::new();
        entity_types.insert(1, EntityType::Camera);
        entity_types.insert(2, EntityType::Light);
        entity_types.insert(3, EntityType::GameObject);
        entity_types.insert(4, EntityType::Sprite);
        entity_types.insert(5, EntityType::UI);
        
        Self {
            selected_entity: None,
            hierarchy_expanded: HashMap::new(),
            entity_names,
            entity_parent_map: HashMap::new(),
            drag_entity_id: None,
            show_hierarchy_search: false,
            hierarchy_search_text: String::new(),
            show_create_entity_menu: false,
            entity_types,
        }
    }
    
    /// Render the hierarchy panel
    pub fn render(&mut self, ui: &mut Ui, log_info: &mut dyn FnMut(&str)) {
        // Unity-like header with dark background
        ui.horizontal(|ui| {
            ui.heading("Hierarchy");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("＋").clicked() {
                    self.show_create_entity_menu = true;
                    log_info("Create entity menu opened");
                }
                if ui.button("🔍").clicked() {
                    self.show_hierarchy_search = !self.show_hierarchy_search;
                    if !self.show_hierarchy_search {
                        self.hierarchy_search_text.clear();
                    }
                }
            });
        });
        
        ui.separator();
        
        if self.show_hierarchy_search {
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.add(egui::TextEdit::singleline(&mut self.hierarchy_search_text)
                    .hint_text("Search...")
                    .desired_width(f32::INFINITY));
                if ui.button("✖").clicked() {
                    self.hierarchy_search_text.clear();
                }
            });
        }
        
        // Render entity creation menu if open
        if self.show_create_entity_menu {
            self.render_create_menu(ui, log_info);
        }
        
        ScrollArea::vertical().show(ui, |ui| {
            let items = self.build_entity_hierarchy();
            self.render_entity_hierarchy(ui, &items, 0, log_info);
        });
    }
    
    /// Render the create entity menu
    fn render_create_menu(&mut self, ui: &mut Ui, log_info: &mut dyn FnMut(&str)) {
        // Unity-like dropdown menu
        egui::Frame::none()
            .fill(Color32::from_rgb(60, 60, 60))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                
                if ui.selectable_label(false, "Create Empty").clicked() {
                    self.create_entity("New GameObject".to_string(), EntityType::GameObject);
                    self.show_create_entity_menu = false;
                    log_info("Created empty GameObject");
                }
                
                if ui.selectable_label(false, "3D Object").clicked() {
                    log_info("3D Object submenu clicked");
                }
                
                if ui.selectable_label(false, "2D Object").clicked() {
                    log_info("2D Object submenu clicked");
                }
                
                if ui.selectable_label(false, "Light").clicked() {
                    self.create_entity("New Light".to_string(), EntityType::Light);
                    self.show_create_entity_menu = false;
                    log_info("Created Light");
                }
                
                if ui.selectable_label(false, "Camera").clicked() {
                    self.create_entity("New Camera".to_string(), EntityType::Camera);
                    self.show_create_entity_menu = false;
                    log_info("Created Camera");
                }
                
                if ui.selectable_label(false, "UI").clicked() {
                    self.create_entity("UI Element".to_string(), EntityType::UI);
                    self.show_create_entity_menu = false;
                    log_info("Created UI Element");
                }
                
                if ui.selectable_label(false, "Audio").clicked() {
                    self.create_entity("Audio Source".to_string(), EntityType::AudioSource);
                    self.show_create_entity_menu = false;
                    log_info("Created Audio Source");
                }
                
                if ui.selectable_label(false, "Particle System").clicked() {
                    self.create_entity("Particle System".to_string(), EntityType::ParticleSystem);
                    self.show_create_entity_menu = false;
                    log_info("Created Particle System");
                }
                
                ui.separator();
                
                if ui.button("Cancel").clicked() {
                    self.show_create_entity_menu = false;
                }
            });
    }
    
    /// Create a new entity
    fn create_entity(&mut self, name: String, entity_type: EntityType) {
        let new_id = self.entity_names.keys().max().map_or(1, |max| max + 1);
        self.entity_names.insert(new_id, name);
        self.entity_types.insert(new_id, entity_type);
        
        // If an entity is selected, make this a child of it
        if let Some(parent_id) = self.selected_entity {
            self.entity_parent_map.insert(new_id, parent_id);
        }
        
        // Select the new entity
        self.selected_entity = Some(new_id);
    }
    
    /// Build the entity hierarchy
    pub fn build_entity_hierarchy(&self) -> Vec<HierarchyItem> {
        let mut root_items = Vec::new();
        let mut used_ids = std::collections::HashSet::new();
        
        // First, collect all entities that have parents
        for (&id, _) in &self.entity_names {
            if let Some(_parent_id) = self.entity_parent_map.get(&id) {
                used_ids.insert(id);
            }
        }
        
        // Then, create hierarchy items for all root entities (those without parents)
        for (&id, name) in &self.entity_names {
            if !used_ids.contains(&id) {
                let mut item = HierarchyItem {
                    id,
                    name: name.clone(),
                    children: Vec::new(),
                };
                
                self.add_children_to_hierarchy(&mut item);
                root_items.push(item);
            }
        }
        
        root_items
    }
    
    /// Add children to a hierarchy item
    pub fn add_children_to_hierarchy(&self, parent: &mut HierarchyItem) {
        for (&id, &parent_id) in &self.entity_parent_map {
            if parent_id == parent.id {
                if let Some(name) = self.entity_names.get(&id) {
                    let mut child = HierarchyItem {
                        id,
                        name: name.clone(),
                        children: Vec::new(),
                    };
                    
                    self.add_children_to_hierarchy(&mut child);
                    parent.children.push(child);
                }
            }
        }
    }
    
    /// Render the entity hierarchy
    pub fn render_entity_hierarchy(&mut self, ui: &mut Ui, items: &[HierarchyItem], depth: usize, log_info: &mut dyn FnMut(&str)) {
        for item in items {
            // Skip items that don't match the search
            if !self.hierarchy_search_text.is_empty() && 
               !item.name.to_lowercase().contains(&self.hierarchy_search_text.to_lowercase()) {
                continue;
            }
            
            // Convert depth to float for UI spacing
            let indent = (depth as f32) * 20.0;
            let has_children = !item.children.is_empty();
            
            // Entity row frame for hover effect and selection
            let is_selected = Some(item.id) == self.selected_entity;
            let row_color = if is_selected {
                Color32::from_rgb(44, 93, 135) // Unity's selection blue
            } else {
                Color32::TRANSPARENT
            };
            
            egui::Frame::none()
                .fill(row_color)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(indent);
                        
                        if has_children {
                            let expanded = self.hierarchy_expanded.entry(item.id).or_insert(false);
                            let text = if *expanded { "▼" } else { "►" };
                            if ui.button(text).clicked() {
                                *expanded = !*expanded;
                            }
                        } else {
                            ui.add_space(15.0);
                        }
                        
                        // Display entity type icon
                        if let Some(entity_type) = self.entity_types.get(&item.id) {
                            let icon = match entity_type {
                                EntityType::GameObject => "⬚",
                                EntityType::Camera => "📷",
                                EntityType::Light => "💡",
                                EntityType::UI => "🖼",
                                EntityType::Sprite => "🎨",
                                EntityType::ParticleSystem => "✨",
                                EntityType::AudioSource => "🔊",
                            };
                            ui.label(icon);
                        }
                        
                        // Entity name with proper styling
                        let mut text = RichText::new(&item.name);
                        if is_selected {
                            text = text.color(Color32::WHITE);
                        }
                        
                        let response = ui.add(egui::Label::new(text).sense(Sense::click()));
                        
                        if response.clicked() {
                            self.selected_entity = Some(item.id);
                            // Log selection
                            log_info(&format!("Selected entity: {}", item.name));
                        }
                        
                        // Handle drag & drop for hierarchy reordering
                        if response.dragged() {
                            self.drag_entity_id = Some(item.id);
                        }
                        
                        if response.drag_released() {
                            self.drag_entity_id = None;
                        }
                        
                        if let Some(drag_id) = self.drag_entity_id {
                            if drag_id != item.id && response.hovered() {
                                // Handle parenting logic here
                                self.entity_parent_map.insert(drag_id, item.id);
                                
                                // We need to rebuild the hierarchy
                                self.drag_entity_id = None;
                                
                                if let Some(dragged_name) = self.entity_names.get(&drag_id) {
                                    log_info(&format!("Moved '{}' to be a child of '{}'", 
                                                   dragged_name, item.name));
                                }
                            }
                        }
                    });
                });
            
            if has_children && *self.hierarchy_expanded.get(&item.id).unwrap_or(&false) {
                self.render_entity_hierarchy(ui, &item.children, depth + 1, log_info);
            }
        }
    }
} 