#[derive(Debug, Copy, Clone)]
pub enum RuleViolationType {
    SingleLocation,
    SingleObject,
    Cycle,
    IncompleteLayerSpecification,
    LayerDoNotExist,
}
