use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn apply_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).clamp(0.0, self.max);
    }

    pub fn ratio(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_new_at_max() {
        let health = Health::new(100.0);
        assert_eq!(health.current, 100.0);
        assert_eq!(health.max, 100.0);
    }

    #[test]
    fn test_apply_damage_reduces_health() {
        let mut health = Health::new(100.0);
        health.apply_damage(30.0);
        assert_eq!(health.current, 70.0);
    }

    #[test]
    fn test_health_does_not_go_below_zero() {
        let mut health = Health::new(10.0);
        health.apply_damage(50.0);
        assert_eq!(health.current, 0.0);
    }

    #[test]
    fn test_health_does_not_exceed_max() {
        let mut health = Health::new(100.0);
        health.apply_damage(-50.0); // negative damage = healing
        assert_eq!(health.current, 100.0);
    }

    #[test]
    fn test_health_ratio() {
        let mut health = Health::new(100.0);
        assert_eq!(health.ratio(), 1.0);
        health.apply_damage(50.0);
        assert_eq!(health.ratio(), 0.5);
        health.apply_damage(50.0);
        assert_eq!(health.ratio(), 0.0);
    }
}
