use glam::{Mat4, Vec2, Vec3};

/// Base trait for all cameras
pub trait Camera {
    /// Get the view matrix
    fn view_matrix(&self) -> Mat4;
    
    /// Get the projection matrix
    fn projection_matrix(&self) -> Mat4;
    
    /// Get the view-projection matrix
    fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}

/// 2D orthographic camera for rendering 2D scenes
pub struct OrthographicCamera {
    position: Vec2,
    zoom: f32,
    rotation: f32, // In radians
    aspect_ratio: f32,
    near: f32,
    far: f32,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
}

impl OrthographicCamera {
    /// Create a new orthographic camera
    pub fn new(width: f32, height: f32, near: f32, far: f32) -> Self {
        let aspect_ratio = width / height;
        let zoom = 1.0;
        
        let half_height = height / 2.0;
        let half_width = width / 2.0;
        
        Self {
            position: Vec2::ZERO,
            zoom,
            rotation: 0.0,
            aspect_ratio,
            near,
            far,
            left: -half_width,
            right: half_width,
            bottom: -half_height,
            top: half_height,
        }
    }
    
    /// Create a new orthographic camera with specific bounds
    pub fn with_bounds(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let width = right - left;
        let height = top - bottom;
        let aspect_ratio = width / height;
        
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            rotation: 0.0,
            aspect_ratio,
            near,
            far,
            left,
            right,
            bottom,
            top,
        }
    }
    
    /// Set the camera position
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
    
    /// Get the camera position
    pub fn position(&self) -> Vec2 {
        self.position
    }
    
    /// Set the camera rotation in radians
    pub fn set_rotation(&mut self, radians: f32) {
        self.rotation = radians;
    }
    
    /// Get the camera rotation in radians
    pub fn rotation(&self) -> f32 {
        self.rotation
    }
    
    /// Set the camera zoom level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1); // Prevent negative or zero zoom
    }
    
    /// Get the camera zoom level
    pub fn zoom(&self) -> f32 {
        self.zoom
    }
    
    /// Resize the camera viewport
    pub fn resize(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
        
        let half_height = height / (2.0 * self.zoom);
        let half_width = width / (2.0 * self.zoom);
        
        self.left = -half_width;
        self.right = half_width;
        self.bottom = -half_height;
        self.top = half_height;
    }
}

impl Camera for OrthographicCamera {
    fn view_matrix(&self) -> Mat4 {
        // Create translation matrix
        let translation = Mat4::from_translation(Vec3::new(-self.position.x, -self.position.y, 0.0));
        
        // Create rotation matrix around Z axis
        let rotation = Mat4::from_rotation_z(-self.rotation);
        
        // Combine transformations: first rotate, then translate
        rotation * translation
    }
    
    fn projection_matrix(&self) -> Mat4 {
        // Create orthographic projection matrix
        Mat4::orthographic_rh(
            self.left / self.zoom,
            self.right / self.zoom,
            self.bottom / self.zoom,
            self.top / self.zoom,
            self.near,
            self.far,
        )
    }
} 