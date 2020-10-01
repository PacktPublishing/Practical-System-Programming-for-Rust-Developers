#[cfg(feature = "feature1")]
pub use self::with_feature_config::execute;

#[cfg(not(feature = "feature1"))]
pub use self::without_feature_config::execute;

#[cfg(feature = "feature1")]
mod with_feature_config {
    pub fn execute() {
        println!("Executing feature 1");
    }
}

#[cfg(not(feature = "feature1"))]
mod without_feature_config {
    pub fn execute() {
        println!("Not executing feature 1")
    }
}
