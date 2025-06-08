use egui::{Context, Ui, Color32, Vec2, Rect, Pos2, Stroke};

/// Game view panel for previewing the game
pub struct GameViewPanel {
    /// Original window size
    pub original_size: Option<[f32; 2]>,
    /// Play mode
    pub play_mode: bool,
    /// Frame counter
    pub frame_counter: u64,
    /// Last frame time
    pub last_frame_time: f64,
}

impl GameViewPanel {
    /// Create a new game view panel
    pub fn new() -> Self {
        Self {
            original_size: None,
            play_mode: false,
            frame_counter: 0,
            last_frame_time: 0.0,
        }
    }
    
    /// Get the original window size
    pub fn get_original_size(&self) -> Option<[f32; 2]> {
        self.original_size
    }
    
    /// Render the game view
    pub fn render(&mut self, ui: &mut Ui, log_info: &mut dyn FnMut(&str)) {
        let available_size = ui.available_size();
        
        // Store the original window size
        if self.original_size.is_none() {
            self.original_size = Some([available_size.x, available_size.y]);
        }
        
        // Draw the game viewport
        let (response, painter) = ui.allocate_painter(
            Vec2::new(available_size.x, available_size.y),
            egui::Sense::click(),
        );
        
        let rect = response.rect;
        
        // Draw mock game screen
        painter.rect_filled(rect, 0.0, Color32::from_rgb(20, 20, 20));
        
        // Draw player if in play mode
        if self.play_mode {
            self.frame_counter += 1;
            
            // Simple moving player
            let player_size = 30.0;
            let t = self.frame_counter as f32 * 0.02;
            let player_x = rect.center().x + (t.sin() * 100.0);
            let player_y = rect.center().y;
            
            painter.rect_filled(
                Rect::from_center_size(
                    Pos2::new(player_x, player_y),
                    Vec2::new(player_size, player_size),
                ),
                0.0,
                Color32::from_rgb(0, 200, 0),
            );
            
            // Draw collectibles
            for i in 0..5 {
                let angle = i as f32 * std::f32::consts::PI * 0.4 + t * 0.5;
                let x = rect.center().x + angle.cos() * 150.0;
                let y = rect.center().y + angle.sin() * 100.0;
                let size = 15.0 + (t + i as f32 * 0.5).sin().abs() * 5.0;
                
                painter.circle_filled(
                    Pos2::new(x, y),
                    size,
                    Color32::from_rgb(255, 200, 0),
                );
            }
            
            // Draw stats
            painter.text(
                Pos2::new(rect.left() + 10.0, rect.top() + 20.0),
                egui::Align2::LEFT_TOP,
                format!("Frame: {}", self.frame_counter),
                egui::FontId::default(),
                Color32::WHITE,
            );
            
            painter.text(
                Pos2::new(rect.left() + 10.0, rect.top() + 40.0),
                egui::Align2::LEFT_TOP,
                if self.frame_counter % 60 < 30 { "PLAYING" } else { "" },
                egui::FontId::proportional(16.0),
                Color32::from_rgb(255, 50, 50),
            );
        } else {
            // Show "Play" message when not in play mode
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Click Play button to start the game",
                egui::FontId::proportional(18.0),
                Color32::WHITE,
            );
        }
        
        // Draw game view controls
        self.draw_controls(ui, rect, log_info);
    }
    
    /// Draw game view controls
    fn draw_controls(&mut self, ui: &mut Ui, rect: Rect, log_info: &mut dyn FnMut(&str)) {
        // Draw resolution picker in the bottom right
        let resolution_text = match rect.width() as u32 {
            w if w > 1280 => "1920×1080",
            w if w > 960 => "1280×720",
            w if w > 640 => "960×540",
            _ => "640×360",
        };
        
        ui.put(
            Rect::from_min_size(
                Pos2::new(rect.right() - 100.0, rect.bottom() - 30.0),
                Vec2::new(90.0, 20.0),
            ),
            egui::SelectableLabel::new(true, resolution_text),
        );
        
        // Draw aspect ratio picker
        ui.put(
            Rect::from_min_size(
                Pos2::new(rect.right() - 100.0, rect.bottom() - 60.0),
                Vec2::new(90.0, 20.0),
            ),
            egui::SelectableLabel::new(true, "16:9"),
        );
        
        // Draw scale picker
        let scale_text = "1×";
        ui.put(
            Rect::from_min_size(
                Pos2::new(rect.right() - 100.0, rect.bottom() - 90.0),
                Vec2::new(90.0, 20.0),
            ),
            egui::SelectableLabel::new(true, scale_text),
        );
        
        // Draw maximize button
        if ui.put(
            Rect::from_min_size(
                Pos2::new(rect.right() - 30.0, rect.top() + 10.0),
                Vec2::new(20.0, 20.0),
            ),
            egui::Button::new("□"),
        ).clicked() {
            log_info("Maximize game view");
        }
    }
} 