#[derive(Debug, Copy, Clone)]
pub enum RuleViolationType {
    SingleLocation,
    NotAvailable,
    Cycle,
    IncompleteLayerSpecification,
    LayerDoNotExist,
}
