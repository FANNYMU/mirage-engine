use egui::{Context, RichText, Ui, Window, SidePanel, TopBottomPanel, CentralPanel, Style, Visuals, Color32, Stroke, Rect};
use std::collections::HashMap;
use crate::ui::editor::{
    ui_components::{ProjectTab, LogLevel, ConsoleLog, ProjectFile, ProjectFileType, 
    EntityComponent, ComponentType, EntityTransform, HierarchyItem, 
    AudioFile, SceneViewTool},
    hierarchy::HierarchyPanel,
    inspector::InspectorPanel,
    project::ProjectPanel,
    scene_view::SceneViewPanel,
    game_view::GameViewPanel,
    audio_panel::AudioPanel,
    console::ConsolePanel,
};
use crate::audio::AudioCategory;

/// The main editor UI for the engine
pub struct EditorUI {
    /// Hierarchy panel
    pub hierarchy_panel: HierarchyPanel,
    /// Inspector panel
    pub inspector_panel: InspectorPanel,
    /// Project panel
    pub project_panel: ProjectPanel,
    /// Scene view panel
    pub scene_view_panel: SceneViewPanel,
    /// Game view panel
    pub game_view_panel: GameViewPanel,
    /// Audio panel
    pub audio_panel: AudioPanel,
    /// Console panel
    pub console_panel: ConsolePanel,
    /// Current active view (Scene/Game)
    pub active_view: ActiveView,
    /// Show grid in scene view
    pub show_grid: bool,
    /// Editor toolbar state
    pub toolbar: ToolbarState,
    /// Current editor theme
    pub theme: EditorTheme,
}

/// Active view in the editor
#[derive(PartialEq)]
pub enum ActiveView {
    /// Scene view
    Scene,
    /// Game view
    Game,
}

/// Editor toolbar state
pub struct ToolbarState {
    /// Current transform tool
    pub transform_tool: SceneViewTool,
    /// Toggle for play/pause
    pub play_mode: bool,
    /// Toggle for pause
    pub paused: bool,
}

/// Editor theme
pub struct EditorTheme {
    /// Background color
    pub background: Color32,
    /// Panel background color
    pub panel_background: Color32,
    /// Text color
    pub text: Color32,
    /// Selected item color
    pub selected: Color32,
    /// Header color
    pub header: Color32,
    /// Grid color
    pub grid: Color32,
    /// Component header color
    pub component_header: Color32,
}

impl EditorUI {
    /// Create a new editor UI
    pub fn new() -> Self {
        let hierarchy_panel = HierarchyPanel::new();
        let mut inspector_panel = InspectorPanel::new();
        let project_panel = ProjectPanel::new();
        let mut scene_view_panel = SceneViewPanel::new();
        let game_view_panel = GameViewPanel::new();
        let audio_panel = AudioPanel::new();
        let console_panel = ConsolePanel::new();
        
        // Share entity names with scene view
        scene_view_panel.set_entity_names(hierarchy_panel.entity_names.clone());
        
        // Share entity transforms with inspector
        inspector_panel.set_entity_transforms(scene_view_panel.entity_transforms.clone());
        
        Self {
            hierarchy_panel,
            inspector_panel,
            project_panel,
            scene_view_panel,
            game_view_panel,
            audio_panel,
            console_panel,
            active_view: ActiveView::Scene,
            show_grid: true,
            toolbar: ToolbarState {
                transform_tool: SceneViewTool::Select,
                play_mode: false,
                paused: false,
            },
            theme: EditorTheme {
                background: Color32::from_rgb(56, 56, 56),
                panel_background: Color32::from_rgb(42, 42, 42),
                text: Color32::from_rgb(220, 220, 220),
                selected: Color32::from_rgb(44, 93, 135),
                header: Color32::from_rgb(37, 37, 37),
                grid: Color32::from_rgb(60, 60, 60),
                component_header: Color32::from_rgb(65, 65, 65),
            },
        }
    }
    
    /// Get the original window size
    pub fn get_original_size(&self) -> Option<[f32; 2]> {
        self.game_view_panel.get_original_size()
    }
    
    /// Set editor theme
    pub fn set_theme(&self, ctx: &Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals = Visuals::dark();
        
        // Customize Unity-like theme
        style.visuals.widgets.noninteractive.bg_fill = self.theme.panel_background;
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.theme.text);
        style.visuals.widgets.inactive.bg_fill = self.theme.panel_background;
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.theme.text);
        
        // Brighten the panel background slightly for hover
        let hover_bg = Color32::from_rgb(
            (self.theme.panel_background.r() as f32 * 1.05).min(255.0) as u8,
            (self.theme.panel_background.g() as f32 * 1.05).min(255.0) as u8,
            (self.theme.panel_background.b() as f32 * 1.05).min(255.0) as u8,
        );
        style.visuals.widgets.hovered.bg_fill = hover_bg;
        
        style.visuals.widgets.active.bg_fill = self.theme.selected;
        style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        style.visuals.selection.bg_fill = self.theme.selected;
        
        // Brighten the selected color for the stroke
        let selected_bright = Color32::from_rgb(
            (self.theme.selected.r() as f32 * 1.2).min(255.0) as u8,
            (self.theme.selected.g() as f32 * 1.2).min(255.0) as u8,
            (self.theme.selected.b() as f32 * 1.2).min(255.0) as u8,
        );
        style.visuals.selection.stroke = Stroke::new(1.0, selected_bright);
        
        style.visuals.window_fill = self.theme.panel_background;
        style.visuals.panel_fill = self.theme.panel_background;
        
        ctx.set_style(style);
    }
    
    /// Update the editor UI
    pub fn update(&mut self, ctx: &Context, delta_time: f32) {
        // Apply Unity-like theme
        self.set_theme(ctx);
        
        // Update the selected entity in scene view based on hierarchy
        self.scene_view_panel.set_selected_entity(self.hierarchy_panel.selected_entity);
        self.scene_view_panel.scene_view_tool = self.toolbar.transform_tool.clone();
        
        // Collect log messages first
        let mut messages = Vec::new();
        let mut log_info = |message: &str| {
            messages.push(message.to_string());
        };
        
        // Draw toolbar at top
        self.render_toolbar(ctx, &mut log_info);
        
        // Left side with hierarchy panel
        egui::SidePanel::left("hierarchy_panel_container")
            .resizable(true)
            .default_width(300.0)
            .min_width(200.0)
            .frame(egui::Frame::default().fill(self.theme.panel_background))
            .show(ctx, |ui| {
                self.hierarchy_panel.render(ui, &mut log_info);
            });
        
        // Right side with inspector panel
        egui::SidePanel::right("inspector_panel_container")
            .resizable(true)
            .default_width(300.0)
            .min_width(200.0)
            .frame(egui::Frame::default().fill(self.theme.panel_background))
            .show(ctx, |ui| {
                self.inspector_panel.render(ui, self.hierarchy_panel.selected_entity, 
                                          &self.hierarchy_panel.entity_names, &mut log_info);
            });
        
        // Bottom with project panel
        egui::TopBottomPanel::bottom("project_console_panel")
            .resizable(true)
            .default_height(200.0)
            .min_height(100.0)
            .frame(egui::Frame::default().fill(self.theme.panel_background))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Project/Console Tabs
                    if ui.selectable_label(self.project_panel.project_active_tab == ProjectTab::Files, 
                                          "Project").clicked() {
                        self.project_panel.project_active_tab = ProjectTab::Files;
                    }
                    if ui.selectable_label(self.project_panel.project_active_tab == ProjectTab::Console, 
                                          "Console").clicked() {
                        self.project_panel.project_active_tab = ProjectTab::Console;
                    }
                });
                
                ui.separator();
                
                match self.project_panel.project_active_tab {
                    ProjectTab::Files => {
                        self.project_panel.render_project_files(ui, &mut log_info);
                    },
                    ProjectTab::Console => {
                        self.console_panel.render_project_console(ui);
                    },
                    ProjectTab::Audio => {
                        self.audio_panel.render_project_audio(ui, &mut log_info);
                    },
                }
            });
        
        // Central panel with Scene/Game view
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.theme.background))
            .show(ctx, |ui| {
                // Scene/Game view tabs
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.active_view == ActiveView::Scene, "Scene").clicked() {
                        self.active_view = ActiveView::Scene;
                    }
                    if ui.selectable_label(self.active_view == ActiveView::Game, "Game").clicked() {
                        self.active_view = ActiveView::Game;
                    }
                    
                    ui.separator();
                    
                    // Additional view options
                    ui.checkbox(&mut self.show_grid, "Grid");
                });
                
                ui.separator();
                
                // Display active view
                match self.active_view {
                    ActiveView::Scene => {
                        self.scene_view_panel.show_grid = self.show_grid;
                        self.scene_view_panel.render(ui, &mut log_info);
                    },
                    ActiveView::Game => {
                        self.game_view_panel.render(ui, &mut log_info);
                    },
                }
            });
        
        // Process collected log messages
        for message in messages {
            self.console_panel.log_info(&message);
        }
    }
    
    /// Render the Unity-like toolbar
    fn render_toolbar(&mut self, ctx: &Context, log_info: &mut dyn FnMut(&str)) {
        egui::TopBottomPanel::top("toolbar")
            .frame(egui::Frame::default().fill(self.theme.header))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Transform tools
                    ui.selectable_value(&mut self.toolbar.transform_tool, SceneViewTool::Select, "Select");
                    ui.selectable_value(&mut self.toolbar.transform_tool, SceneViewTool::Move, "Move");
                    ui.selectable_value(&mut self.toolbar.transform_tool, SceneViewTool::Rotate, "Rotate");
                    ui.selectable_value(&mut self.toolbar.transform_tool, SceneViewTool::Scale, "Scale");
                    
                    ui.separator();
                    
                    // Play controls
                    let play_text = if self.toolbar.play_mode { "Stop" } else { "Play" };
                    if ui.button(play_text).clicked() {
                        self.toolbar.play_mode = !self.toolbar.play_mode;
                        if self.toolbar.play_mode {
                            log_info("Starting play mode");
                            self.active_view = ActiveView::Game;
                            self.game_view_panel.play_mode = true;
                        } else {
                            log_info("Stopping play mode");
                            self.game_view_panel.play_mode = false;
                        }
                    }
                    
                    if ui.button("Pause").clicked() {
                        self.toolbar.paused = !self.toolbar.paused;
                        log_info(if self.toolbar.paused { "Game paused" } else { "Game resumed" });
                    }
                    
                    ui.separator();
                    
                    // Layers dropdown
                    ui.label("Layers");
                    if ui.button("▼").clicked() {
                        log_info("Layers dropdown clicked");
                    }
                    
                    ui.separator();
                    
                    // Layout dropdown
                    ui.label("Layout");
                    if ui.button("▼").clicked() {
                        log_info("Layout dropdown clicked");
                    }
                });
            });
    }
} 