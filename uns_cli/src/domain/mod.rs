// Domain module exports
pub mod tag;
pub mod tag_repository;
pub mod tag_service;

// Re-export key types
pub use tag::Tag;
pub use tag_repository::TagRepository;
pub use tag_service::TagService;
